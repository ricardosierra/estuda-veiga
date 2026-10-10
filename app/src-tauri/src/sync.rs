#[cfg(test)]
mod tests {
    use super::{
        Reconciliation, SurfaceValue, SyncEngine, json_envelope_projection, mirror_relative_path,
        reconcile, sha256, stable_id,
    };
    use std::fs;

    #[test]
    fn a_single_markdown_edit_propagates_to_sqlite_and_json() {
        let result = reconcile(
            Some("base"),
            &SurfaceValue::Value("base".into()),
            &SurfaceValue::Value("edição Markdown".into()),
            &SurfaceValue::Value("base".into()),
        );

        assert_eq!(result, Reconciliation::Propagate("edição Markdown".into()));
    }

    #[test]
    fn divergent_edits_are_preserved_as_a_conflict() {
        let result = reconcile(
            Some("base"),
            &SurfaceValue::Value("edição SQLite".into()),
            &SurfaceValue::Value("base".into()),
            &SurfaceValue::Value("edição JSON".into()),
        );

        assert!(matches!(result, Reconciliation::Conflict { .. }));
    }

    #[test]
    fn explicit_tombstone_is_distinct_from_a_missing_projection() {
        let result = reconcile(
            Some("base"),
            &SurfaceValue::Value("base".into()),
            &SurfaceValue::Tombstone,
            &SurfaceValue::Value("base".into()),
        );

        assert_eq!(result, Reconciliation::Delete);
    }

    #[test]
    fn markdown_sync_creates_sqlite_record_and_json_mirror() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let markdown = root.path().join("materias/aula.md");
        fs::create_dir_all(markdown.parent().unwrap()).unwrap();
        fs::write(&markdown, "# Aula\n\nConteúdo existente.\n").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();

        let report = engine.reconcile_all().unwrap();

