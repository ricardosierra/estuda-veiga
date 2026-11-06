use super::{
    classification::{PathClassification, classify_path},
    documents::extract_document,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Component, Path, PathBuf},
    sync::Mutex,
    time::UNIX_EPOCH,
};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

const MAX_FILES: usize = 10_000;
const MAX_DEPTH: usize = 16;
const MAX_DOCUMENT_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ASSET_BYTES: u64 = 16 * 1024 * 1024;
const MAX_INDEX_TEXT_BYTES: usize = 128 * 1024 * 1024;
const MAX_SEARCH_RESULTS: usize = 100;

fn canonical_library_root(root: &Path) -> Result<PathBuf, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
    if !root.is_dir() {
        return Err("A pasta configurada para o acervo não é uma pasta.".into());
    }
    Ok(root)
}

fn resolve_path(root: &Path, relative_path: &str) -> Result<(PathBuf, PathBuf), String> {
    let requested = Path::new(relative_path);
    if requested.as_os_str().is_empty()
        || requested.is_absolute()
        || requested
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("Caminho local inválido.".into());
    }
    let canonical_root = canonical_library_root(root)?;
    let path = canonical_root
        .join(requested)
        .canonicalize()
        .map_err(|error| format!("Não foi possível abrir este arquivo local: {error}"))?;
    if !path.starts_with(&canonical_root) || !path.is_file() {
        return Err("O arquivo selecionado está fora da pasta do acervo.".into());
    }
    Ok((canonical_root, path))
}

fn normalize_for_search(value: &str) -> String {
    value
        .nfd()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(char::to_lowercase)
        .collect()
}

