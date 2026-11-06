//! Serviços de leitura do acervo para a interface.
//!
//! Só usa o índice do acervo em memória (`CatalogService`) e leitura de arquivos.
//! Não abre o SQLite `.estuda-uva.sqlite3`: abrir o `AcademicStore` ou o `SyncEngine`
//! executa `CREATE TABLE`/`ALTER TABLE`, e isso exige aprovação prévia do dono.

use std::path::PathBuf;
use std::sync::Arc;

use estuda_core::catalog::scanner::{CatalogService, LocalCatalogSnapshot, LocalDocumentContent};
use estuda_core::sync::SyncEngine;

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

/// Texto em português para o estado de identidade vindo dos dados (sem o identificador cru).
pub fn identity_label(raw: &str) -> String {
    match raw {
        "matricula_registrada" => "matrícula registrada".to_string(),
        "same_title_and_workload_candidate" => "candidata: mesmo título e carga horária".to_string(),
        "" => String::new(),
        other => other.replace('_', " "),
    }
}

/// Trajetória (matrícula) do estudante no catálogo: programa, situação e semestre.
#[derive(Clone, Debug)]
pub struct Enrollment {
    pub program: String,
    pub status: String,
    pub semester: String,
    pub active: bool,
}

/// Disciplina do catálogo acadêmico, lida da projeção do banco (que vem de `dados/`).
#[derive(Clone, Debug)]
pub struct Discipline {
    pub id: String,
    pub title: String,
    pub identity_status: String,
    pub pending: usize,
}

/// Serviços compartilhados pela interface. Clonar é barato (`Arc`).
#[derive(Clone)]
pub struct Services {
    catalog: Arc<CatalogService>,
    sync: Arc<SyncEngine>,
}

impl Services {
    /// Abre o índice do acervo e o banco. Abrir só garante as tabelas de sincronização
    /// (`CREATE TABLE IF NOT EXISTS`); não reescreve nem regrava nenhuma linha.
    pub fn open(root: PathBuf) -> Result<Self, String> {
        let database = root.join(".estuda-uva.sqlite3");
        let sync = SyncEngine::open(&database, &root)?;
        Ok(Self {
            catalog: Arc::new(CatalogService::new(root)?),
            sync: Arc::new(sync),
        })
    }

    /// Trajetórias do catálogo (uma por programa). Só lê a projeção do banco.
    pub fn enrollments(&self) -> Result<Vec<Enrollment>, String> {
        let catalog = estuda_core::catalog::academic::load(&self.sync)?;
        let text = |value: &serde_json::Value, key: &str| {
            value.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
        };
        Ok(catalog
            .trajectories
            .iter()
            .map(|value| {
                let status = text(value, "status");
                let active = status == "active";
                Enrollment {
                    program: text(value, "programTitle"),
                    status: match status.as_str() {
                        "active" => "Ativo".to_string(),
                        "evadido" => "Evadido".to_string(),
                        other => other.to_string(),
                    },
                    semester: text(value, "semester"),
                    active,
                }
            })
            .collect())
    }

    /// Disciplinas do catálogo acadêmico já sincronizadas. Só lê a projeção do banco.
    pub fn disciplines(&self) -> Result<Vec<Discipline>, String> {
        let catalog = estuda_core::catalog::academic::load(&self.sync)?;
        let text = |value: &serde_json::Value, key: &str| {
            value.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
        };
        Ok(catalog
            .disciplines
            .iter()
            .map(|value| Discipline {
                id: text(value, "id"),
                title: text(value, "title"),
                identity_status: identity_label(&text(value, "identityStatus")),
                pending: value
                    .get("pendencias")
                    .and_then(|v| v.as_array())
                    .map_or(0, Vec::len),
            })
            .collect())
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

    /// Lê as disciplinas do banco sincronizado (somente leitura) e imprime só a contagem.
    /// Rode com: cargo test -p estuda-gpui disciplinas_do_banco -- --ignored --nocapture
    #[test]
    #[ignore]
    fn disciplinas_do_banco() {
        let services = Services::open(default_library()).expect("abrir acervo e banco");
        let disciplines = services.disciplines().expect("ler disciplinas");
        println!("disciplinas no catálogo: {}", disciplines.len());
        println!("com pendências: {}", disciplines.iter().filter(|d| d.pending > 0).count());
        let enrollments = services.enrollments().expect("ler matrículas");
        println!("matrículas no catálogo: {} (ativas: {})", enrollments.len(), enrollments.iter().filter(|e| e.active).count());
    }

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
