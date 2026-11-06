//! Serviços de leitura do acervo para a interface.
//!
//! Só usa o índice do acervo em memória (`CatalogService`) e leitura de arquivos.
//! Não abre o SQLite `.estuda-uva.sqlite3`: abrir o `AcademicStore` ou o `SyncEngine`
//! executa `CREATE TABLE`/`ALTER TABLE`, e isso exige aprovação prévia do dono.

use std::path::PathBuf;
use std::sync::Arc;

use estuda_core::catalog::scanner::{CatalogService, LocalCatalogSnapshot, LocalDocumentContent};

/// Raiz do acervo: o diretório do repositório, a partir de `app/crates/estuda-gpui`.
pub fn default_library() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.join("../../..");
    root.canonicalize().unwrap_or(root)
}

/// Memória residente (RSS) deste processo, em MB, lida de `ps`. `None` se a leitura falhar.
pub fn resident_mb() -> Option<u64> {
    let output = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()?;
    let kilobytes: u64 = String::from_utf8_lossy(&output.stdout).trim().parse().ok()?;
    Some(kilobytes / 1024)
}

/// Serviços compartilhados pela interface. Clonar é barato (`Arc`).
#[derive(Clone)]
pub struct Services {
    catalog: Arc<CatalogService>,
}

impl Services {
    pub fn open(root: PathBuf) -> Result<Self, String> {
        Ok(Self {
            catalog: Arc::new(CatalogService::new(root)?),
        })
    }

    /// Relê a pasta do acervo e atualiza o índice em RAM.
    pub fn refresh(&self) -> Result<LocalCatalogSnapshot, String> {
        self.catalog.refresh()
    }

    pub fn read_document(&self, relative_path: &str) -> Result<LocalDocumentContent, String> {
        self.catalog.read_document(relative_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lê o acervo real (somente leitura) e imprime contagens agregadas, sem títulos.
    /// Rode com: cargo test -p estuda-gpui services -- --ignored --nocapture
    #[test]
    #[ignore]
    fn contagem_do_acervo_real() {
        let services = Services::open(default_library()).expect("abrir acervo");
        let snapshot = services.refresh().expect("varrer acervo");
        let mut by_category = std::collections::BTreeMap::<String, usize>::new();
        let mut by_format = std::collections::BTreeMap::<String, usize>::new();
        for document in &snapshot.documents {
            *by_category.entry(document.category.clone()).or_default() += 1;
            *by_format.entry(document.format.clone()).or_default() += 1;
        }
        println!("documentos: {}", snapshot.documents.len());
        println!("pastas: {} | ilegíveis: {} | truncado: {}", snapshot.folders, snapshot.unreadable_files, snapshot.truncated);
        println!("por categoria: {by_category:?}");
        println!("por formato: {by_format:?}");
    }
}