        assert_eq!(report.created, 1);
        let mirror = root
            .path()
            .join(".estuda-uva/mirrors/materias/aula.md.json");
        let envelope: serde_json::Value =
            serde_json::from_slice(&fs::read(mirror).unwrap()).unwrap();
        assert_eq!(envelope["content"], "# Aula\n\nConteúdo existente.\n");
        assert_eq!(
            engine.read_content("materias/aula.md").unwrap().as_deref(),
            Some("# Aula\n\nConteúdo existente.\n")
        );
    }

    #[test]
    fn new_json_capture_is_synced_without_replacing_an_existing_source() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let relative_path = "materias/catalogo/capturas/lyceum/registro.json";
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        let content = r#"{"schemaVersion":1,"pagesRead":1}"#;

        let report = engine.create_content(relative_path, content).unwrap();

        assert_eq!(report.created, 1);
        assert_eq!(
            engine.read_content(relative_path).unwrap().as_deref(),
            Some(content)
        );
        let markdown = root
            .path()
            .join(".estuda-uva/mirrors/materias/catalogo/capturas/lyceum/registro.json.md");
        assert!(fs::read_to_string(markdown).unwrap().contains(content));
        assert!(
            engine
                .create_content(relative_path, r#"{"schemaVersion":1,"pagesRead":2}"#)
                .is_err()
        );
        assert_eq!(
            engine.read_content(relative_path).unwrap().as_deref(),
            Some(content)
        );
    }

    #[test]
    fn a_json_edit_updates_markdown_and_sqlite() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let markdown = root.path().join("materias/aula.md");
        fs::create_dir_all(markdown.parent().unwrap()).unwrap();
        fs::write(&markdown, "# Aula\n\nVersão inicial.\n").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        engine.reconcile_all().unwrap();

        let mirror = root
            .path()
            .join(".estuda-uva/mirrors/materias/aula.md.json");
        let mut envelope: serde_json::Value =
            serde_json::from_slice(&fs::read(&mirror).unwrap()).unwrap();
        envelope["content"] = serde_json::Value::String("# Aula\n\nEditada pelo JSON.\n".into());
        envelope["campoDesconhecido"] = serde_json::json!({"preservar": true});
        fs::write(&mirror, serde_json::to_vec_pretty(&envelope).unwrap()).unwrap();

        let report = engine.reconcile_all().unwrap();

        assert_eq!(report.updated, 1);
        assert_eq!(
            fs::read_to_string(markdown).unwrap(),
            "# Aula\n\nEditada pelo JSON.\n"
        );
        assert_eq!(
            engine.read_content("materias/aula.md").unwrap().as_deref(),
            Some("# Aula\n\nEditada pelo JSON.\n")
        );
        let envelope: serde_json::Value =
            serde_json::from_slice(&fs::read(mirror).unwrap()).unwrap();
        assert_eq!(envelope["campoDesconhecido"]["preservar"], true);
    }

    #[test]
    fn a_sqlite_edit_updates_markdown_and_json() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let markdown = root.path().join("materias/aula.md");
        fs::create_dir_all(markdown.parent().unwrap()).unwrap();
        fs::write(&markdown, "# Aula inicial\n").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        engine.reconcile_all().unwrap();

        engine
            .set_content("materias/aula.md", "# Editada pelo SQLite\n")
            .unwrap();
        let report = engine.reconcile_all().unwrap();

        assert_eq!(report.updated, 1);
        assert_eq!(
            fs::read_to_string(&markdown).unwrap(),
            "# Editada pelo SQLite\n"
        );
        let mirror: serde_json::Value = serde_json::from_slice(
            &fs::read(
                root.path()
                    .join(".estuda-uva/mirrors/materias/aula.md.json"),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(mirror["content"], "# Editada pelo SQLite\n");
    }

    #[test]
    fn interrupted_projection_is_replayed_after_reopening_the_database() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let relative_path = "materias/aula.md";
        let markdown = root.path().join(relative_path);
        fs::create_dir_all(markdown.parent().unwrap()).unwrap();
        let original = "# Versão inicial\n";
        let desired = "# Projeção recuperada\n";
        fs::write(&markdown, original).unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        engine.reconcile_all().unwrap();
        let mirror_relative = mirror_relative_path(relative_path, "md").unwrap();
        let mirror_path = root.path().join(&mirror_relative);
        let mirror_bytes = fs::read(&mirror_path).unwrap();
        let source_bytes = fs::read(&markdown).unwrap();
        let mirror_projection = json_envelope_projection(
            &stable_id(relative_path),
            relative_path,
            "md",
            2,
            desired,
            Some(&mirror_bytes),
        )
        .unwrap();
        {
            let connection = engine.connection.lock().unwrap();
            connection
                .execute(
                    "UPDATE sync_documents SET content=?2,base_content=?2,revision=2 WHERE relative_path=?1",
                    rusqlite::params![relative_path, desired],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO sync_outbox(relative_path,source_expected_hash,mirror_expected_hash,source_projection,mirror_projection,revision,state)
                     VALUES(?1,?2,?3,?4,?5,2,'pending')",
                    rusqlite::params![
                        relative_path,
                        sha256(&source_bytes),
                        sha256(&mirror_bytes),
                        desired,
                        mirror_projection,
                    ],
                )
                .unwrap();
        }
        drop(engine);

        let reopened = SyncEngine::open(&database, root.path()).unwrap();
        reopened.reconcile_all().unwrap();

        assert_eq!(fs::read_to_string(&markdown).unwrap(), desired);
        let mirror: serde_json::Value =
            serde_json::from_slice(&fs::read(mirror_path).unwrap()).unwrap();
        assert_eq!(mirror["content"], desired);
        assert_eq!(
            reopened
                .connection
                .lock()
                .unwrap()
                .query_row("SELECT COUNT(*) FROM sync_outbox", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            0
        );
    }

    #[test]
    fn simultaneous_divergent_edits_leave_both_files_and_record_a_conflict() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let markdown = root.path().join("materias/aula.md");
        fs::create_dir_all(markdown.parent().unwrap()).unwrap();
        fs::write(&markdown, "base\n").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        engine.reconcile_all().unwrap();

        let mirror = root
            .path()
            .join(".estuda-uva/mirrors/materias/aula.md.json");
        let mut envelope: serde_json::Value =
            serde_json::from_slice(&fs::read(&mirror).unwrap()).unwrap();
        envelope["content"] = serde_json::Value::String("edição JSON\n".into());
        fs::write(&mirror, serde_json::to_vec_pretty(&envelope).unwrap()).unwrap();
        fs::write(&markdown, "edição Markdown\n").unwrap();

        let report = engine.reconcile_all().unwrap();

        assert_eq!(report.conflicts, 1);
        assert_eq!(fs::read_to_string(markdown).unwrap(), "edição Markdown\n");
        assert_eq!(engine.read_conflicts().unwrap(), 1);
    }

    #[test]
    fn deleting_a_source_keeps_its_content_and_exposes_a_restoration_conflict() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let source = root.path().join("materias/aula.md");
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        fs::write(&source, "# Aula preservada\n").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        engine.reconcile_all().unwrap();

        fs::remove_file(&source).unwrap();
        let report = engine.reconcile_all().unwrap();
        let conflicts = engine.open_conflicts().unwrap();

        assert_eq!(report.conflicts, 1);
        assert_eq!(conflicts.len(), 1);
        assert!(conflicts[0].tombstone);
        assert_eq!(conflicts[0].sqlite.as_deref(), Some("# Aula preservada\n"));
        assert!(source.exists() == false);
        assert!(
            root.path()
                .join(".estuda-uva/mirrors/materias/aula.md.json")
                .exists()
        );
    }

    #[test]
    fn resolving_a_tombstone_restores_the_source_and_clears_recovery_state() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let source = root.path().join("materias/aula.md");
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        fs::write(&source, "# Aula preservada\n").unwrap();
        let engine = SyncEngine::open(&database, root.path()).unwrap();
        engine.reconcile_all().unwrap();
        fs::remove_file(&source).unwrap();
        engine.reconcile_all().unwrap();

        engine
            .resolve_conflict("materias/aula.md", "# Aula restaurada\n")
            .unwrap();

        assert_eq!(fs::read_to_string(&source).unwrap(), "# Aula restaurada\n");
        assert!(engine.open_conflicts().unwrap().is_empty());
        assert_eq!(engine.read_conflicts().unwrap(), 0);
    }
}
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

const MAX_SYNC_FILE_BYTES: u64 = 64 * 1024 * 1024;
const MIRROR_ROOT: &str = ".estuda-uva/mirrors";

