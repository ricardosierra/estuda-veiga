//! Esquema SQLite do Estuda UVA: **uma baseline só**, sem `ALTER TABLE`.
//!
//! O banco local é 100% regenerável: documentos e projeção `academic_*` vêm dos arquivos do
//! acervo (`sync::reconcile_all`), e o resto é estado de uso. Por isso não há cadeia de
//! migrações incrementais. Se o arquivo tem versão **menor** que [`CURRENT_VERSION`], ele é
//! guardado como backup (`storage::quarantine_old_database`) e um banco novo é criado a partir
//! de [`BASELINE`]. Versão **maior** é recusada sem tocar no arquivo.
//!
//! Mudou o esquema? Edite [`BASELINE`], some 1 em [`CURRENT_VERSION`] e atualize os testes de
//! `storage`. Nunca acrescente `ALTER TABLE`.
//!
//! As tabelas `academic_*` (catálogo) e `sync_*` (sincronização de documentos) são criadas por
//! `catalog::academic` e `sync` com `CREATE TABLE IF NOT EXISTS`, também sem `ALTER`.

pub const CURRENT_VERSION: i64 = 8;

/// Tabelas esperadas num banco criado pela baseline (sem as de `catalog` e `sync`).
pub const BASELINE_TABLES: &[&str] = &[
    "courses",
    "materials",
    "material_associations",
    "study_sessions",
    "lesson_sessions",
    "lesson_interactions",
    "practice_questions",
    "study_summaries",
    "academic_events",
    "settings",
    "capture_runs",
    "capture_items",
    "sync_runs",
    "http_validators",
    "source_observations",
    "ava_checks",
    "exam_records",
];

pub const BASELINE: &str = "
PRAGMA journal_mode=WAL;

