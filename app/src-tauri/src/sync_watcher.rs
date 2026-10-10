use crate::sync::{SyncEngine, SyncReport};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const QUIET_PERIOD: Duration = Duration::from_millis(500);
const SYNCABLE_EXTENSIONS: &[&str] = &["md", "markdown", "txt", "alg", "csv", "json"];

struct WatcherState {
    root: RwLock<PathBuf>,
    watch_set: Mutex<WatchSet>,
    watcher: Mutex<RecommendedWatcher>,
}

pub struct SyncWatcher {
    state: Arc<WatcherState>,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

#[derive(Debug)]
struct WatchSet {
    root: PathBuf,
    directories: Vec<(PathBuf, RecursiveMode)>,
    recursive_roots: HashSet<PathBuf>,
}

impl WatchSet {
    fn from_root(root: &Path) -> Result<Self, String> {
        let root = root
            .canonicalize()
            .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
        if !root.is_dir() {
            return Err("A pasta configurada para o acervo não é uma pasta.".into());
        }
        let mut directories = vec![(root.clone(), RecursiveMode::NonRecursive)];
        let mut recursive_roots = HashSet::new();
        for name in ["materias", "docs"] {
            let path = root.join(name);
            if is_real_directory(&root, &path) {
                recursive_roots.insert(path.clone());
                directories.push((path, RecursiveMode::Recursive));
            }
        }
        Ok(Self {
            root,
            directories,
            recursive_roots,
        })
    }

    fn covers(&self, path: &Path) -> bool {
        path.parent() == Some(self.root.as_path())
            || self
                .recursive_roots
                .iter()
                .any(|directory| path.starts_with(directory))
    }

    fn add_new_academic_roots(&mut self, watcher: &mut RecommendedWatcher) {
        for name in ["materias", "docs"] {
            let path = self.root.join(name);
            if !is_real_directory(&self.root, &path) || self.recursive_roots.contains(&path) {
                continue;
            }
            if watcher.watch(&path, RecursiveMode::Recursive).is_ok() {
                self.recursive_roots.insert(path.clone());
                self.directories.push((path, RecursiveMode::Recursive));
            }
        }
    }
}

fn is_real_directory(root: &Path, path: &Path) -> bool {
    let Ok(metadata) = std::fs::symlink_metadata(path) else {
        return false;
    };
    if !metadata.file_type().is_dir() {
        return false;
    }
    path.canonicalize()
        .is_ok_and(|canonical| canonical.starts_with(root))
}

impl SyncWatcher {
    pub fn start(
        root: PathBuf,
        engine: Arc<SyncEngine>,
        on_sync: impl Fn(SyncReport) + Send + Sync + 'static,
    ) -> Result<Self, String> {
        let watch_set = WatchSet::from_root(&root)?;
        let root = watch_set.root.clone();
        let (sender, receiver) = mpsc::channel::<notify::Result<Event>>();
        let mut watcher = RecommendedWatcher::new(
            move |event| {
                let _ = sender.send(event);
            },
            Config::default(),
        )
        .map_err(|error| format!("Não foi possível monitorar a pasta do acervo: {error}"))?;
        for (directory, mode) in &watch_set.directories {
            watcher.watch(directory, *mode).map_err(|error| {
                format!(
                    "Não foi possível monitorar {}: {error}",
                    directory.display()
                )
            })?;
        }

        let state = Arc::new(WatcherState {
            root: RwLock::new(root),
            watch_set: Mutex::new(watch_set),
            watcher: Mutex::new(watcher),
        });
        let worker_state = Arc::clone(&state);
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let on_sync = Arc::new(on_sync);
        let worker_callback = Arc::clone(&on_sync);
        let worker = thread::Builder::new()
            .name("estuda-uva-sync-watcher".into())
            .spawn(move || {
                while !worker_stop.load(Ordering::Acquire) {
                    let first = match receiver.recv_timeout(Duration::from_millis(200)) {
                        Ok(event) => event,
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    };
                    match first {
                        Ok(event) => {
                            let events = coalesce_event_burst_until_stopped(
                                &receiver,
                                Ok(event),
                                QUIET_PERIOD,
                                &worker_stop,
                            );
                            if worker_stop.load(Ordering::Acquire) {
                                break;
                            }
                            let root = match worker_state.root.read() {
                                Ok(root) => root.clone(),
                                Err(_) => continue,
                            };
                            let relevant = events.iter().any(|event| match event {
                                Ok(event) => {
                                    !matches!(event.kind, EventKind::Access(_))
                                        && event
                                            .paths
                                            .iter()
                                            .any(|path| is_relevant_path(&root, path))
                                }
                                Err(_) => true,
                            });
                            if !relevant {
                                continue;
                            }
                            if let (Ok(mut watcher), Ok(mut watch_set)) =
                                (worker_state.watcher.lock(), worker_state.watch_set.lock())
                            {
                                watch_set.add_new_academic_roots(&mut watcher);
                            }
                            if let Ok(report) = engine.reconcile_all() {
                                worker_callback(report);
                            }
                        }
                        Err(_) => {
                            if let Ok(report) = engine.reconcile_all() {
                                worker_callback(report);
                            }
                        }
                    }
                }
            })
            .map_err(|error| format!("Não foi possível iniciar o monitor do acervo: {error}"))?;

        Ok(Self {
            state,
            stop,
            worker: Some(worker),
        })
    }