pub struct SyncEngine {
    root: Mutex<PathBuf>,
    connection: Mutex<Connection>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReport {
    pub scanned: usize,
    pub created: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub conflicts: usize,
    pub pending_operations: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncConflict {
    pub id: String,
    pub relative_path: String,
    pub tombstone: bool,
    pub sqlite: Option<String>,
    pub markdown: Option<String>,
    pub json: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
struct StoredDocument {
    id: String,
    content: String,
    base_content: Option<String>,
    revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceValue {
    Value(String),
    Missing,
    Tombstone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reconciliation {
    Unchanged,
    Propagate(String),
    Delete,
    Conflict {
        sqlite: SurfaceValue,
        markdown: SurfaceValue,
        json: SurfaceValue,
    },
}

pub fn reconcile(
    base: Option<&str>,
    sqlite: &SurfaceValue,
    markdown: &SurfaceValue,
    json: &SurfaceValue,
) -> Reconciliation {
    let mut changes: Vec<Option<String>> = Vec::new();
    for surface in [sqlite, markdown, json] {
        match surface {
            SurfaceValue::Missing => {}
            SurfaceValue::Tombstone => changes.push(None),
            SurfaceValue::Value(value) if Some(value.as_str()) == base => {}
            SurfaceValue::Value(value) => changes.push(Some(value.clone())),
        }
    }
    if changes.is_empty() {
        return Reconciliation::Unchanged;
    }
    let first = changes[0].clone();
    if changes.iter().all(|change| *change == first) {
        return match first {
            Some(value) => Reconciliation::Propagate(value),
            None => Reconciliation::Delete,
        };
    }
    Reconciliation::Conflict {
        sqlite: sqlite.clone(),
        markdown: markdown.clone(),
        json: json.clone(),
    }
}

fn initialize_sync_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA foreign_keys=ON;
             CREATE TABLE IF NOT EXISTS sync_documents (
               id TEXT PRIMARY KEY,
               relative_path TEXT NOT NULL UNIQUE,
               format TEXT NOT NULL,
               content TEXT NOT NULL,
               base_content TEXT,
               revision INTEGER NOT NULL,
               content_hash TEXT NOT NULL,
               updated_at TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS sync_conflicts (
               relative_path TEXT PRIMARY KEY,
               base_content TEXT,
               sqlite_value TEXT,
               markdown_value TEXT,
               json_value TEXT,
               created_at TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS sync_tombstones (
               relative_path TEXT PRIMARY KEY,
               format TEXT NOT NULL,
               content TEXT NOT NULL,
               created_at TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS sync_outbox (
               relative_path TEXT PRIMARY KEY,
               source_expected_hash TEXT NOT NULL,
               mirror_expected_hash TEXT NOT NULL,
               source_projection TEXT NOT NULL,
               mirror_projection TEXT NOT NULL,
               revision INTEGER NOT NULL,
               state TEXT NOT NULL DEFAULT 'pending'
             );",
        )
        .map_err(|error| error.to_string())?;
    crate::catalog::academic::initialize_schema(connection)
}

impl SyncEngine {
    pub fn open(database_path: &Path, root: &Path) -> Result<Self, String> {
        let root = root
            .canonicalize()
            .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
        if let Some(parent) = database_path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let connection = Connection::open(database_path).map_err(|error| error.to_string())?;
        initialize_sync_schema(&connection)?;
        Ok(Self {
            root: Mutex::new(root),
            connection: Mutex::new(connection),
        })
    }

    pub fn set_workspace(&self, database_path: &Path, root: &Path) -> Result<(), String> {
        let root = root
            .canonicalize()
            .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
        let connection = Connection::open(database_path).map_err(|error| error.to_string())?;
        initialize_sync_schema(&connection)?;
        *self.connection.lock().map_err(|error| error.to_string())? = connection;
        *self.root.lock().map_err(|error| error.to_string())? = root;
        Ok(())
    }

    pub fn reconcile_all(&self) -> Result<SyncReport, String> {
        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        let mut report = SyncReport::default();
        self.replay_outbox(&root, &mut report)?;
        let snapshot = crate::catalog::scanner::scan_library(&root)?;
        let mut seen = std::collections::HashSet::new();

        for entry in snapshot.documents {
            if !matches!(
                entry.format.as_str(),
                "md" | "markdown" | "json" | "txt" | "alg" | "csv"
            ) || is_sensitive_or_excluded(&entry.relative_path)
            {
                continue;
            }
            report.scanned += 1;
            seen.insert(entry.relative_path.clone());
            if entry.size_bytes > MAX_SYNC_FILE_BYTES {
                report
                    .errors
                    .push(format!("{} excede o limite de 64 MB.", entry.relative_path));
                continue;
            }
            if let Err(error) = self.reconcile_path(&root, &entry.relative_path, &mut report) {
                report
                    .errors
                    .push(format!("{}: {error}", entry.relative_path));
            }
        }
        self.record_missing_sources(&seen, &mut report)?;
        self.refresh_academic_projection(&mut report)?;
        report.pending_operations = self
            .connection
            .lock()
            .map_err(|error| error.to_string())?
            .query_row(
                "SELECT COUNT(*) FROM sync_outbox WHERE state='pending'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|error| error.to_string())?
            .max(0) as usize;
        Ok(report)
    }

    fn refresh_academic_projection(&self, report: &mut SyncReport) -> Result<(), String> {
        let mut connection = self.connection.lock().map_err(|error| error.to_string())?;
        match crate::catalog::academic::load_documents(&connection) {
            Ok(Some(catalog)) => {
                if let Err(error) =
                    crate::catalog::academic::replace_projection(&mut connection, &catalog)
                {
                    report
                        .errors
                        .push(format!("Catálogo acadêmico preservado: {error}"));
                }
            }
            Ok(None) => {}
            Err(error) => report
                .errors
                .push(format!("Catálogo acadêmico preservado: {error}")),
        }
        Ok(())
    }

    pub fn read_academic_catalog(
        &self,
    ) -> Result<crate::catalog::academic::AcademicCatalog, String> {
        let connection = self.connection.lock().map_err(|error| error.to_string())?;
        crate::catalog::academic::read_projection(&connection)
    }

    pub fn read_content(&self, relative_path: &str) -> Result<Option<String>, String> {
        validate_relative_path(relative_path)?;
        self.connection
            .lock()
            .map_err(|error| error.to_string())?
            .query_row(
                "SELECT content FROM sync_documents WHERE relative_path=?1",
                [relative_path],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| error.to_string())
    }

    pub fn create_content(&self, relative_path: &str, content: &str) -> Result<SyncReport, String> {
        let relative = validate_relative_path(relative_path)?;
        if is_sensitive_or_excluded(relative_path) {
            return Err("O caminho pertence a dados locais privados ou excluídos.".into());
        }
        if content.len() as u64 > MAX_SYNC_FILE_BYTES {
            return Err("O documento excede o limite de 64 MB.".into());
        }
        let format = relative
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !matches!(
            format.as_str(),
            "md" | "markdown" | "json" | "txt" | "alg" | "csv"
        ) {
            return Err("O tipo de documento não é aceito pela sincronização local.".into());
        }
        if format == "json" && serde_json::from_str::<JsonValue>(content).is_err() {
            return Err("JSON inválido; nenhum arquivo foi criado.".into());
        }

        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        write_new_source(&root, relative_path, content)?;
        let mut report = SyncReport::default();
        self.reconcile_path(&root, relative_path, &mut report)?;
        if report
            .errors
            .iter()
            .any(|error| error.starts_with(relative_path))
            || self.read_content(relative_path)?.is_none()
        {
            return Err(format!(
                "O documento foi preservado em {relative_path}, mas sua sincronização ficou pendente."
            ));
        }
        Ok(report)
    }

    pub fn set_content(&self, relative_path: &str, content: &str) -> Result<(), String> {
        validate_relative_path(relative_path)?;
        if content.len() as u64 > MAX_SYNC_FILE_BYTES {
            return Err("O documento excede o limite de 64 MB.".into());
        }
        let _current = self
            .load_document(relative_path)?
            .ok_or_else(|| "Documento não encontrado na sincronização local.".to_owned())?;
        let format = Path::new(relative_path)
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default();
        if format.eq_ignore_ascii_case("json")
            && serde_json::from_str::<JsonValue>(content).is_err()
        {
            return Err("JSON inválido; nenhuma versão foi alterada.".into());
        }
        let changed = self
            .connection
            .lock()
            .map_err(|error| error.to_string())?
            .execute(
                "UPDATE sync_documents SET content=?2, updated_at=?3 WHERE relative_path=?1",
                params![relative_path, content, timestamp()],
            )
            .map_err(|error| error.to_string())?;
        if changed == 0 {
            return Err("Documento não encontrado na sincronização local.".into());
        }
        Ok(())
    }

    pub fn open_conflicts(&self) -> Result<Vec<SyncConflict>, String> {
        let connection = self.connection.lock().map_err(|error| error.to_string())?;
        let mut statement = connection
            .prepare(
                "SELECT c.relative_path,EXISTS(SELECT 1 FROM sync_tombstones t WHERE t.relative_path=c.relative_path),
                        c.sqlite_value,c.markdown_value,c.json_value,c.created_at
             FROM sync_conflicts c ORDER BY c.created_at DESC",
            )
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map([], |row| {
                let relative_path: String = row.get(0)?;
                Ok(SyncConflict {
                    id: relative_path.clone(),
                    relative_path,
                    tombstone: row.get(1)?,
                    sqlite: row.get(2)?,
                    markdown: row.get(3)?,
                    json: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
            .map_err(|error| error.to_string())?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())
    }

    pub fn resolve_conflict(&self, relative_path: &str, content: &str) -> Result<(), String> {
        validate_relative_path(relative_path)?;
        if content.len() as u64 > MAX_SYNC_FILE_BYTES {
            return Err("O conteúdo excede o limite de 64 MB.".into());
        }
        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        let source_bytes = read_optional_safe(&root, relative_path)?;
        let source_missing = source_bytes.is_none();
        if source_missing && !self.has_tombstone(relative_path)? {
            return Err(
                "O arquivo não existe e não há uma exclusão pendente para restaurar.".into(),
            );
        }
        let source_bytes = source_bytes.unwrap_or_default();
        let format = Path::new(relative_path)
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let content = if format == "json" && serde_json::from_str::<JsonValue>(content).is_err() {
            decode_json_markdown(content.as_bytes())?
        } else {
            content.to_owned()
        };
        if format == "json" && serde_json::from_str::<JsonValue>(&content).is_err() {
            return Err("JSON inválido; nenhuma versão foi alterada.".into());
        }
        let mirror_relative = mirror_relative_path(relative_path, &format)?;
        let mirror_bytes = read_optional_safe(&root, &mirror_relative)?;
        let existing = self.load_document(relative_path)?;
        self.apply_content(
            &root,
            relative_path,
            &format,
            &content,
            existing.as_ref(),
            &source_bytes,
            source_missing,
            mirror_bytes.as_deref(),
            &mirror_relative,
        )?;
        self.connection
            .lock()
            .map_err(|error| error.to_string())?
            .execute(
                "DELETE FROM sync_conflicts WHERE relative_path=?1",
                [relative_path],
            )
            .map_err(|error| error.to_string())?;
        self.connection
            .lock()
            .map_err(|error| error.to_string())?
            .execute(
                "DELETE FROM sync_tombstones WHERE relative_path=?1",
                [relative_path],
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    fn has_tombstone(&self, relative_path: &str) -> Result<bool, String> {
        self.connection
            .lock()
            .map_err(|error| error.to_string())?
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sync_tombstones WHERE relative_path=?1)",
                [relative_path],
                |row| row.get(0),
            )
            .map_err(|error| error.to_string())
    }

    pub fn read_conflicts(&self) -> Result<usize, String> {
        self.connection
            .lock()
            .map_err(|error| error.to_string())?
            .query_row("SELECT COUNT(*) FROM sync_conflicts", [], |row| {
                row.get::<_, i64>(0)
            })
            .map(|count| count as usize)
            .map_err(|error| error.to_string())
    }

    fn reconcile_path(
        &self,
        root: &Path,
        relative_path: &str,
        report: &mut SyncReport,
    ) -> Result<(), String> {
        if self.has_tombstone(relative_path)? {
            let mut connection = self.connection.lock().map_err(|error| error.to_string())?;
            let transaction = connection
                .transaction()
                .map_err(|error| error.to_string())?;
            transaction
                .execute(
                    "DELETE FROM sync_tombstones WHERE relative_path=?1",
                    [relative_path],
                )
                .map_err(|error| error.to_string())?;
            transaction
                .execute(
                    "DELETE FROM sync_conflicts WHERE relative_path=?1",
                    [relative_path],
                )
                .map_err(|error| error.to_string())?;
            transaction.commit().map_err(|error| error.to_string())?;
        }
        let source_path = safe_existing_path(root, relative_path)?;
        let source_bytes = fs::read(&source_path).map_err(|error| error.to_string())?;
        if source_bytes.len() as u64 > MAX_SYNC_FILE_BYTES {
            return Err("o arquivo excede o limite de 64 MB".into());
        }
        let source_text = String::from_utf8(source_bytes.clone())
            .map_err(|_| "o arquivo não está em UTF-8".to_owned())?;
        let format = source_path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if format == "json" && serde_json::from_str::<JsonValue>(&source_text).is_err() {
            return Err("JSON inválido; os últimos dados válidos foram preservados".into());
        }

        let mirror_relative = mirror_relative_path(relative_path, &format)?;
        let mirror_bytes = read_optional_safe(root, &mirror_relative)?;
        let stored = self.load_document(relative_path)?;
        let markdown_value = if format == "json" {
            match mirror_bytes.as_deref() {
                Some(bytes) => SurfaceValue::Value(decode_json_markdown(bytes)?),
                None => SurfaceValue::Missing,
            }
        } else {
            SurfaceValue::Value(source_text.clone())
        };
        let json_value = if format == "json" {
            SurfaceValue::Value(source_text.clone())
        } else {
            match mirror_bytes.as_deref() {
                Some(bytes) => SurfaceValue::Value(parse_json_envelope(
                    bytes,
                    stored.as_ref().map(|record| record.id.as_str()),
                    relative_path,
                )?),
                None => SurfaceValue::Missing,
            }
        };
        let sqlite_value = stored
            .as_ref()
            .map(|record| SurfaceValue::Value(record.content.clone()))
            .unwrap_or(SurfaceValue::Missing);
        let base = stored
            .as_ref()
            .and_then(|record| record.base_content.as_deref());

        match reconcile(base, &sqlite_value, &markdown_value, &json_value) {
            Reconciliation::Unchanged => {
                if stored.is_some() {
                    report.unchanged += 1;
                }
            }
            Reconciliation::Propagate(content) => {
                self.apply_content(
                    root,
                    relative_path,
                    &format,
                    &content,
                    stored.as_ref(),
                    &source_bytes,
                    false,
                    mirror_bytes.as_deref(),
                    &mirror_relative,
                )?;
                if stored.is_some() {
                    report.updated += 1;
                } else {
                    report.created += 1;
                }
            }
            Reconciliation::Delete => {
                report.errors.push(format!(
                    "{} tem uma solicitação de exclusão explícita pendente.",
                    relative_path
                ));
            }
            Reconciliation::Conflict {
                sqlite,
                markdown,
                json,
            } => {
                self.save_conflict(relative_path, base, &sqlite, &markdown, &json)?;
                report.conflicts += 1;
            }
        }
        Ok(())
    }

    fn apply_content(
        &self,
        root: &Path,
        relative_path: &str,
        format: &str,
        content: &str,
        existing: Option<&StoredDocument>,
        source_bytes: &[u8],
        source_missing: bool,
        mirror_bytes: Option<&[u8]>,
        mirror_relative: &str,
    ) -> Result<(), String> {
        if content.len() as u64 > MAX_SYNC_FILE_BYTES {
            return Err("o conteúdo sincronizado excede 64 MB".into());
        }
        if format == "json" && serde_json::from_str::<JsonValue>(content).is_err() {
            return Err(
                "a edição não é um JSON válido; os últimos dados válidos foram preservados".into(),
            );
        }
        let id = existing
            .map(|record| record.id.clone())
            .unwrap_or_else(|| stable_id(relative_path));
        let revision = existing.map_or(1, |record| record.revision + 1);
        let source_projection = content.to_owned();
        let mirror_projection = if format == "json" {
            json_markdown_projection(relative_path, content)
        } else {
            json_envelope_projection(&id, relative_path, format, revision, content, mirror_bytes)?
        };
        let source_hash = if source_missing {
            String::new()
        } else {
            sha256(source_bytes)
        };
        let mirror_hash = mirror_bytes.map(sha256).unwrap_or_default();
        let mut connection = self.connection.lock().map_err(|error| error.to_string())?;
        let transaction = connection
            .transaction()
            .map_err(|error| error.to_string())?;
        transaction.execute(
            "INSERT INTO sync_documents(id,relative_path,format,content,base_content,revision,content_hash,updated_at)
             VALUES(?1,?2,?3,?4,?4,?5,?6,?7)
             ON CONFLICT(relative_path) DO UPDATE SET id=excluded.id,format=excluded.format,content=excluded.content,
               base_content=excluded.base_content,revision=excluded.revision,content_hash=excluded.content_hash,updated_at=excluded.updated_at",
            params![id, relative_path, format, content, revision as i64, sha256(content.as_bytes()), timestamp()],
        ).map_err(|error| error.to_string())?;
        transaction.execute(
            "INSERT INTO sync_outbox(relative_path,source_expected_hash,mirror_expected_hash,source_projection,mirror_projection,revision,state)
             VALUES(?1,?2,?3,?4,?5,?6,'pending')
             ON CONFLICT(relative_path) DO UPDATE SET source_expected_hash=excluded.source_expected_hash,
               mirror_expected_hash=excluded.mirror_expected_hash,source_projection=excluded.source_projection,
               mirror_projection=excluded.mirror_projection,revision=excluded.revision,state='pending'",
            params![relative_path, source_hash, mirror_hash, source_projection, mirror_projection, revision as i64],
        ).map_err(|error| error.to_string())?;
        transaction.commit().map_err(|error| error.to_string())?;
        drop(connection);
        self.replay_one_outbox(root, relative_path, mirror_relative)
    }

    fn replay_outbox(&self, root: &Path, report: &mut SyncReport) -> Result<(), String> {
        let rows = {
            let connection = self.connection.lock().map_err(|error| error.to_string())?;
            let mut statement = connection.prepare(
                "SELECT relative_path FROM sync_outbox WHERE state='pending' ORDER BY relative_path",
            ).map_err(|error| error.to_string())?;
            statement
                .query_map([], |row| row.get::<_, String>(0))
                .map_err(|error| error.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?
        };
        for relative_path in rows {
            let format = Path::new(&relative_path)
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            let mirror_relative = mirror_relative_path(&relative_path, &format)?;
            if let Err(error) = self.replay_one_outbox(root, &relative_path, &mirror_relative) {
                report.errors.push(format!("{relative_path}: {error}"));
            }
        }
        Ok(())
    }

    fn replay_one_outbox(
        &self,
        root: &Path,
        relative_path: &str,
        mirror_relative: &str,
    ) -> Result<(), String> {
        let item = self.connection.lock().map_err(|error| error.to_string())?
            .query_row(
                "SELECT source_expected_hash,mirror_expected_hash,source_projection,mirror_projection
                 FROM sync_outbox WHERE relative_path=?1 AND state='pending'",
                [relative_path],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?)),
            ).optional().map_err(|error| error.to_string())?;
        let Some((source_expected, mirror_expected, source_projection, mirror_projection)) = item
        else {
            return Ok(());
        };
        let source_current = read_optional_safe(root, relative_path)?;
        let mirror_current = read_optional_safe(root, mirror_relative)?;
        let source_target_hash = sha256(source_projection.as_bytes());
        let mirror_target_hash = sha256(mirror_projection.as_bytes());
        if !hash_matches_or_absent(&source_current, &source_expected, &source_target_hash)
            || !hash_matches_or_absent(&mirror_current, &mirror_expected, &mirror_target_hash)
        {
            let current_source = source_current
                .as_deref()
                .and_then(|bytes| String::from_utf8(bytes.to_vec()).ok());
            let current_mirror = mirror_current
                .as_deref()
                .and_then(|bytes| String::from_utf8(bytes.to_vec()).ok());
            let record = self.load_document(relative_path)?;
            let base = record
                .as_ref()
                .and_then(|record| record.base_content.as_deref());
            self.save_conflict(
                relative_path,
                base,
                &record
                    .as_ref()
                    .map(|record| SurfaceValue::Value(record.content.clone()))
                    .unwrap_or(SurfaceValue::Missing),
                &current_source
                    .map(SurfaceValue::Value)
                    .unwrap_or(SurfaceValue::Missing),
                &current_mirror
                    .map(SurfaceValue::Value)
                    .unwrap_or(SurfaceValue::Missing),
            )?;
            self.connection
                .lock()
                .map_err(|error| error.to_string())?
                .execute(
                    "UPDATE sync_outbox SET state='conflict' WHERE relative_path=?1",
                    [relative_path],
                )
                .map_err(|error| error.to_string())?;
            return Err("o arquivo mudou durante a projeção pendente; conflito preservado".into());
        }
        write_projection(root, relative_path, &source_expected, &source_projection)?;
        write_projection(root, mirror_relative, &mirror_expected, &mirror_projection)?;
        self.connection
            .lock()
            .map_err(|error| error.to_string())?
            .execute(
                "DELETE FROM sync_outbox WHERE relative_path=?1",
                [relative_path],
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    fn load_document(&self, relative_path: &str) -> Result<Option<StoredDocument>, String> {
        self.connection.lock().map_err(|error| error.to_string())?
            .query_row(
                "SELECT id,content,base_content,revision FROM sync_documents WHERE relative_path=?1",
                [relative_path],
                |row| Ok(StoredDocument {
                    id: row.get(0)?, content: row.get(1)?, base_content: row.get(2)?, revision: row.get::<_, i64>(3)? as u64,
                }),
            )
            .optional()
            .map_err(|error| error.to_string())
    }

    fn save_conflict(
        &self,
        relative_path: &str,
        base: Option<&str>,
        sqlite: &SurfaceValue,
        markdown: &SurfaceValue,
        json: &SurfaceValue,
    ) -> Result<(), String> {
        self.connection.lock().map_err(|error| error.to_string())?.execute(
            "INSERT INTO sync_conflicts(relative_path,base_content,sqlite_value,markdown_value,json_value,created_at)
             VALUES(?1,?2,?3,?4,?5,?6)
             ON CONFLICT(relative_path) DO UPDATE SET base_content=excluded.base_content,sqlite_value=excluded.sqlite_value,
               markdown_value=excluded.markdown_value,json_value=excluded.json_value,created_at=excluded.created_at",
            params![relative_path, base, surface_text(sqlite), surface_text(markdown), surface_text(json), timestamp()],
        ).map_err(|error| error.to_string())?;
        Ok(())
    }

    fn record_missing_sources(
        &self,
        seen: &std::collections::HashSet<String>,
        report: &mut SyncReport,
    ) -> Result<(), String> {
        let paths = {
            let connection = self.connection.lock().map_err(|error| error.to_string())?;
            let mut statement = connection
                .prepare("SELECT relative_path,format,content FROM sync_documents ORDER BY relative_path")
                .map_err(|error| error.to_string())?;
            statement
                .query_map([], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                })
                .map_err(|error| error.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?
        };
        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        for (path, format, content) in paths {
            if seen.contains(&path) {
                continue;
            }
            let was_already_marked = self.has_tombstone(&path)?;
            self.connection
                .lock()
                .map_err(|error| error.to_string())?
                .execute(
                    "INSERT OR IGNORE INTO sync_tombstones(relative_path,format,content,created_at)
                     VALUES(?1,?2,?3,?4)",
                    params![path, format, content, timestamp()],
                )
                .map_err(|error| error.to_string())?;
            if was_already_marked {
                continue;
            }
            let format = format.to_ascii_lowercase();
            let mirror_relative = mirror_relative_path(&path, &format)?;
            let mirror_bytes = read_optional_safe(&root, &mirror_relative)?;
            let sqlite = SurfaceValue::Value(content.clone());
            let (markdown, json) = if format == "json" {
                let markdown = mirror_bytes
                    .as_deref()
                    .and_then(|bytes| std::str::from_utf8(bytes).ok().map(str::to_owned));
                (
                    markdown
                        .map(SurfaceValue::Value)
                        .unwrap_or(SurfaceValue::Missing),
                    SurfaceValue::Missing,
                )
            } else {
                let json = mirror_bytes.as_deref().and_then(|bytes| {
                    let id = stable_id(&path);
                    parse_json_envelope(bytes, Some(&id), &path).ok()
                });
                (
                    SurfaceValue::Missing,
                    json.map(SurfaceValue::Value)
                        .unwrap_or(SurfaceValue::Missing),
                )
            };
            self.save_conflict(&path, Some(&content), &sqlite, &markdown, &json)?;
            report.conflicts += 1;
        }
        Ok(())
    }
}

fn validate_relative_path(relative_path: &str) -> Result<&Path, String> {
    let path = Path::new(relative_path);
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("Caminho relativo inválido.".into());
    }
    Ok(path)
}

fn is_sensitive_or_excluded(relative_path: &str) -> bool {
    let lower = relative_path.to_ascii_lowercase();
    lower.starts_with("originais/")
        || lower.split('/').any(|part| {
            part.contains("senha")
                || part.contains("password")
                || part.contains("credential")
                || part.contains("credencial")
                || part.contains("cookie")
                || part.contains("token")
                || part.contains("secret")
        })
}

fn safe_existing_path(root: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let relative = validate_relative_path(relative_path)?;
    let mut current = root.to_path_buf();
    for component in relative.components() {
        let Component::Normal(part) = component else {
            unreachable!()
        };
        current.push(part);
        let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() {
            return Err("o caminho atravessa um link simbólico".into());
        }
    }
    let canonical = current.canonicalize().map_err(|error| error.to_string())?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err("o arquivo está fora da pasta do acervo".into());
    }
    Ok(canonical)
}

fn read_optional_safe(root: &Path, relative_path: &str) -> Result<Option<Vec<u8>>, String> {
    let relative = validate_relative_path(relative_path)?;
    let mut current = root.to_path_buf();
    for component in relative.components() {
        let Component::Normal(part) = component else {
            unreachable!()
        };
        current.push(part);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err("o caminho atravessa um link simbólico".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.to_string()),
        }
    }
    let canonical = current.canonicalize().map_err(|error| error.to_string())?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err("o arquivo está fora da pasta do acervo".into());
    }
    let metadata = fs::metadata(&canonical).map_err(|error| error.to_string())?;
    if metadata.len() > MAX_SYNC_FILE_BYTES {
        return Err("o arquivo excede o limite de 64 MB".into());
    }
    fs::read(canonical)
        .map(Some)
        .map_err(|error| error.to_string())
}

fn mirror_relative_path(relative_path: &str, format: &str) -> Result<String, String> {
    validate_relative_path(relative_path)?;
    Ok(format!(
        "{MIRROR_ROOT}/{relative_path}{}",
        if format == "json" { ".md" } else { ".json" }
    ))
}

fn stable_id(relative_path: &str) -> String {
    format!("doc-{}", sha256(relative_path.as_bytes()))
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

fn parse_json_envelope(
    bytes: &[u8],
    expected_id: Option<&str>,
    relative_path: &str,
) -> Result<String, String> {
    let value: JsonValue = serde_json::from_slice(bytes)
        .map_err(|_| "o espelho JSON é inválido; nenhuma versão foi sobrescrita".to_owned())?;
    let object = value
        .as_object()
        .ok_or_else(|| "o espelho JSON precisa ser um objeto".to_owned())?;
    if object.get("schemaVersion").and_then(JsonValue::as_u64) != Some(1)
        || object.get("relativePath").and_then(JsonValue::as_str) != Some(relative_path)
    {
        return Err("o envelope JSON não corresponde ao caminho ou à versão esperada".into());
    }
    if let Some(id) = expected_id {
        if object.get("id").and_then(JsonValue::as_str) != Some(id) {
            return Err("o ID do espelho JSON não corresponde ao registro".into());
        }
    }
    object
        .get("content")
        .and_then(JsonValue::as_str)
        .map(str::to_owned)
        .ok_or_else(|| "o espelho JSON não contém o texto sincronizado".into())
}

fn json_envelope_projection(
    id: &str,
    relative_path: &str,
    format: &str,
    revision: u64,
    content: &str,
    previous: Option<&[u8]>,
) -> Result<String, String> {
    let mut object = match previous {
        Some(bytes) => {
            let value: JsonValue = serde_json::from_slice(bytes)
                .map_err(|_| "o espelho JSON é inválido".to_owned())?;
            value
                .as_object()
                .cloned()
                .ok_or_else(|| "o espelho JSON precisa ser um objeto".to_owned())?
        }
        None => Map::new(),
    };
    for (key, value) in [
        ("schemaVersion", JsonValue::from(1)),
        ("id", JsonValue::from(id)),
        ("kind", JsonValue::from(format!("{format}_document"))),
        ("relativePath", JsonValue::from(relative_path)),
        ("revision", JsonValue::from(revision)),
        ("contentHash", JsonValue::from(sha256(content.as_bytes()))),
        ("updatedAt", JsonValue::from(timestamp())),
        ("content", JsonValue::from(content)),
    ] {
        object.insert(key.into(), value);
    }
    let mut json = serde_json::to_string_pretty(&JsonValue::Object(object))
        .map_err(|error| error.to_string())?;
    json.push('\n');
    Ok(json)
}

fn json_markdown_projection(relative_path: &str, json: &str) -> String {
    let final_newline = json.ends_with('\n');
    let longest = json
        .lines()
        .map(|line| line.chars().take_while(|c| *c == '`').count())
        .max()
        .unwrap_or(0);
    let fence = "`".repeat((longest + 1).max(3));
    let title = Path::new(relative_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("JSON acadêmico");
    let separator = if final_newline { "" } else { "\n" };
    format!(
        "# {title}\n\n<!-- estuda-uva-json final-newline={} -->\n\n{fence}json\n{json}{separator}{fence}\n",
        if final_newline { 1 } else { 0 }
    )
}

fn decode_json_markdown(bytes: &[u8]) -> Result<String, String> {
    let markdown = std::str::from_utf8(bytes)
        .map_err(|_| "a projeção Markdown não está em UTF-8".to_owned())?;
    let final_newline = markdown
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("<!-- estuda-uva-json final-newline=")
        })
        .and_then(|line| line.strip_suffix(" -->"))
        .map(|value| value == "1")
        .ok_or_else(|| "a projeção Markdown não contém o marcador de origem JSON".to_owned())?;
    let lines = markdown.lines().collect::<Vec<_>>();
    let (start, fence) = lines
        .iter()
        .enumerate()
        .find_map(|(index, line)| {
            let fence = line.strip_suffix("json")?;
            (fence.len() >= 3 && fence.chars().all(|c| c == '`')).then_some((index + 1, fence))
        })
        .ok_or_else(|| "a projeção Markdown não contém o bloco JSON".to_owned())?;
    let end = lines[start..]
        .iter()
        .position(|line| *line == fence)
        .map(|offset| start + offset)
        .ok_or_else(|| "o bloco JSON da projeção não foi fechado".to_owned())?;
    let mut content = lines[start..end].join("\n");
    if final_newline {
        content.push('\n');
    }
    serde_json::from_str::<JsonValue>(&content)
        .map_err(|_| "o bloco da projeção não é um JSON válido".to_owned())?;
    Ok(content)
}

fn write_new_source(root: &Path, relative_path: &str, content: &str) -> Result<(), String> {
    let relative = validate_relative_path(relative_path)?;
    let root = root
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
    let mut parent = root.clone();
    let parts = relative.components().collect::<Vec<_>>();
    if parts.is_empty() {
        return Err("Caminho relativo vazio.".into());
    }
    for component in &parts[..parts.len() - 1] {
        let Component::Normal(name) = component else {
            return Err("Caminho relativo inválido.".into());
        };
        let next = parent.join(name);
        match fs::symlink_metadata(&next) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err("O caminho atravessa um link ou um item que não é pasta.".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir(&next).map_err(|error| error.to_string())?;
            }
            Err(error) => return Err(error.to_string()),
        }
        parent = next.canonicalize().map_err(|error| error.to_string())?;
        if !parent.starts_with(&root) {
            return Err("O caminho de destino está fora do acervo.".into());
        }
    }
    let Component::Normal(name) = parts[parts.len() - 1] else {
        return Err("Nome de arquivo inválido.".into());
    };
    let destination = parent.join(name);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&destination)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                "O arquivo já existe; nenhum conteúdo foi sobrescrito.".to_owned()
            } else {
                error.to_string()
            }
        })?;
    if let Err(error) = file
        .write_all(content.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = fs::remove_file(destination);
        return Err(error.to_string());
    }
    Ok(())
}

fn surface_text(surface: &SurfaceValue) -> Option<&str> {
    match surface {
        SurfaceValue::Value(value) => Some(value),
        SurfaceValue::Missing | SurfaceValue::Tombstone => None,
    }
}

fn hash_matches_or_absent(current: &Option<Vec<u8>>, expected: &str, desired: &str) -> bool {
    match current {
        Some(bytes) => {
            let hash = sha256(bytes);
            hash == expected || hash == desired
        }
        None => expected.is_empty(),
    }
}

fn write_projection(
    root: &Path,
    relative_path: &str,
    expected_hash: &str,
    content: &str,
) -> Result<(), String> {
    let relative = validate_relative_path(relative_path)?;
    let parts = relative.components().collect::<Vec<_>>();
    if parts.is_empty() {
        return Err("destino vazio".into());
    }
    let mut parent = root.to_path_buf();
    for component in &parts[..parts.len() - 1] {
        let Component::Normal(part) = component else {
            return Err("destino inválido".into());
        };
        parent.push(part);
        match fs::symlink_metadata(&parent) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err("destino atravessa link simbólico".into());
            }
            Ok(metadata) if !metadata.is_dir() => {
                return Err("destino contém componente que não é pasta".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir(&parent).map_err(|error| error.to_string())?
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    let Component::Normal(name) = parts[parts.len() - 1] else {
        return Err("nome de destino inválido".into());
    };
    let destination = parent.join(name);
    let current = match fs::symlink_metadata(&destination) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err("destino não é arquivo regular".into());
        }
        Ok(_) => Some(fs::read(&destination).map_err(|error| error.to_string())?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.to_string()),
    };
    if current.as_deref() == Some(content.as_bytes()) {
        return Ok(());
    }
    if !hash_matches_or_absent(&current, expected_hash, &sha256(content.as_bytes())) {
        return Err("o arquivo mudou depois da leitura; nenhuma versão foi sobrescrita".into());
    }
    let temporary = parent.join(format!(".estuda-sync-{}.tmp", uuid::Uuid::new_v4()));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| error.to_string())?;
    if let Err(error) = file
        .write_all(content.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = fs::remove_file(&temporary);
        return Err(error.to_string());
    }
    drop(file);
    if let Err(error) = fs::rename(&temporary, &destination) {
        let _ = fs::remove_file(&temporary);
        return Err(error.to_string());
    }
    Ok(())
}