fn matches_filter(document: &LocalDocument, filter: &LocalSearchFilter) -> bool {
    fn matches(value: &str, filter: Option<&str>) -> bool {
        filter.is_none_or(|filter| {
            normalize_for_search(value).contains(&normalize_for_search(filter))
        })
    }
    matches(
        document.course.as_deref().unwrap_or_default(),
        filter.course.as_deref(),
    ) && matches(
        document.semester.as_deref().unwrap_or_default(),
        filter.semester.as_deref(),
    ) && matches(&document.category, filter.category.as_deref())
        && matches(&document.format, filter.format.as_deref())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalDocument {
    pub relative_path: String,
    pub title: String,
    pub format: String,
    pub size_bytes: u64,
    pub modified_at: Option<u64>,
    pub course: Option<String>,
    pub semester: Option<String>,
    pub category: String,
    pub read_status: String,
    pub read_reason: Option<String>,
    pub searchable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalCatalogSnapshot {
    pub root: String,
    pub documents: Vec<LocalDocument>,
    pub folders: usize,
    pub unreadable_directories: usize,
    pub unreadable_files: usize,
    pub truncated: bool,
    pub status: String,
    pub scanned_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalDocumentContent {
    pub document: LocalDocument,
    pub markdown: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalSearchResult {
    pub document: LocalDocument,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalSearchResponse {
    pub results: Vec<LocalSearchResult>,
    pub unreadable_files: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalSearchFilter {
    pub course: Option<String>,
    pub semester: Option<String>,
    pub category: Option<String>,
    pub format: Option<String>,
}

#[derive(Default)]
struct CatalogIndex {
    snapshot: Option<LocalCatalogSnapshot>,
    text_by_path: HashMap<String, String>,
}

pub struct CatalogService {
    root: Mutex<PathBuf>,
    index: Mutex<CatalogIndex>,
}

impl CatalogService {
    pub fn new(root: PathBuf) -> Result<Self, String> {
        let root = canonical_library_root(&root)?;
        Ok(Self {
            root: Mutex::new(root),
            index: Mutex::new(CatalogIndex::default()),
        })
    }

    pub fn set_root(&self, root: &Path) -> Result<(), String> {
        let root = canonical_library_root(root)?;
        *self.root.lock().map_err(|error| error.to_string())? = root;
        *self.index.lock().map_err(|error| error.to_string())? = CatalogIndex::default();
        Ok(())
    }

    pub fn snapshot(&self) -> Result<LocalCatalogSnapshot, String> {
        self.index
            .lock()
            .map_err(|error| error.to_string())?
            .snapshot
            .clone()
            .ok_or_else(|| "O acervo ainda não foi indexado; atualize a pasta.".to_owned())
    }

    pub fn refresh(&self) -> Result<LocalCatalogSnapshot, String> {
        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        let mut snapshot = scan_library(&root)?;
        let mut text_by_path = HashMap::new();
        let mut indexed_bytes = 0_usize;
        for document in &mut snapshot.documents {
            if !document.searchable {
                continue;
            }
            match read_document(&root, &document.relative_path) {
                Ok(content) => {
                    let size = content.markdown.len();
                    if indexed_bytes.saturating_add(size) > MAX_INDEX_TEXT_BYTES {
                        document.searchable = false;
                        document.read_status = "index_limit".into();
                        document.read_reason = Some(
                            "O limite de 128 MiB de texto indexado foi atingido; atualize após reduzir o acervo.".into(),
                        );
                        snapshot.truncated = true;
                        continue;
                    }
                    indexed_bytes += size;
                    if !content.warnings.is_empty() {
                        document.read_reason = Some(content.warnings.join(" "));
                    }
                    text_by_path.insert(document.relative_path.clone(), content.markdown);
                }
                Err(error) => {
                    document.searchable = false;
                    document.read_status = "unreadable".into();
                    document.read_reason = Some(error);
                    snapshot.unreadable_files += 1;
                }
            }
        }
        if snapshot.truncated {
            snapshot.status = "partial".into();
        }
        let mut index = self.index.lock().map_err(|error| error.to_string())?;
        index.snapshot = Some(snapshot.clone());
        index.text_by_path = text_by_path;
        Ok(snapshot)
    }

    pub fn read_document(&self, relative_path: &str) -> Result<LocalDocumentContent, String> {
        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        read_document(&root, relative_path)
    }

    pub fn read_asset(&self, relative_path: &str) -> Result<Vec<u8>, String> {
        let root = self.root.lock().map_err(|error| error.to_string())?.clone();
        let (canonical_root, path) = resolve_path(&root, relative_path)?;
        if !is_asset_file(&path) {
            return Err("Este arquivo não é uma imagem local suportada.".into());
        }
        let metadata = path
            .metadata()
            .map_err(|error| format!("Não foi possível ler a imagem local: {error}"))?;
        if metadata.len() > MAX_ASSET_BYTES {
            return Err("Esta imagem excede o limite de 16 MB para leitura no app.".into());
        }
        if !path.starts_with(canonical_root) {
            return Err("A imagem selecionada está fora da pasta do acervo.".into());
        }
        fs::read(path).map_err(|error| format!("Não foi possível abrir a imagem local: {error}"))
    }

    pub fn search(
        &self,
        query: &str,
        filter: &LocalSearchFilter,
    ) -> Result<LocalSearchResponse, String> {
        let query = normalize_for_search(query);
        if query.chars().count() > 200 {
            return Err("A busca aceita até 200 caracteres.".into());
        }
        let index = self.index.lock().map_err(|error| error.to_string())?;
        let snapshot = index
            .snapshot
            .as_ref()
            .ok_or_else(|| "O acervo ainda não foi indexado; atualize a pasta.".to_owned())?;
        let mut response = LocalSearchResponse {
            results: Vec::new(),
            unreadable_files: snapshot.unreadable_files,
            truncated: snapshot.truncated,
        };
        if query.is_empty() {
            return Ok(response);
        }
        for document in &snapshot.documents {
            if !matches_filter(document, filter) {
                continue;
            }
            let name_match = normalize_for_search(&document.relative_path).contains(&query);
            let excerpt = if name_match {
                Some(document.relative_path.clone())
            } else {
                index
                    .text_by_path
                    .get(&document.relative_path)
                    .and_then(|text| matching_line(text, &query))
            };
            if let Some(excerpt) = excerpt {
                response.results.push(LocalSearchResult {
                    document: document.clone(),
                    excerpt,
                });
                if response.results.len() >= MAX_SEARCH_RESULTS {
                    response.truncated = true;
                    break;
                }
            }
        }
        Ok(response)
    }
}

pub fn scan_library(root: &Path) -> Result<LocalCatalogSnapshot, String> {
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
    if !canonical_root.is_dir() {
        return Err("A pasta configurada para o acervo não existe ou não é uma pasta.".into());
    }

    let mut documents = Vec::new();
    let mut unreadable_directories = 0;
    let mut unreadable_files = 0;
    let mut truncated = false;
    walk_directory(
        &canonical_root,
        &canonical_root,
        0,
        &mut documents,
        &mut unreadable_directories,
        &mut unreadable_files,
        &mut truncated,
    );
    documents.sort_by(|left, right| {
        left.relative_path
            .to_lowercase()
            .cmp(&right.relative_path.to_lowercase())
    });

    let folders = documents
        .iter()
        .filter_map(|document| Path::new(&document.relative_path).parent())
        .filter(|parent| !parent.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .collect::<HashSet<_>>()
        .len();
    Ok(LocalCatalogSnapshot {
        root: canonical_root.display().to_string(),
        documents,
        folders,
        unreadable_directories,
        unreadable_files,
        truncated,
        status: if truncated || unreadable_directories > 0 || unreadable_files > 0 {
            "partial".into()
        } else {
            "ready".into()
        },
        scanned_at: std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    })
}

pub fn read_document(root: &Path, relative_path: &str) -> Result<LocalDocumentContent, String> {
    let (canonical_root, file_path) = resolve_document_path(root, relative_path)?;
    let metadata = file_path
        .metadata()
        .map_err(|error| format!("Não foi possível ler os dados do arquivo: {error}"))?;
    if metadata.len() > MAX_DOCUMENT_BYTES {
        return Err("Este arquivo excede o limite de 64 MB para leitura no app.".into());
    }
    if !is_supported_file(&file_path) {
        return Err("Este formato não pode ser exibido como documento.".into());
    }

    let document = document_metadata(&canonical_root, &file_path, metadata.len())?;
    let bytes = fs::read(&file_path)
        .map_err(|error| format!("Não foi possível abrir o documento local: {error}"))?;
    let extracted = extract_document(&file_path, &bytes).map_err(|error| error.to_string())?;

    Ok(LocalDocumentContent {
        document,
        markdown: extracted.text,
        warnings: extracted.warnings,
    })
}

fn walk_directory(
    root: &Path,
    directory: &Path,
    depth: usize,
    documents: &mut Vec<LocalDocument>,
    unreadable_directories: &mut usize,
    unreadable_files: &mut usize,
    truncated: &mut bool,
) {
    if depth > MAX_DEPTH {
        *truncated = true;
        return;
    }
    if documents.len() >= MAX_FILES {
        *truncated = true;
        return;
    }
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => {
            *unreadable_directories += 1;
            return;
        }
    };

    for entry in entries {
        if documents.len() >= MAX_FILES {
            *truncated = true;
            break;
        }
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                *unreadable_directories += 1;
                continue;
            }
        };
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => {
                *unreadable_files += 1;
                continue;
            }
        };
        let path = entry.path();
        if file_type.is_dir() {
            if should_skip_directory(root, &path) {
                continue;
            }
            walk_directory(
                root,
                &path,
                depth + 1,
                documents,
                unreadable_directories,
                unreadable_files,
                truncated,
            );
        } else if file_type.is_file() && is_in_scope_file(root, &path) {
            match entry.metadata() {
                Ok(metadata) => match document_metadata(root, &path, metadata.len()) {
                    Ok(document) => documents.push(document),
                    Err(_) => *unreadable_files += 1,
                },
                Err(_) => *unreadable_files += 1,
            }
        }
    }
}

fn should_skip_directory(root: &Path, directory: &Path) -> bool {
    let Some(name) = directory.file_name().and_then(|name| name.to_str()) else {
        return true;
    };
    if name.starts_with('.')
        || matches!(
            name.to_ascii_lowercase().as_str(),
            "app" | "node_modules" | "target" | "dist" | "build"
        )
    {
        return true;
    }

    if directory == root {
        return false;
    }
    let relative = directory.strip_prefix(root).unwrap_or(directory);
    let components = relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => value.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>();
    if components.iter().any(|part| is_credential_filename(part)) {
        return false;
    }
    let in_academic_tree = matches!(
        components
            .first()
            .map(|part| part.to_ascii_lowercase())
            .as_deref(),
        Some("docs" | "materias" | "originais" | "fontes-uva" | "periodos" | "cursos")
    );
    !in_academic_tree
        || components.windows(2).any(|parts| {
            parts[0].eq_ignore_ascii_case("docs") && parts[1].eq_ignore_ascii_case("superpowers")
        })
}

fn is_in_scope_file(root: &Path, path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    if name.starts_with('.')
        || name.starts_with("~$")
        || name.eq_ignore_ascii_case("thumbs.db")
        || is_credential_filename(name)
    {
        return false;
    }
    let relative = path.strip_prefix(root).unwrap_or(path);
    let components = relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => value.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>();
    match components.as_slice() {
        [root_file] => is_root_register(root_file),
        [first, second, ..] if first.eq_ignore_ascii_case("docs") => {
            !second.eq_ignore_ascii_case("superpowers")
        }
        [first, ..] => {
            first.eq_ignore_ascii_case("materias")
                || first.eq_ignore_ascii_case("originais")
                || first.eq_ignore_ascii_case("fontes-uva")
                || first.eq_ignore_ascii_case("periodos")
                || first.eq_ignore_ascii_case("cursos")
        }
        _ => false,
    }
}

fn is_root_register(file_name: &str) -> bool {
    let name = file_name.to_ascii_lowercase();
    [
        "registro_academico",
        "cursos_",
        "materiais_",
        "calendario_",
        "notas_de_estudo_",
        "futuros_semestres",
        "planejamento",
        "engenharia_de_computacao",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix))
}

fn is_credential_filename(file_name: &str) -> bool {
    let name = file_name.to_ascii_lowercase();
    [
        "senha",
        "password",
        "credential",
        "credencial",
        "cookie",
        "token",
        "secret",
    ]
    .iter()
    .any(|marker| name.contains(marker))
}

fn is_supported_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "md" | "markdown"
                    | "txt"
                    | "alg"
                    | "csv"
                    | "json"
                    | "html"
                    | "htm"
                    | "pdf"
                    | "docx"
                    | "pages"
            )
        })
}