    pub fn set_root(&self, root: &Path) -> Result<(), String> {
        let next = WatchSet::from_root(root)?;
        let same_root = self
            .state
            .root
            .read()
            .map_err(|error| error.to_string())?
            .as_path()
            == next.root.as_path();
        if same_root {
            return Ok(());
        }
        let mut watcher = self
            .state
            .watcher
            .lock()
            .map_err(|error| error.to_string())?;
        let mut current = self
            .state
            .watch_set
            .lock()
            .map_err(|error| error.to_string())?;
        for (directory, mode) in &next.directories {
            watcher.watch(directory, *mode).map_err(|error| {
                format!(
                    "Não foi possível monitorar {}: {error}",
                    directory.display()
                )
            })?;
        }
        for (directory, _) in &current.directories {
            if !next
                .directories
                .iter()
                .any(|(next_path, _)| next_path == directory)
            {
                let _ = watcher.unwatch(directory);
            }
        }
        *self.state.root.write().map_err(|error| error.to_string())? = next.root.clone();
        *current = next;
        Ok(())
    }
}

impl Drop for SyncWatcher {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn is_relevant_path(root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return false;
    };
    let parts = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>();
    if parts.is_empty() {
        return false;
    }
    if parts.len() == 1 {
        let name = parts[0].to_ascii_lowercase();
        if matches!(name.as_str(), "materias" | "docs") {
            return true;
        }
        if name == ".estuda-uva.sqlite3"
            || ["-wal", "-shm", "-journal"]
                .iter()
                .any(|suffix| name == format!(".estuda-uva.sqlite3{suffix}"))
        {
            return true;
        }
    }
    if parts.iter().any(|part| {
        part.starts_with('.')
            || [
                "senha",
                "password",
                "credential",
                "credencial",
                "cookie",
                "token",
                "secret",
            ]
            .iter()
            .any(|marker| part.to_ascii_lowercase().contains(marker))
    }) {
        return false;
    }
    let first = parts[0].to_ascii_lowercase();
    if first == "originais"
        || first == "app"
        || first == "node_modules"
        || first == "target"
        || first == "dist"
        || first == ".git"
        || (first == "docs"
            && parts
                .get(1)
                .is_some_and(|part| part.eq_ignore_ascii_case("superpowers")))
    {
        return false;
    }
    let root_document = parts.len() == 1;
    let academic_tree = first == "materias" || first == "docs";
    (root_document || academic_tree)
        && path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                SYNCABLE_EXTENSIONS
                    .iter()
                    .any(|supported| extension.eq_ignore_ascii_case(supported))
            })
}