CREATE TABLE IF NOT EXISTS courses (
  id TEXT PRIMARY KEY, title TEXT NOT NULL, url TEXT NOT NULL, kind TEXT NOT NULL,
  seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS materials (
  id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL, module_title TEXT NOT NULL,
  title TEXT NOT NULL, path TEXT NOT NULL, lesson_path TEXT NOT NULL DEFAULT '', origin TEXT NOT NULL,
  pages INTEGER NOT NULL, semester TEXT, sha256 TEXT NOT NULL,
  imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  source_key TEXT NOT NULL DEFAULT '', source_kind TEXT NOT NULL DEFAULT 'pdf',
  source_etag TEXT, source_last_modified TEXT, source_validator_host TEXT, text_sha256 TEXT
);
CREATE INDEX IF NOT EXISTS materials_source_version ON materials(source_key, text_sha256, imported_at DESC);

CREATE TABLE IF NOT EXISTS material_associations (
  material_id TEXT NOT NULL, course_id TEXT NOT NULL, course_title TEXT NOT NULL,
  module_title TEXT NOT NULL, item_title TEXT NOT NULL, semester TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(material_id, course_id, module_title, item_title)
);
CREATE INDEX IF NOT EXISTS material_associations_course ON material_associations(course_id, semester, material_id);

CREATE TABLE IF NOT EXISTS study_sessions (
  id TEXT PRIMARY KEY, course_id TEXT NOT NULL, material_id TEXT NOT NULL,
  correct INTEGER NOT NULL, total INTEGER NOT NULL, percent INTEGER NOT NULL,
  studied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS lesson_sessions (
  id TEXT PRIMARY KEY, resource_path TEXT NOT NULL, course_id TEXT, course_title TEXT NOT NULL,
  material_title TEXT NOT NULL, section_index INTEGER NOT NULL DEFAULT 0, section_count INTEGER NOT NULL,
  started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  completed_at TEXT
);
CREATE INDEX IF NOT EXISTS lesson_sessions_resource_opened ON lesson_sessions(resource_path, completed_at, updated_at);
CREATE INDEX IF NOT EXISTS lesson_sessions_history ON lesson_sessions(updated_at DESC);

CREATE TABLE IF NOT EXISTS lesson_interactions (
  id TEXT PRIMARY KEY, session_id TEXT NOT NULL, activity_id TEXT NOT NULL, section_index INTEGER NOT NULL,
  kind TEXT NOT NULL, prompt TEXT NOT NULL, response TEXT NOT NULL, assessment TEXT NOT NULL,
  source_page INTEGER, source_heading TEXT NOT NULL, source_excerpt TEXT NOT NULL, source_sha256 TEXT NOT NULL,
  attempted_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS lesson_interactions_session_order ON lesson_interactions(session_id, attempted_at, id);

CREATE TABLE IF NOT EXISTS practice_questions (
  material_id TEXT NOT NULL, position INTEGER NOT NULL, prompt TEXT NOT NULL, answer TEXT NOT NULL,
  source_page INTEGER NOT NULL, PRIMARY KEY(material_id, position)
);

CREATE TABLE IF NOT EXISTS study_summaries (
  id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL, semester TEXT NOT NULL,
  source_hash TEXT NOT NULL, source_manifest_json TEXT NOT NULL, short_text TEXT NOT NULL, long_text TEXT NOT NULL,
  model TEXT NOT NULL, input_tokens INTEGER NOT NULL, output_tokens INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS study_summaries_subject_latest ON study_summaries(course_id, semester, created_at DESC);

CREATE TABLE IF NOT EXISTS academic_events (
  id TEXT PRIMARY KEY, semester TEXT NOT NULL, course_id TEXT, course_title TEXT,
  title TEXT NOT NULL, starts_at TEXT NOT NULL, all_day INTEGER NOT NULL,
  source_kind TEXT NOT NULL, source_url TEXT NOT NULL, source_label TEXT NOT NULL,
  verified_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS academic_events_semester_date ON academic_events(semester, starts_at);

CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);

CREATE TABLE IF NOT EXISTS capture_runs (
  id TEXT PRIMARY KEY, scope TEXT NOT NULL, limits_snapshot TEXT NOT NULL, status TEXT NOT NULL,
  courses_total INTEGER NOT NULL DEFAULT 0, courses_done INTEGER NOT NULL DEFAULT 0,
  items_total INTEGER NOT NULL DEFAULT 0, captured INTEGER NOT NULL DEFAULT 0, reused INTEGER NOT NULL DEFAULT 0,
  ignored INTEGER NOT NULL DEFAULT 0, failed INTEGER NOT NULL DEFAULT 0, needs_review INTEGER NOT NULL DEFAULT 0,
  pending INTEGER NOT NULL DEFAULT 0, current_course_id TEXT, current_course_title TEXT,
  current_module_title TEXT, current_item_title TEXT, warning TEXT,
  started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  finished_at TEXT
);
CREATE INDEX IF NOT EXISTS capture_runs_latest ON capture_runs(status, updated_at DESC);

CREATE TABLE IF NOT EXISTS capture_items (
  run_id TEXT NOT NULL, position INTEGER NOT NULL DEFAULT 0, reference_key TEXT NOT NULL, content_key TEXT,
  canvas_item_id TEXT, course_id TEXT NOT NULL, course_title TEXT NOT NULL, module_title TEXT NOT NULL,
  title TEXT NOT NULL, source_kind TEXT NOT NULL, source_url TEXT NOT NULL, status TEXT NOT NULL,
  retryable INTEGER NOT NULL DEFAULT 0, attempts INTEGER NOT NULL DEFAULT 0, material_id TEXT, message TEXT,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(run_id, reference_key)
);
CREATE INDEX IF NOT EXISTS capture_items_course_order ON capture_items(run_id, course_id, position, reference_key);

-- Motor de sincronização acadêmica (crate estuda-sync). Dados privados; nunca vão ao catálogo público.

-- Uma execução da sincronização: etapas, plano proposto (aguardando aprovação) e métricas.
CREATE TABLE IF NOT EXISTS sync_runs (
  id TEXT PRIMARY KEY,
  status TEXT NOT NULL CHECK(status IN ('in_progress','completed','completed_pending_approval','cancelled','failed')),
  steps_json TEXT NOT NULL,
  plan_digest TEXT, plan_json TEXT, metrics_json TEXT,
  started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  finished_at TEXT
);
CREATE INDEX IF NOT EXISTS sync_runs_latest ON sync_runs(status, updated_at DESC);

-- ETag/Last-Modified por URL (já sem query), para requisições condicionais e HTTP 304.
CREATE TABLE IF NOT EXISTS http_validators (
  url_hash TEXT PRIMARY KEY, url TEXT NOT NULL, etag TEXT, last_modified TEXT,
  content_hash TEXT NOT NULL, byte_len INTEGER NOT NULL, fetched_at TEXT NOT NULL
);

-- Toda leitura de fonte com proveniência. id = sha256(sistema|entidade|chave|hash): idempotente.
CREATE TABLE IF NOT EXISTS source_observations (
  id TEXT PRIMARY KEY, run_id TEXT,
  source_system TEXT NOT NULL, source_url TEXT, read_at TEXT NOT NULL,
  entity TEXT NOT NULL, entity_key TEXT NOT NULL,
  verification TEXT NOT NULL CHECK(verification IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  content_hash TEXT NOT NULL, payload_json TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS source_observations_entity ON source_observations(entity, entity_key, read_at DESC);

-- AVA 1 e AVA 2 por disciplina e período. Estados explícitos: ausência de dado é 'unknown', nunca 0.
CREATE TABLE IF NOT EXISTS ava_checks (
  subject_key TEXT NOT NULL, term TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('ava1','ava2')),
  existence TEXT NOT NULL CHECK(existence IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  assessment_type TEXT,
  due_at TEXT, due_at_state TEXT NOT NULL CHECK(due_at_state IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  submission TEXT, submission_state TEXT NOT NULL CHECK(submission_state IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  grade_hundredths INTEGER, grade_state TEXT NOT NULL CHECK(grade_state IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  feedback TEXT, feedback_state TEXT NOT NULL CHECK(feedback_state IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  link_confidence TEXT CHECK(link_confidence IN ('explicit','probable')),
  source_url TEXT, read_at TEXT NOT NULL,
  PRIMARY KEY(subject_key, term, kind)
);

-- Provas por disciplina e período. Sem confirmação oficial a data fica NULL e a UI mostra 'a confirmar'.
CREATE TABLE IF NOT EXISTS exam_records (
  subject_key TEXT NOT NULL, term TEXT NOT NULL, name TEXT NOT NULL,
  exam_date TEXT, date_state TEXT NOT NULL CHECK(date_state IN ('confirmed','not_found','unknown','conflicting','not_applicable')),
  exam_time TEXT, location TEXT,
  scheduling TEXT NOT NULL CHECK(scheduling IN ('scheduled','not_scheduled','unknown','conflicting')),
  status TEXT NOT NULL CHECK(status IN ('not_yet_taken','taken','unknown','conflicting')),
  source_system TEXT NOT NULL, source_url TEXT, read_at TEXT NOT NULL,
  PRIMARY KEY(subject_key, term, name)
);

PRAGMA user_version=8;
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_has_no_alter_table_and_declares_its_version() {
        assert!(!BASELINE.to_ascii_uppercase().contains("ALTER TABLE"));
        assert!(BASELINE.contains(&format!("PRAGMA user_version={CURRENT_VERSION};")));
    }

    #[test]
    fn baseline_is_fully_idempotent() {
        // Todo CREATE precisa ser IF NOT EXISTS: reabrir o banco nunca falha nem recria.
        for line in BASELINE.lines().map(str::trim) {
            if line.starts_with("CREATE TABLE") || line.starts_with("CREATE INDEX") {
                assert!(line.contains("IF NOT EXISTS"), "{line}");
            }
        }
    }

    #[test]
    fn every_listed_table_is_declared() {
        for table in BASELINE_TABLES {
            assert!(
                BASELINE.contains(&format!("CREATE TABLE IF NOT EXISTS {table} ")),
                "{table}"
            );
        }
    }
}