fn is_asset_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "webp"
            )
        })
}

fn document_metadata(root: &Path, path: &Path, size_bytes: u64) -> Result<LocalDocument, String> {
    let relative_path = path
        .strip_prefix(root)
        .map_err(|_| "O arquivo está fora da pasta configurada.".to_owned())?
        .to_string_lossy()
        .replace('\\', "/");
    let title = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("Documento sem título")
        .to_owned();
    let format = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("arquivo")
        .to_ascii_lowercase();
    let classification: PathClassification = classify_path(Path::new(&relative_path));
    let supported = is_supported_file(path);
    let asset = is_asset_file(path);
    let (read_status, read_reason) = if !supported && !asset {
        (
            "unsupported",
            Some("Este formato não possui leitor disponível.".into()),
        )
    } else if size_bytes
        > if asset {
            MAX_ASSET_BYTES
        } else {
            MAX_DOCUMENT_BYTES
        }
    {
        (
            "too_large",
            Some("O arquivo excede o limite de leitura do app.".into()),
        )
    } else if asset {
        ("asset", None)
    } else {
        ("ready", None)
    };
    let modified_at = fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs());
    let is_raw_original = relative_path
        .split('/')
        .next()
        .is_some_and(|part| part.eq_ignore_ascii_case("originais"));
    Ok(LocalDocument {
        relative_path,
        title,
        format,
        size_bytes,
        modified_at,
        course: classification.course,
        semester: classification.semester,
        category: classification.category,
        read_status: read_status.into(),
        read_reason,
        searchable: supported && size_bytes <= MAX_DOCUMENT_BYTES && !is_raw_original,
    })
}