fn coalesce_event_burst<T>(
    receiver: &mpsc::Receiver<T>,
    first: T,
    quiet_period: Duration,
) -> Vec<T> {
    coalesce_event_burst_until_stopped(receiver, first, quiet_period, &AtomicBool::new(false))
}

fn coalesce_event_burst_until_stopped<T>(
    receiver: &mpsc::Receiver<T>,
    first: T,
    quiet_period: Duration,
    stop: &AtomicBool,
) -> Vec<T> {
    let mut events = vec![first];
    while !stop.load(Ordering::Acquire) {
        match receiver.recv_timeout(quiet_period) {
            Ok(event) => events.push(event),
            Err(mpsc::RecvTimeoutError::Timeout | mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    events
}

#[cfg(test)]
mod tests {
    use super::{SyncEngine, SyncWatcher, WatchSet, coalesce_event_burst, is_relevant_path};
    use std::{
        path::PathBuf,
        sync::{Arc, mpsc},
        time::Duration,
    };

    #[test]
    fn watcher_debounces_burst_into_one_batch() {
        let (sender, receiver) = mpsc::channel();
        sender.send(PathBuf::from("materias/a.md")).unwrap();
        sender.send(PathBuf::from("materias/b.md")).unwrap();
        sender.send(PathBuf::from("materias/c.md")).unwrap();

        let first = receiver.recv().unwrap();
        let batch = coalesce_event_burst(&receiver, first, Duration::from_millis(5));

        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn watcher_ignores_generated_mirrors_and_git() {
        let root = std::env::temp_dir().join("faculdade-watch-test");
        assert!(!is_relevant_path(&root, &root.join(".git/index")));
        assert!(!is_relevant_path(
            &root,
            &root.join(".estuda-uva/mirrors/materias/aula.md.json")
        ));
        assert!(is_relevant_path(&root, &root.join("materias/aula.md")));
    }

    #[test]
    fn external_sqlite_write_schedules_reconciliation() {
        let root = std::env::temp_dir().join("faculdade-watch-test");
        assert!(is_relevant_path(
            &root,
            &root.join(".estuda-uva.sqlite3-wal")
        ));
    }

    #[test]
    fn new_in_scope_directory_is_covered_by_recursive_watch() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("materias/nova-materia")).unwrap();
        let canonical_root = root.path().canonicalize().unwrap();

        let set = WatchSet::from_root(&canonical_root).unwrap();

        assert!(set.covers(&canonical_root.join("materias/nova-materia/aula.md")));
    }

    #[cfg(unix)]
    #[test]
    fn watcher_does_not_follow_a_symlinked_academic_directory() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), root.path().join("materias")).unwrap();

        let set = WatchSet::from_root(root.path()).unwrap();

        assert!(!set.recursive_roots.contains(&root.path().join("materias")));
    }

    #[test]
    fn app_sync_write_does_not_echo_from_generated_mirror() {
        let root = std::env::temp_dir().join("faculdade-watch-test");

        assert!(!is_relevant_path(
            &root,
            &root.join(".estuda-uva/mirrors/materias/aula.md.json")
        ));
    }

    #[test]
    fn missed_event_is_recovered_by_full_reconcile() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let source = root.path().join("materias/fora-do-indice.md");
        std::fs::create_dir_all(source.parent().unwrap()).unwrap();
        std::fs::write(&source, "conteúdo salvo antes da inicialização do watcher").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();

        let report = engine.reconcile_all().unwrap();

        assert_eq!(report.created, 1);
        assert!(
            engine
                .read_content("materias/fora-do-indice.md")
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn dropping_the_watcher_stops_and_joins_its_worker() {
        let root = tempfile::tempdir().unwrap();
        let engine = Arc::new(
            SyncEngine::open(&root.path().join(".estuda-uva.sqlite3"), root.path()).unwrap(),
        );
        let watcher = SyncWatcher::start(root.path().to_path_buf(), engine, |_| {}).unwrap();

        drop(watcher);
    }
}