fn resolve_document_path(root: &Path, relative_path: &str) -> Result<(PathBuf, PathBuf), String> {
    let requested = Path::new(relative_path);
    if requested.as_os_str().is_empty()
        || requested.is_absolute()
        || requested
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("Caminho local inválido.".into());
    }
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
    let file_path = canonical_root
        .join(requested)
        .canonicalize()
        .map_err(|error| format!("Não foi possível abrir este documento: {error}"))?;
    if !file_path.starts_with(&canonical_root) || !file_path.is_file() {
        return Err("O documento selecionado está fora da pasta do acervo.".into());
    }
    if !is_supported_file(&file_path) {
        return Err("Este formato ainda não pode ser lido pelo app.".into());
    }
    Ok((canonical_root, file_path))
}

fn matching_line(content: &str, needle: &str) -> Option<String> {
    let needle = normalize_for_search(needle);
    content.lines().find_map(|line| {
        if normalize_for_search(line).contains(&needle) {
            let excerpt = line.trim().chars().take(240).collect::<String>();
            Some(if excerpt.is_empty() {
                "Trecho correspondente encontrado no documento.".into()
            } else {
                excerpt
            })
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{CatalogService, LocalSearchFilter, read_document, scan_library};
    use std::fs;

    #[test]
    fn unsupported_academic_files_remain_visible_in_the_catalog() {
        let root = tempfile::tempdir().unwrap();
        let course = root.path().join("materias/bases-de-programacao");
        fs::create_dir_all(&course).unwrap();
        fs::write(course.join("anexo.bin"), [0_u8, 1, 2, 3]).unwrap();

        let snapshot = scan_library(root.path()).unwrap();

        assert!(snapshot
            .documents
            .iter()
            .any(|document| document.relative_path == "materias/bases-de-programacao/anexo.bin"));
    }

    #[test]
    fn indexes_top_level_academic_folders_moved_out_of_materias() {
        let root = tempfile::tempdir().unwrap();
        let files = [
            "fontes-uva/2026.2/bases-de-programacao/BPR_Cap_01.md",
            "periodos/2026.2/README.md",
            "cursos/engenharia-de-computacao/README.md",
        ];
        for relative in files {
            let file = root.path().join(relative);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(&file, "Conteúdo acadêmico.").unwrap();
        }

        let snapshot = scan_library(root.path()).unwrap();

        for relative in files {
            assert!(
                snapshot
                    .documents
                    .iter()
                    .any(|document| document.relative_path == relative),
                "{relative} deveria estar no catálogo"
            );
        }
    }

    #[test]
    fn traversal_does_not_follow_symlink_outside_the_library() {
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("privado.txt"), "conteúdo externo").unwrap();
        let materias = root.path().join("materias");
        fs::create_dir_all(&materias).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(outside.path(), materias.join("atalho")).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(outside.path(), materias.join("atalho")).unwrap();

        let snapshot = scan_library(root.path()).unwrap();

        assert!(
            !snapshot
                .documents
                .iter()
                .any(|document| document.relative_path.contains("privado"))
        );
        assert!(read_document(root.path(), "materias/atalho/privado.txt").is_err());
    }

    #[test]
    fn refresh_reflects_added_changed_and_removed_source_files() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("materias/nota.md");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "versão 1").unwrap();
        assert!(
            scan_library(root.path())
                .unwrap()
                .documents
                .iter()
                .any(|document| document.relative_path == "materias/nota.md")
        );

        fs::write(&file, "versão 2").unwrap();
        fs::write(root.path().join("materias/nova.txt"), "novo").unwrap();
        let changed = scan_library(root.path()).unwrap();
        assert!(
            changed
                .documents
                .iter()
                .any(|document| document.relative_path == "materias/nova.txt")
        );
        assert_eq!(
            read_document(root.path(), "materias/nota.md")
                .unwrap()
                .markdown,
            "versão 2"
        );

        fs::remove_file(file).unwrap();
        assert!(
            !scan_library(root.path())
                .unwrap()
                .documents
                .iter()
                .any(|document| document.relative_path == "materias/nota.md")
        );
        assert!(
            scan_library(root.path())
                .unwrap()
                .documents
                .iter()
                .any(|document| document.relative_path == "materias/nova.txt")
        );
    }

    #[test]
    fn search_matches_portuguese_words_without_diacritics() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("materias/aula.md");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "A equação resolve a questão.").unwrap();
        let catalog = CatalogService::new(root.path().to_path_buf()).unwrap();
        catalog.refresh().unwrap();

        let result = catalog
            .search("equacao", &LocalSearchFilter::default())
            .unwrap();

        assert_eq!(result.results.len(), 1);
        assert!(result.results[0].excerpt.contains("equação"));
    }
}
