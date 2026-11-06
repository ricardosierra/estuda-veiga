use crate::calendar::CalendarEvent;
use crate::canvas::{Course, CourseKind};
use crate::canvas_capture::{
    CanvasCaptureLimits, CaptureCounts, CaptureItem, CaptureItemStatus, CaptureRun,
    CaptureRunStatus, MaterialAssociation,
};
use crate::lesson::LessonQuestion;
use crate::tutor::{TutorActivity, TutorAssessment, TutorAssessmentStatus};
use rusqlite::{Connection, OptionalExtension, Row, backup::Backup, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

const CURRENT_SCHEMA_VERSION: i64 = 7;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("não foi possível preparar o banco local: {0}")]
    Io(String),
    #[error("não foi possível salvar os dados locais: {0}")]
    Sql(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedMaterial {
    pub id: String,
    pub course_id: String,
    pub course_title: String,
    pub module_title: String,
    pub title: String,
    pub path: String,
    pub lesson_path: String,
    pub origin: String,
    pub pages: usize,
    #[serde(default, skip_serializing)]
    pub sha256: String,
    #[serde(default)]
    pub semester: Option<String>,
    pub imported_at: String,
    #[serde(default)]
    pub source_key: String,
    #[serde(default)]
    pub source_kind: String,
    #[serde(default)]
    pub source_validator_host: Option<String>,
    #[serde(default)]
    pub source_etag: Option<String>,
    #[serde(default)]
    pub source_last_modified: Option<String>,
    #[serde(default)]
    pub text_sha256: Option<String>,
    #[serde(default)]
    pub associations: Vec<MaterialAssociation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    pub current_courses: usize,
    pub past_courses: usize,
    pub materials: usize,
    pub study_sessions: usize,
    pub accuracy_percent: u8,
    pub library_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonSession {
    pub id: String,
    pub resource_path: String,
    pub course_id: Option<String>,
    pub course_title: String,
    pub material_title: String,
    pub section_index: usize,
    pub section_count: usize,
    pub started_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonInteraction {
    pub id: String,
    pub session_id: String,
    pub activity_id: String,
    pub section_index: usize,
    pub kind: String,
    pub prompt: String,
    pub response: String,
    pub assessment: TutorAssessment,
    pub source_page: Option<u32>,
    pub source_heading: String,
    pub source_excerpt: String,
    pub source_sha256: String,
    pub attempted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudySummary {
    pub id: String,
    pub course_id: String,
    pub course_title: String,
    pub semester: String,
    pub source_hash: String,
    pub source_manifest_json: String,
    pub short_text: String,
    pub long_text: String,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyHistoryEntry {
    pub id: String,
    pub kind: String,
    pub course_id: Option<String>,
    pub course_title: Option<String>,
    pub material_id: Option<String>,
    pub resource_path: Option<String>,
    pub material_title: String,
    pub correct: Option<u32>,
    pub total: Option<u32>,
    pub percent: Option<u8>,
    pub section_index: Option<usize>,
    pub section_count: Option<usize>,
    pub started_at: String,
    pub occurred_at: String,
    pub completed_at: Option<String>,
}

fn lesson_session_from_row(row: &Row<'_>) -> rusqlite::Result<LessonSession> {
    Ok(LessonSession {
        id: row.get(0)?,
        resource_path: row.get(1)?,
        course_id: row.get(2)?,
        course_title: row.get(3)?,
        material_title: row.get(4)?,
        section_index: row.get::<_, i64>(5)?.max(0) as usize,
        section_count: row.get::<_, i64>(6)?.max(0) as usize,
        started_at: row.get(7)?,
        updated_at: row.get(8)?,
        completed_at: row.get(9)?,
    })
}

fn lesson_interaction_from_row(row: &Row<'_>) -> rusqlite::Result<LessonInteraction> {
    let assessment_json = row.get::<_, String>(7)?;
    let assessment = serde_json::from_str(&assessment_json).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(error))
    })?;
    Ok(LessonInteraction {
        id: row.get(0)?,
        session_id: row.get(1)?,
        activity_id: row.get(2)?,
        section_index: row.get::<_, i64>(3)?.max(0) as usize,
        kind: row.get(4)?,
        prompt: row.get(5)?,
        response: row.get(6)?,
        assessment,
        source_page: row.get::<_, Option<i64>>(8)?.map(|page| page.max(0) as u32),
        source_heading: row.get(9)?,
        source_excerpt: row.get(10)?,
        source_sha256: row.get(11)?,
        attempted_at: row.get(12)?,
        source_changed: None,
    })
}

fn study_summary_from_row(row: &Row<'_>) -> rusqlite::Result<StudySummary> {
    Ok(StudySummary {
        id: row.get(0)?,
        course_id: row.get(1)?,
        course_title: row.get(2)?,
        semester: row.get(3)?,
        source_hash: row.get(4)?,
        source_manifest_json: row.get(5)?,
        short_text: row.get(6)?,
        long_text: row.get(7)?,
        model: row.get(8)?,
        input_tokens: row.get::<_, i64>(9)?.max(0) as u64,
        output_tokens: row.get::<_, i64>(10)?.max(0) as u64,
        created_at: row.get(11)?,
    })
}

fn imported_material_from_row(row: &Row<'_>) -> rusqlite::Result<ImportedMaterial> {
    Ok(ImportedMaterial {
        id: row.get(0)?,
        course_id: row.get(1)?,
        course_title: row.get(2)?,
        module_title: row.get(3)?,
        title: row.get(4)?,
        path: row.get(5)?,
        lesson_path: row.get(6)?,
        origin: row.get(7)?,
        pages: row.get::<_, i64>(8)?.max(0) as usize,
        semester: row.get(9)?,
        sha256: row.get(10)?,
        imported_at: row.get(11)?,
        source_key: row.get(12)?,
        source_kind: row.get(13)?,
        source_validator_host: row.get(14)?,
        source_etag: row.get(15)?,
        source_last_modified: row.get(16)?,
        text_sha256: row.get(17)?,
        associations: Vec::new(),
    })
}

fn list_material_associations(
    connection: &Connection,
    material_id: &str,
) -> Result<Vec<MaterialAssociation>, StorageError> {
    let mut statement = connection
        .prepare(
            "SELECT material_id,course_id,course_title,module_title,item_title,semester
             FROM material_associations WHERE material_id=?1
             ORDER BY course_title,module_title,item_title",
        )
        .map_err(sql_error)?;
    let rows = statement
        .query_map([material_id], |row| {
            Ok(MaterialAssociation {
                material_id: row.get(0)?,
                course_id: row.get(1)?,
                course_title: row.get(2)?,
                module_title: row.get(3)?,
                item_title: row.get(4)?,
                semester: row.get(5)?,
            })
        })
        .map_err(sql_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
}

fn insert_material_association(
    connection: &Connection,
    association: &MaterialAssociation,
) -> Result<(), StorageError> {
    if association.material_id.trim().is_empty() || association.course_id.trim().is_empty() {
        return Err(StorageError::Sql(
            "a associação acadêmica não possui uma referência válida".into(),
        ));
    }
    let material_exists: i64 = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM materials WHERE id=?1)",
            [&association.material_id],
            |row| row.get(0),
        )
        .map_err(sql_error)?;
    if material_exists == 0 {
        return Err(StorageError::Sql("o material associado não existe".into()));
    }
    connection
        .execute(
            "INSERT INTO material_associations(
               material_id,course_id,course_title,module_title,item_title,semester
             ) VALUES(?1,?2,?3,?4,?5,?6)
             ON CONFLICT(material_id,course_id,module_title,item_title) DO UPDATE SET
               course_title=excluded.course_title,semester=COALESCE(excluded.semester,material_associations.semester)",
            params![
                association.material_id,
                association.course_id,
                clean_capture_text(Some(&association.course_title), 250)
                    .unwrap_or_else(|| "Disciplina".into()),
                clean_capture_text(Some(&association.module_title), 250)
                    .unwrap_or_else(|| "Sem módulo".into()),
                clean_capture_text(Some(&association.item_title), 250)
                    .unwrap_or_else(|| "Material acadêmico".into()),
                association.semester,
            ],
        )
        .map_err(sql_error)?;
    Ok(())
}

fn clean_validator(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    let lowercase = value.to_ascii_lowercase();
    if value.is_empty()
        || value.len() > 512
        || value.chars().any(char::is_control)
        || [
            "http://",
            "https://",
            "token",
            "signature",
            "credential",
            "cookie",
        ]
        .iter()
        .any(|marker| lowercase.contains(marker))
    {
        None
    } else {
        Some(value.to_owned())
    }
}

fn clean_validator_host(value: Option<&str>) -> Option<String> {
    let host = value?.trim().to_ascii_lowercase();
    let valid_chars = host.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
    });
    let allowed = host == "uva.instructure.com"
        || host == "uva.br"
        || host.ends_with(".uva.br")
        || (host.ends_with(".amazonaws.com") && host.contains(".s3."));
    (valid_chars && allowed && host.len() <= 253).then_some(host)
}

fn capture_run_from_row(row: &Row<'_>) -> rusqlite::Result<CaptureRun> {
    let limits_json: String = row.get(2)?;
    let limits_snapshot = serde_json::from_str(&limits_json).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(error))
    })?;
    let status_value: String = row.get(3)?;
    let status = CaptureRunStatus::from_db(&status_value).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            3,
            rusqlite::types::Type::Text,
            Box::new(std::io::Error::other("estado de captura inválido")),
        )
    })?;
    Ok(CaptureRun {
        id: row.get(0)?,
        scope: row.get(1)?,
        limits_snapshot,
        status,
        counts: CaptureCounts {
            courses_total: row.get::<_, i64>(4)?.max(0) as usize,
            courses_done: row.get::<_, i64>(5)?.max(0) as usize,
            items_total: row.get::<_, i64>(6)?.max(0) as usize,
            captured: row.get::<_, i64>(7)?.max(0) as usize,
            reused: row.get::<_, i64>(8)?.max(0) as usize,
            ignored: row.get::<_, i64>(9)?.max(0) as usize,
            failed: row.get::<_, i64>(10)?.max(0) as usize,
            needs_review: row.get::<_, i64>(11)?.max(0) as usize,
            pending: row.get::<_, i64>(12)?.max(0) as usize,
        },
        current_course_id: row.get(13)?,
        current_course_title: row.get(14)?,
        current_module_title: row.get(15)?,
        current_item_title: row.get(16)?,
        warning: row.get(17)?,
        started_at: row.get(18)?,
        updated_at: row.get(19)?,
        finished_at: row.get(20)?,
    })
}

fn capture_item_from_row(row: &Row<'_>) -> rusqlite::Result<CaptureItem> {
    let status_value: String = row.get(11)?;
    let status = CaptureItemStatus::from_db(&status_value).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            11,
            rusqlite::types::Type::Text,
            Box::new(std::io::Error::other("estado de item de captura inválido")),
        )
    })?;
    Ok(CaptureItem {
        position: row.get::<_, i64>(1)?.max(0) as usize,
        reference_key: row.get(2)?,
        content_key: row.get(3)?,
        canvas_item_id: row.get(4)?,
        course_id: row.get(5)?,
        course_title: row.get(6)?,
        module_title: row.get(7)?,
        title: row.get(8)?,
        source_kind: row.get(9)?,
        source_url: row.get(10)?,
        status,
        retryable: row.get::<_, i64>(12)? != 0,
        attempts: row.get::<_, i64>(13)?.max(0) as usize,
        material_id: row.get(14)?,
        message: row.get(15)?,
        updated_at: row.get(16)?,
    })
}

fn query_capture_counts(
    connection: &Connection,
    run_id: &str,
    courses_total: usize,
) -> Result<CaptureCounts, StorageError> {
    let counts = connection
        .query_row(
            "SELECT COUNT(*),
                    COALESCE(SUM(status='captured'),0),
                    COALESCE(SUM(status='reused'),0),
                    COALESCE(SUM(status='ignored'),0),
                    COALESCE(SUM(status='failed'),0),
                    COALESCE(SUM(status='needs_review'),0),
                    COALESCE(SUM(status IN ('pending','running')),0)
             FROM capture_items WHERE run_id=?1",
            [run_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            },
        )
        .map_err(sql_error)?;
    let courses_done: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM (
               SELECT DISTINCT item.course_id FROM capture_items AS item
               WHERE item.run_id=?1 AND NOT EXISTS (
                 SELECT 1 FROM capture_items AS pending
                 WHERE pending.run_id=item.run_id AND pending.course_id=item.course_id
                   AND pending.status IN ('pending','running')
               )
             )",
            [run_id],
            |row| row.get(0),
        )
        .map_err(sql_error)?;
    Ok(CaptureCounts {
        courses_total,
        courses_done: courses_done.max(0) as usize,
        items_total: counts.0.max(0) as usize,
        captured: counts.1.max(0) as usize,
        reused: counts.2.max(0) as usize,
        ignored: counts.3.max(0) as usize,
        failed: counts.4.max(0) as usize,
        needs_review: counts.5.max(0) as usize,
        pending: counts.6.max(0) as usize,
    })
}

fn clean_capture_text(value: Option<&str>, max_chars: usize) -> Option<String> {
    let value = value?.split_whitespace().collect::<Vec<_>>().join(" ");
    let lowercase = value.to_ascii_lowercase();
    if lowercase.contains("https://")
        || lowercase.contains("http://")
        || [
            "access_token=",
            "refresh_token=",
            "signature=",
            "x-amz-signature",
        ]
        .iter()
        .any(|marker| lowercase.contains(marker))
    {
        return None;
    }
    let value = value.chars().take(max_chars).collect::<String>();
    (!value.is_empty()).then_some(value)
}

fn sanitize_capture_message(value: Option<&str>) -> Option<String> {
    let mut value = clean_capture_text(value, 500)?;
    let lowercase = value.to_ascii_lowercase();
    if [
        "access_token",
        "refresh_token",
        "signature=",
        "x-amz-signature",
        "cookie",
        "password",
        "authorization",
        "api_key",
    ]
    .iter()
    .any(|marker| lowercase.contains(marker))
        || value.contains('?')
        || value.contains('#')
        || lowercase.contains("https://")
        || lowercase.contains("http://")
    {
        value = "O item não pôde ser capturado com segurança.".into();
    }
    Some(value)
}

fn valid_capture_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b":_-".contains(&byte))
}

fn valid_source_key(value: &str) -> bool {
    let Some((kind, digest)) = value.split_once(':') else {
        return false;
    };
    matches!(kind, "pdf" | "canvas_page")
        && digest.len() == 64
        && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn material_column_exists(connection: &Connection, column: &str) -> Result<bool, StorageError> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM pragma_table_info('materials') WHERE name=?1)",
            [column],
            |row| row.get::<_, i64>(0),
        )
        .map(|exists| exists != 0)
        .map_err(sql_error)
}

fn canonical_material_origin(raw: &str) -> Option<String> {
    let mut url = url::Url::parse(raw).ok()?;
    let _ = url.set_username("");
    let _ = url.set_password(None);
    url.set_query(None);
    url.set_fragment(None);
    Some(url.to_string())
}

fn legacy_material_source_key(
    id: &str,
    origin: &str,
    source_kind: &str,
) -> (String, String, Option<String>) {
    let canonical_origin = canonical_material_origin(origin);
    let canvas_file_id = (source_kind == "pdf")
        .then(|| canonical_origin.as_deref())
        .flatten()
        .and_then(|origin| {
            let url = url::Url::parse(origin).ok()?;
            let segments = url.path_segments()?.collect::<Vec<_>>();
            segments
                .windows(2)
                .find(|parts| parts[0] == "files")
                .map(|parts| parts[1].to_owned())
                .filter(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()))
        });
    let key = canonical_origin
        .as_deref()
        .and_then(|origin| {
            crate::canvas::content_key(canvas_file_id.as_deref(), origin, source_kind)
        })
        .unwrap_or_else(|| {
            let digest = Sha256::digest(format!("legacy-material\0{source_kind}\0{id}").as_bytes());
            format!("{source_kind}:{digest:x}")
        });
    (key, source_kind.to_owned(), canonical_origin)
}

fn migrate_capture_schema(connection: &Connection) -> Result<(), StorageError> {
    connection
        .execute_batch("BEGIN IMMEDIATE;")
        .map_err(sql_error)?;
    let migration = (|| {
        for (column, statement) in [
            (
                "source_key",
                "ALTER TABLE materials ADD COLUMN source_key TEXT NOT NULL DEFAULT '';",
            ),
            (
                "source_kind",
                "ALTER TABLE materials ADD COLUMN source_kind TEXT NOT NULL DEFAULT 'pdf';",
            ),
            (
                "source_etag",
                "ALTER TABLE materials ADD COLUMN source_etag TEXT;",
            ),
            (
                "source_last_modified",
                "ALTER TABLE materials ADD COLUMN source_last_modified TEXT;",
            ),
            (
                "source_validator_host",
                "ALTER TABLE materials ADD COLUMN source_validator_host TEXT;",
            ),
            (
                "text_sha256",
                "ALTER TABLE materials ADD COLUMN text_sha256 TEXT;",
            ),
        ] {
            if !material_column_exists(connection, column)? {
                connection.execute_batch(statement).map_err(sql_error)?;
            }
        }
        connection
            .execute_batch(
                "CREATE INDEX IF NOT EXISTS materials_source_version
                   ON materials(source_key, text_sha256, imported_at DESC);
                 CREATE TABLE IF NOT EXISTS material_associations (
                   material_id TEXT NOT NULL,
                   course_id TEXT NOT NULL,
                   course_title TEXT NOT NULL,
                   module_title TEXT NOT NULL,
                   item_title TEXT NOT NULL,
                   semester TEXT,
                   created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                   PRIMARY KEY(material_id,course_id,module_title,item_title)
                 );
                 CREATE INDEX IF NOT EXISTS material_associations_course
                   ON material_associations(course_id,semester,material_id);
                 CREATE TABLE IF NOT EXISTS capture_runs (
                   id TEXT PRIMARY KEY,
                   scope TEXT NOT NULL,
                   limits_snapshot TEXT NOT NULL,
                   status TEXT NOT NULL,
                   courses_total INTEGER NOT NULL DEFAULT 0,
                   courses_done INTEGER NOT NULL DEFAULT 0,
                   items_total INTEGER NOT NULL DEFAULT 0,
                   captured INTEGER NOT NULL DEFAULT 0,
                   reused INTEGER NOT NULL DEFAULT 0,
                   ignored INTEGER NOT NULL DEFAULT 0,
                   failed INTEGER NOT NULL DEFAULT 0,
                   needs_review INTEGER NOT NULL DEFAULT 0,
                   pending INTEGER NOT NULL DEFAULT 0,
                   current_course_id TEXT,
                   current_course_title TEXT,
                   current_module_title TEXT,
                   current_item_title TEXT,
                   warning TEXT,
                   started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                   updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                   finished_at TEXT
                 );
                 CREATE INDEX IF NOT EXISTS capture_runs_latest
                   ON capture_runs(status,updated_at DESC);
                 CREATE TABLE IF NOT EXISTS capture_items (
                   run_id TEXT NOT NULL,
                   position INTEGER NOT NULL DEFAULT 0,
                   reference_key TEXT NOT NULL,
                   content_key TEXT,
                   canvas_item_id TEXT,
                   course_id TEXT NOT NULL,
                   course_title TEXT NOT NULL,
                   module_title TEXT NOT NULL,
                   title TEXT NOT NULL,
                   source_kind TEXT NOT NULL,
                   source_url TEXT NOT NULL,
                   status TEXT NOT NULL,
                   retryable INTEGER NOT NULL DEFAULT 0,
                   attempts INTEGER NOT NULL DEFAULT 0,
                   material_id TEXT,
                   message TEXT,
                   updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                   PRIMARY KEY(run_id,reference_key)
                 );
                 CREATE INDEX IF NOT EXISTS capture_items_course_order
                   ON capture_items(run_id,course_id,position,reference_key);",
            )
            .map_err(sql_error)?;

        let materials = {
            let mut statement = connection
                .prepare("SELECT id,course_id,course_title,module_title,title,origin,semester,source_key FROM materials")
                .map_err(sql_error)?;
            let rows = statement
                .query_map([], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, String>(7)?,
                    ))
                })
                .map_err(sql_error)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)?
        };
        for (id, course_id, course_title, module_title, title, origin, semester, existing_key) in
            materials
        {
            let (derived_key, source_kind, canonical_origin) =
                legacy_material_source_key(&id, &origin, "pdf");
            let source_key = if existing_key.is_empty() {
                derived_key
            } else {
                existing_key
            };
            connection
                .execute(
                    "UPDATE materials SET source_key=?1,source_kind=COALESCE(NULLIF(source_kind,''),?2),
                       origin=COALESCE(?3,origin) WHERE id=?4",
                    params![source_key, source_kind, canonical_origin, id],
                )
                .map_err(sql_error)?;
            connection
                .execute(
                    "INSERT OR IGNORE INTO material_associations(
                       material_id,course_id,course_title,module_title,item_title,semester
                     ) VALUES(?1,?2,?3,?4,?5,?6)",
                    params![id, course_id, course_title, module_title, title, semester],
                )
                .map_err(sql_error)?;
        }
        connection
            .pragma_update(None, "user_version", 7)
            .map_err(sql_error)?;
        Ok::<(), StorageError>(())
    })();
    match migration {
        Ok(()) => connection.execute_batch("COMMIT;").map_err(sql_error),
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK;");
            Err(error)
        }
    }
}

fn backfill_missing_material_sources(connection: &Connection) -> Result<(), StorageError> {
    let materials = {
        let mut statement = connection
            .prepare(
                "SELECT id,course_id,course_title,module_title,title,origin,semester
                 FROM materials WHERE source_key='' OR source_key IS NULL",
            )
            .map_err(sql_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            })
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)?
    };
    for (id, course_id, course_title, module_title, title, origin, semester) in materials {
        let (source_key, source_kind, canonical_origin) =
            legacy_material_source_key(&id, &origin, "pdf");
        connection
            .execute(
                "UPDATE materials SET source_key=?1,source_kind=?2,origin=COALESCE(?3,origin) WHERE id=?4",
                params![source_key, source_kind, canonical_origin, id],
            )
            .map_err(sql_error)?;
        connection
            .execute(
                "INSERT OR IGNORE INTO material_associations(
                   material_id,course_id,course_title,module_title,item_title,semester
                 ) VALUES(?1,?2,?3,?4,?5,?6)",
                params![id, course_id, course_title, module_title, title, semester],
            )
            .map_err(sql_error)?;
    }
    Ok(())
}

pub struct AcademicStore {
    connection: Mutex<Connection>,
    library_path: Mutex<PathBuf>,
    database_path: Mutex<PathBuf>,
}

fn initialize_storage_schema(connection: &Connection) -> Result<(), StorageError> {
    let schema_version = connection
        .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
        .map_err(sql_error)?;
    if schema_version > CURRENT_SCHEMA_VERSION {
        return Err(StorageError::Sql(format!(
            "o banco está na versão {schema_version}, mais nova que a versão suportada {CURRENT_SCHEMA_VERSION}"
        )));
    }
    connection.execute_batch("PRAGMA journal_mode=WAL;
        CREATE TABLE IF NOT EXISTS courses (
          id TEXT PRIMARY KEY, title TEXT NOT NULL, url TEXT NOT NULL, kind TEXT NOT NULL, seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS materials (
          id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL, module_title TEXT NOT NULL,
          title TEXT NOT NULL, path TEXT NOT NULL, lesson_path TEXT NOT NULL DEFAULT '', origin TEXT NOT NULL, pages INTEGER NOT NULL,
          semester TEXT,
          sha256 TEXT NOT NULL, imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
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
        CREATE TABLE IF NOT EXISTS practice_questions (
          material_id TEXT NOT NULL, position INTEGER NOT NULL, prompt TEXT NOT NULL, answer TEXT NOT NULL, source_page INTEGER NOT NULL,
          PRIMARY KEY(material_id, position)
        );
        CREATE TABLE IF NOT EXISTS academic_events (
          id TEXT PRIMARY KEY, semester TEXT NOT NULL, course_id TEXT, course_title TEXT,
          title TEXT NOT NULL, starts_at TEXT NOT NULL, all_day INTEGER NOT NULL,
          source_kind TEXT NOT NULL, source_url TEXT NOT NULL, source_label TEXT NOT NULL,
          verified_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE INDEX IF NOT EXISTS academic_events_semester_date ON academic_events(semester, starts_at);
        CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);")
        .map_err(sql_error)?;
    crate::catalog::academic::initialize_schema(connection).map_err(StorageError::Sql)?;
    let materials_have_semester = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM pragma_table_info('materials') WHERE name='semester')",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(sql_error)?;
    if materials_have_semester == 0 {
        connection
            .execute("ALTER TABLE materials ADD COLUMN semester TEXT", [])
            .map_err(sql_error)?;
    }
    if schema_version < 5 {
        connection
            .execute_batch(
                "BEGIN IMMEDIATE;
                 CREATE TABLE IF NOT EXISTS lesson_interactions (
                   id TEXT PRIMARY KEY,
                   session_id TEXT NOT NULL,
                   activity_id TEXT NOT NULL,
                   section_index INTEGER NOT NULL,
                   kind TEXT NOT NULL,
                   prompt TEXT NOT NULL,
                   response TEXT NOT NULL,
                   assessment TEXT NOT NULL,
                   source_page INTEGER,
                   source_heading TEXT NOT NULL,
                   source_excerpt TEXT NOT NULL,
                   source_sha256 TEXT NOT NULL,
                   attempted_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                 );
                 CREATE INDEX IF NOT EXISTS lesson_interactions_session_order
                   ON lesson_interactions(session_id, attempted_at, id);
                 PRAGMA user_version=5;
                 COMMIT;",
            )
            .map_err(sql_error)?;
    }
    if schema_version < 6 {
        connection
            .execute_batch(
                "BEGIN IMMEDIATE;
                 CREATE TABLE IF NOT EXISTS study_summaries (
                   id TEXT PRIMARY KEY,
                   course_id TEXT NOT NULL,
                   course_title TEXT NOT NULL,
                   semester TEXT NOT NULL,
                   source_hash TEXT NOT NULL,
                   source_manifest_json TEXT NOT NULL,
                   short_text TEXT NOT NULL,
                   long_text TEXT NOT NULL,
                   model TEXT NOT NULL,
                   input_tokens INTEGER NOT NULL,
                   output_tokens INTEGER NOT NULL,
                   created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                 );
                 CREATE INDEX IF NOT EXISTS study_summaries_subject_latest
                   ON study_summaries(course_id, semester, created_at DESC);
                 PRAGMA user_version=6;
                 COMMIT;",
            )
            .map_err(sql_error)?;
    }
    if schema_version < 7 {
        migrate_capture_schema(connection)?;
    }
    Ok(())
}

impl AcademicStore {
    pub fn open(database_path: &Path, library_path: PathBuf) -> Result<Self, StorageError> {
        if let Some(parent) = database_path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| StorageError::Io(error.to_string()))?;
        }
        std::fs::create_dir_all(&library_path)
            .map_err(|error| StorageError::Io(error.to_string()))?;
        let connection = Connection::open(database_path).map_err(sql_error)?;
        initialize_storage_schema(&connection)?;
        let store = Self {
            connection: Mutex::new(connection),
            library_path: Mutex::new(library_path),
            database_path: Mutex::new(
                database_path
                    .canonicalize()
                    .unwrap_or_else(|_| database_path.to_path_buf()),
            ),
        };
        store.load_library_setting()?;
        Ok(store)
    }

    pub fn open_workspace(
        legacy_database_path: &Path,
        default_library_path: PathBuf,
    ) -> Result<Self, StorageError> {
        let library_path = legacy_library_path(legacy_database_path)
            .filter(|path| path.is_dir())
            .unwrap_or(default_library_path);
        let database_path = library_path.join(".estuda-uva.sqlite3");
        let store = Self::open(&database_path, library_path)?;
        store.import_legacy_database(legacy_database_path)?;
        store.load_library_setting()?;
        Ok(store)
    }

    fn import_legacy_database(&self, legacy_database_path: &Path) -> Result<(), StorageError> {
        if !legacy_database_path.is_file() {
            return Ok(());
        }
        if legacy_database_path.canonicalize().ok() == self.database_path()?.canonicalize().ok() {
            return Ok(());
        }
        if self.get_setting("__legacy_database_imported")?.as_deref() == Some("1") {
            return Ok(());
        }

        let legacy_database_path = legacy_database_path.to_string_lossy().into_owned();
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        connection
            .execute(
                "ATTACH DATABASE ?1 AS legacy_store",
                [legacy_database_path.as_str()],
            )
            .map_err(sql_error)?;
        let import_result = (|| {
            connection
                .execute_batch(
                    "BEGIN IMMEDIATE;
                     INSERT OR IGNORE INTO main.courses(id,title,url,kind,seen_at)
                       SELECT id,title,url,kind,seen_at FROM legacy_store.courses;
                     INSERT OR IGNORE INTO main.materials(id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,sha256,imported_at)
                       SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,sha256,imported_at FROM legacy_store.materials;
                     INSERT OR IGNORE INTO main.study_sessions(id,course_id,material_id,correct,total,percent,studied_at)
                       SELECT id,course_id,material_id,correct,total,percent,studied_at FROM legacy_store.study_sessions;
                     INSERT OR IGNORE INTO main.practice_questions(material_id,position,prompt,answer,source_page)
                       SELECT material_id,position,prompt,answer,source_page FROM legacy_store.practice_questions;
                     INSERT OR IGNORE INTO main.academic_events(id,semester,course_id,course_title,title,starts_at,all_day,source_kind,source_url,source_label,verified_at)
                       SELECT id,semester,course_id,course_title,title,starts_at,all_day,source_kind,source_url,source_label,verified_at FROM legacy_store.academic_events;
                     INSERT OR IGNORE INTO main.settings(key,value)
                       SELECT key,value FROM legacy_store.settings;
                     INSERT OR REPLACE INTO main.settings(key,value)
                       VALUES('__legacy_database_imported','1');",
                )
                .map_err(sql_error)?;
            backfill_missing_material_sources(&connection)?;
            connection
                .pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)
                .map_err(sql_error)?;
            connection.execute_batch("COMMIT;").map_err(sql_error)?;
            Ok(())
        })();
        if import_result.is_err() {
            let _ = connection.execute_batch("ROLLBACK");
        }
        let detach_result = connection.execute_batch("DETACH DATABASE legacy_store");
        import_result?;
        detach_result.map_err(sql_error)?;
        Ok(())
    }

    pub fn database_path(&self) -> Result<PathBuf, StorageError> {
        self.database_path
            .lock()
            .map(|path| path.clone())
            .map_err(|error| StorageError::Io(error.to_string()))
    }

    pub fn save_courses(&self, courses: &[Course]) -> Result<(), StorageError> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        for course in courses {
            let kind = match course.kind {
                CourseKind::Current => "current",
                CourseKind::Past => "past",
            };
            transaction.execute("INSERT INTO courses(id,title,url,kind,seen_at) VALUES(?1,?2,?3,?4,CURRENT_TIMESTAMP)
              ON CONFLICT(id) DO UPDATE SET title=excluded.title,url=excluded.url,kind=excluded.kind,seen_at=CURRENT_TIMESTAMP",
              params![course.id, course.title, course.url, kind]).map_err(sql_error)?;
        }
        transaction.commit().map_err(sql_error)
    }

    pub fn save_material(
        &self,
        material: &ImportedMaterial,
        hash: &str,
    ) -> Result<(), StorageError> {
        let source_kind = match material.source_kind.as_str() {
            "pdf" | "canvas_page" => material.source_kind.as_str(),
            _ => "pdf",
        };
        let (derived_key, _, canonical_origin) =
            legacy_material_source_key(&material.id, &material.origin, source_kind);
        let supplied_key = material.source_key.trim();
        let source_key = if valid_source_key(supplied_key) {
            supplied_key.to_owned()
        } else {
            derived_key
        };
        let origin = canonical_origin.unwrap_or_else(|| material.origin.clone());
        let source_validator_host = clean_validator_host(material.source_validator_host.as_deref());
        let source_etag = clean_validator(material.source_etag.as_deref());
        let source_last_modified = clean_validator(material.source_last_modified.as_deref());
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        transaction
            .execute(
                "INSERT INTO materials(
                   id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,sha256,semester,
                   source_key,source_kind,source_validator_host,source_etag,source_last_modified,text_sha256
                 ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
                 ON CONFLICT(id) DO UPDATE SET course_title=excluded.course_title,
                   module_title=excluded.module_title,title=excluded.title,path=excluded.path,
                   lesson_path=excluded.lesson_path,origin=excluded.origin,pages=excluded.pages,
                   sha256=excluded.sha256,semester=COALESCE(excluded.semester,materials.semester),
                   source_key=excluded.source_key,source_kind=excluded.source_kind,
                   source_validator_host=COALESCE(excluded.source_validator_host,materials.source_validator_host),
                   source_etag=COALESCE(excluded.source_etag,materials.source_etag),
                   source_last_modified=COALESCE(excluded.source_last_modified,materials.source_last_modified),
                   text_sha256=COALESCE(excluded.text_sha256,materials.text_sha256),imported_at=CURRENT_TIMESTAMP",
                params![
                    material.id,
                    material.course_id,
                    material.course_title,
                    material.module_title,
                    material.title,
                    material.path,
                    material.lesson_path,
                    origin,
                    material.pages as i64,
                    hash,
                    material.semester,
                    source_key,
                    source_kind,
                    source_validator_host,
                    source_etag,
                    source_last_modified,
                    material.text_sha256,
                ],
            )
            .map_err(sql_error)?;

        if material.associations.is_empty() {
            insert_material_association(
                &transaction,
                &MaterialAssociation {
                    material_id: material.id.clone(),
                    course_id: material.course_id.clone(),
                    course_title: material.course_title.clone(),
                    module_title: material.module_title.clone(),
                    item_title: material.title.clone(),
                    semester: material.semester.clone(),
                },
            )?;
        } else {
            for association in &material.associations {
                if association.material_id == material.id {
                    insert_material_association(&transaction, association)?;
                }
            }
        }
        transaction.commit().map_err(sql_error)
    }

    pub fn find_material_by_source_key_and_text_sha256(
        &self,
        source_key: &str,
        text_sha256: &str,
    ) -> Result<Option<ImportedMaterial>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let material = connection
            .query_row(
                "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,
                        semester,sha256,imported_at,source_key,source_kind,source_validator_host,source_etag,source_last_modified,text_sha256
                 FROM materials WHERE source_key=?1 AND text_sha256=?2
                 ORDER BY imported_at DESC,rowid DESC LIMIT 1",
                params![source_key, text_sha256],
                imported_material_from_row,
            )
            .optional()
            .map_err(sql_error)?;
        let Some(mut material) = material else {
            return Ok(None);
        };
        material.associations = list_material_associations(&connection, &material.id)?;
        Ok(Some(material))
    }

    pub fn find_latest_material_by_source_key(
        &self,
        source_key: &str,
    ) -> Result<Option<ImportedMaterial>, StorageError> {
        if !valid_source_key(source_key) {
            return Ok(None);
        }
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let material = connection
            .query_row(
                "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,
                        semester,sha256,imported_at,source_key,source_kind,source_validator_host,source_etag,source_last_modified,text_sha256
                 FROM materials WHERE source_key=?1 ORDER BY imported_at DESC,rowid DESC LIMIT 1",
                [source_key],
                imported_material_from_row,
            )
            .optional()
            .map_err(sql_error)?;
        let Some(mut material) = material else {
            return Ok(None);
        };
        material.associations = list_material_associations(&connection, &material.id)?;
        Ok(Some(material))
    }

    pub fn save_material_association(
        &self,
        association: &MaterialAssociation,
    ) -> Result<(), StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        insert_material_association(&connection, association)
    }

    pub fn create_capture_run(&self, run: &CaptureRun) -> Result<(), StorageError> {
        if run.id.is_empty()
            || run.id.len() > 128
            || !run
                .id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(StorageError::Sql(
                "identificador da captura inválido".into(),
            ));
        }
        run.limits_snapshot
            .validate()
            .map_err(|message| StorageError::Sql(message.into()))?;
        let scope = clean_capture_text(Some(&run.scope), 64)
            .filter(|scope| {
                scope
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte))
            })
            .ok_or_else(|| StorageError::Sql("escopo da captura inválido".into()))?;
        let limits_json = serde_json::to_string(&run.limits_snapshot)
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let current_course_id = run
            .current_course_id
            .as_deref()
            .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()));
        let status = run.status.as_db();
        let warning = sanitize_capture_message(run.warning.as_deref());
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        connection
            .execute(
                "INSERT INTO capture_runs(
                   id,scope,limits_snapshot,status,courses_total,courses_done,items_total,captured,reused,
                   ignored,failed,needs_review,pending,current_course_id,current_course_title,
                   current_module_title,current_item_title,warning,started_at,updated_at,finished_at
                 ) VALUES(?1,?2,?3,?4,?5,0,0,0,0,0,0,0,0,?6,?7,?8,?9,?10,
                   COALESCE(NULLIF(?11,''),CURRENT_TIMESTAMP),COALESCE(NULLIF(?12,''),CURRENT_TIMESTAMP),?13)",
                params![
                    run.id,
                    scope,
                    limits_json,
                    status,
                    run.counts.courses_total as i64,
                    current_course_id,
                    clean_capture_text(run.current_course_title.as_deref(), 250),
                    clean_capture_text(run.current_module_title.as_deref(), 250),
                    clean_capture_text(run.current_item_title.as_deref(), 250),
                    warning,
                    run.started_at,
                    run.updated_at,
                    run.finished_at,
                ],
            )
            .map_err(sql_error)?;
        Ok(())
    }

    pub fn get_capture_run(&self, run_id: &str) -> Result<Option<CaptureRun>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        connection
            .query_row(
                "SELECT id,scope,limits_snapshot,status,courses_total,courses_done,items_total,captured,
                        reused,ignored,failed,needs_review,pending,current_course_id,current_course_title,
                        current_module_title,current_item_title,warning,started_at,updated_at,finished_at
                 FROM capture_runs WHERE id=?1",
                [run_id],
                capture_run_from_row,
            )
            .optional()
            .map_err(sql_error)
    }

    pub fn latest_incomplete_capture(&self) -> Result<Option<CaptureRun>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        connection
            .query_row(
                "SELECT id,scope,limits_snapshot,status,courses_total,courses_done,items_total,captured,
                        reused,ignored,failed,needs_review,pending,current_course_id,current_course_title,
                        current_module_title,current_item_title,warning,started_at,updated_at,finished_at
                 FROM capture_runs WHERE status IN ('running','waiting_for_login','cancelled','partial','failed')
                 ORDER BY updated_at DESC,rowid DESC LIMIT 1",
                [],
                capture_run_from_row,
            )
            .optional()
            .map_err(sql_error)
    }

    pub fn latest_capture(&self) -> Result<Option<CaptureRun>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        connection
            .query_row(
                "SELECT id,scope,limits_snapshot,status,courses_total,courses_done,items_total,captured,
                        reused,ignored,failed,needs_review,pending,current_course_id,current_course_title,
                        current_module_title,current_item_title,warning,started_at,updated_at,finished_at
                 FROM capture_runs ORDER BY updated_at DESC,rowid DESC LIMIT 1",
                [],
                capture_run_from_row,
            )
            .optional()
            .map_err(sql_error)
    }

    pub fn update_capture_run(&self, run: &CaptureRun) -> Result<(), StorageError> {
        run.limits_snapshot
            .validate()
            .map_err(|message| StorageError::Sql(message.into()))?;
        let scope = clean_capture_text(Some(&run.scope), 64)
            .filter(|scope| {
                scope
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte))
            })
            .ok_or_else(|| StorageError::Sql("escopo da captura inválido".into()))?;
        let limits_json = serde_json::to_string(&run.limits_snapshot)
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let current_course_id = run
            .current_course_id
            .as_deref()
            .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()));
        let warning = sanitize_capture_message(run.warning.as_deref());
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        let updated = transaction
            .execute(
                "UPDATE capture_runs SET scope=?2,limits_snapshot=?3,status=?4,
                   current_course_id=?5,current_course_title=?6,current_module_title=?7,
                   current_item_title=?8,warning=?9,updated_at=CURRENT_TIMESTAMP,
                   finished_at=CASE WHEN ?4 IN ('completed','cancelled','partial','failed')
                     THEN COALESCE(?10,CURRENT_TIMESTAMP) ELSE NULL END
                 WHERE id=?1",
                params![
                    run.id,
                    scope,
                    limits_json,
                    run.status.as_db(),
                    current_course_id,
                    clean_capture_text(run.current_course_title.as_deref(), 250),
                    clean_capture_text(run.current_module_title.as_deref(), 250),
                    clean_capture_text(run.current_item_title.as_deref(), 250),
                    warning,
                    run.finished_at,
                ],
            )
            .map_err(sql_error)?;
        if updated == 0 {
            return Err(StorageError::Sql("a execução de captura não existe".into()));
        }
        let counts = query_capture_counts(&transaction, &run.id, run.counts.courses_total)?;
        transaction
            .execute(
                "UPDATE capture_runs SET courses_total=?2,courses_done=?3,items_total=?4,captured=?5,
                   reused=?6,ignored=?7,failed=?8,needs_review=?9,pending=?10 WHERE id=?1",
                params![
                    run.id,
                    counts.courses_total as i64,
                    counts.courses_done as i64,
                    counts.items_total as i64,
                    counts.captured as i64,
                    counts.reused as i64,
                    counts.ignored as i64,
                    counts.failed as i64,
                    counts.needs_review as i64,
                    counts.pending as i64,
                ],
            )
            .map_err(sql_error)?;
        transaction.commit().map_err(sql_error)
    }

    pub fn upsert_capture_item(
        &self,
        run_id: &str,
        item: &CaptureItem,
    ) -> Result<(), StorageError> {
        if !valid_capture_key(&item.reference_key)
            || item.course_id.is_empty()
            || !item.course_id.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(StorageError::Sql("referência de aula inválida".into()));
        }
        if !matches!(
            item.source_kind.as_str(),
            "pdf" | "canvas_page" | "unsupported"
        ) {
            return Err(StorageError::Sql("tipo de material inválido".into()));
        }
        let source_url = crate::canvas::sanitize_source_url(&item.source_url)
            .ok_or_else(|| StorageError::Sql("a origem do material não é segura".into()))?;
        let retryable = item.retryable
            && matches!(
                item.status,
                CaptureItemStatus::Pending | CaptureItemStatus::Failed
            );
        let message = sanitize_capture_message(item.message.as_deref());
        let content_key = item
            .content_key
            .as_deref()
            .filter(|key| valid_source_key(key));
        let canvas_item_id = item
            .canvas_item_id
            .as_deref()
            .filter(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()));
        let material_id = item
            .material_id
            .as_deref()
            .filter(|id| valid_capture_key(id));
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        transaction
            .execute(
                "INSERT INTO capture_items(
                   run_id,position,reference_key,content_key,canvas_item_id,course_id,course_title,module_title,title,
                   source_kind,source_url,status,retryable,attempts,material_id,message,updated_at
                 ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,CURRENT_TIMESTAMP)
                 ON CONFLICT(run_id,reference_key) DO UPDATE SET position=excluded.position,content_key=excluded.content_key,
                   canvas_item_id=excluded.canvas_item_id,course_id=excluded.course_id,
                   course_title=excluded.course_title,module_title=excluded.module_title,title=excluded.title,
                   source_kind=excluded.source_kind,source_url=excluded.source_url,status=excluded.status,
                   retryable=excluded.retryable,attempts=excluded.attempts,material_id=excluded.material_id,
                   message=excluded.message,updated_at=CURRENT_TIMESTAMP",
                params![
                    run_id,
                    item.position.min(i64::MAX as usize) as i64,
                    item.reference_key,
                    content_key,
                    canvas_item_id,
                    item.course_id,
                    clean_capture_text(Some(&item.course_title), 250).unwrap_or_default(),
                    clean_capture_text(Some(&item.module_title), 250).unwrap_or_default(),
                    clean_capture_text(Some(&item.title), 250).unwrap_or_default(),
                    item.source_kind,
                    source_url,
                    item.status.as_db(),
                    if retryable { 1_i64 } else { 0_i64 },
                    item.attempts.min(100_000) as i64,
                    material_id,
                    message,
                ],
            )
            .map_err(sql_error)?;
        let courses_total: i64 = transaction
            .query_row(
                "SELECT courses_total FROM capture_runs WHERE id=?1",
                [run_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(sql_error)?
            .ok_or_else(|| StorageError::Sql("a execução de captura não existe".into()))?;
        let counts = query_capture_counts(&transaction, run_id, courses_total.max(0) as usize)?;
        transaction
            .execute(
                "UPDATE capture_runs SET courses_done=?2,items_total=?3,captured=?4,reused=?5,
                   ignored=?6,failed=?7,needs_review=?8,pending=?9,updated_at=CURRENT_TIMESTAMP WHERE id=?1",
                params![
                    run_id,
                    counts.courses_done as i64,
                    counts.items_total as i64,
                    counts.captured as i64,
                    counts.reused as i64,
                    counts.ignored as i64,
                    counts.failed as i64,
                    counts.needs_review as i64,
                    counts.pending as i64,
                ],
            )
            .map_err(sql_error)?;
        transaction.commit().map_err(sql_error)
    }

    pub fn list_capture_items(
        &self,
        run_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<CaptureItem>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut statement = connection
            .prepare(
                "SELECT run_id,position,reference_key,content_key,canvas_item_id,course_id,course_title,module_title,
                        title,source_kind,source_url,status,retryable,attempts,material_id,message,updated_at
                 FROM capture_items WHERE run_id=?1 ORDER BY course_id,position,reference_key LIMIT ?2 OFFSET ?3",
            )
            .map_err(sql_error)?;
        let rows = statement
            .query_map(
                params![
                    run_id,
                    limit.clamp(1, 200) as i64,
                    offset.min(i64::MAX as usize) as i64
                ],
                capture_item_from_row,
            )
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn get_capture_limits(&self) -> Result<CanvasCaptureLimits, StorageError> {
        let stored = self.get_setting("canvas_capture_limits")?;
        let Some(stored) = stored else {
            return Ok(CanvasCaptureLimits::default());
        };
        let limits = serde_json::from_str::<CanvasCaptureLimits>(&stored).ok();
        Ok(limits
            .filter(|limits| limits.validate().is_ok())
            .unwrap_or_default())
    }

    pub fn save_capture_limits(&self, limits: &CanvasCaptureLimits) -> Result<(), StorageError> {
        limits
            .validate()
            .map_err(|message| StorageError::Sql(message.into()))?;
        let json =
            serde_json::to_string(limits).map_err(|error| StorageError::Sql(error.to_string()))?;
        self.set_setting("canvas_capture_limits", &json)
    }

    pub fn current_timestamp(&self) -> Result<String, StorageError> {
        self.connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .query_row("SELECT strftime('%Y-%m-%dT%H:%M:%fZ','now')", [], |row| {
                row.get::<_, String>(0)
            })
            .map_err(sql_error)
    }

    pub fn save_study_summary(&self, summary: &StudySummary) -> Result<(), StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        if summary.created_at.is_empty() {
            connection
                .execute(
                    "INSERT INTO study_summaries(
                       id,course_id,course_title,semester,source_hash,source_manifest_json,
                       short_text,long_text,model,input_tokens,output_tokens,created_at
                     ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,strftime('%Y-%m-%dT%H:%M:%fZ','now'))",
                    params![
                        summary.id,
                        summary.course_id,
                        summary.course_title,
                        summary.semester,
                        summary.source_hash,
                        summary.source_manifest_json,
                        summary.short_text,
                        summary.long_text,
                        summary.model,
                        i64::try_from(summary.input_tokens).unwrap_or(i64::MAX),
                        i64::try_from(summary.output_tokens).unwrap_or(i64::MAX),
                    ],
                )
                .map_err(sql_error)?;
            return Ok(());
        }
        connection
            .execute(
                "INSERT INTO study_summaries(
                   id,course_id,course_title,semester,source_hash,source_manifest_json,
                   short_text,long_text,model,input_tokens,output_tokens,created_at
                 ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
                params![
                    summary.id,
                    summary.course_id,
                    summary.course_title,
                    summary.semester,
                    summary.source_hash,
                    summary.source_manifest_json,
                    summary.short_text,
                    summary.long_text,
                    summary.model,
                    i64::try_from(summary.input_tokens).unwrap_or(i64::MAX),
                    i64::try_from(summary.output_tokens).unwrap_or(i64::MAX),
                    summary.created_at,
                ],
            )
            .map_err(sql_error)?;
        Ok(())
    }

    pub fn latest_study_summary(
        &self,
        course_id: &str,
        semester: &str,
    ) -> Result<Option<StudySummary>, StorageError> {
        self.connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .query_row(
                "SELECT id,course_id,course_title,semester,source_hash,source_manifest_json,
                        short_text,long_text,model,input_tokens,output_tokens,created_at
                 FROM study_summaries WHERE course_id=?1 AND semester=?2
                 ORDER BY created_at DESC, rowid DESC LIMIT 1",
                params![course_id, semester],
                study_summary_from_row,
            )
            .optional()
            .map_err(sql_error)
    }

    pub fn list_calendar_events(&self, semester: &str) -> Result<Vec<CalendarEvent>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut statement = connection.prepare("SELECT id,semester,course_id,course_title,title,starts_at,all_day,source_kind,source_url,source_label FROM academic_events WHERE semester=?1 ORDER BY starts_at,title")
            .map_err(sql_error)?;
        let rows = statement
            .query_map([semester], |row| {
                Ok(CalendarEvent {
                    id: row.get(0)?,
                    semester: row.get(1)?,
                    course_id: row.get(2)?,
                    course_title: row.get(3)?,
                    title: row.get(4)?,
                    starts_at: row.get(5)?,
                    all_day: row.get::<_, i64>(6)? != 0,
                    source_kind: row.get(7)?,
                    source_url: row.get(8)?,
                    source_label: row.get(9)?,
                })
            })
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn replace_course_calendar_events(
        &self,
        semester: &str,
        course_id: &str,
        events: &[CalendarEvent],
    ) -> Result<(), StorageError> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        transaction.execute("DELETE FROM academic_events WHERE semester=?1 AND course_id=?2 AND source_kind='canvas_assignment'", params![semester, course_id]).map_err(sql_error)?;
        for event in events {
            transaction.execute(
                "INSERT INTO academic_events(id,semester,course_id,course_title,title,starts_at,all_day,source_kind,source_url,source_label) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
                 ON CONFLICT(id) DO UPDATE SET semester=excluded.semester,course_id=excluded.course_id,course_title=excluded.course_title,title=excluded.title,starts_at=excluded.starts_at,all_day=excluded.all_day,source_kind=excluded.source_kind,source_url=excluded.source_url,source_label=excluded.source_label,verified_at=CURRENT_TIMESTAMP",
                params![event.id, event.semester, event.course_id, event.course_title, event.title, event.starts_at,
                    if event.all_day { 1_i64 } else { 0_i64 }, event.source_kind, event.source_url, event.source_label],
            ).map_err(sql_error)?;
        }
        transaction.commit().map_err(sql_error)
    }

    pub fn list_materials(&self) -> Result<Vec<ImportedMaterial>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut materials = {
            let mut statement = connection
                .prepare("SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,sha256,imported_at,source_key,source_kind,source_validator_host,source_etag,source_last_modified,text_sha256 FROM materials ORDER BY imported_at DESC")
                .map_err(sql_error)?;
            let rows = statement
                .query_map([], imported_material_from_row)
                .map_err(sql_error)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)?
        };
        for material in &mut materials {
            material.associations = list_material_associations(&connection, &material.id)?;
        }
        Ok(materials)
    }

    pub fn find_material(&self, id: &str) -> Result<Option<ImportedMaterial>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let material = connection
            .query_row(
                "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,sha256,imported_at,source_key,source_kind,source_validator_host,source_etag,source_last_modified,text_sha256
                 FROM materials WHERE id=?1",
                [id],
                imported_material_from_row,
            )
            .optional()
            .map_err(sql_error)?;
        let Some(mut material) = material else {
            return Ok(None);
        };
        material.associations = list_material_associations(&connection, &material.id)?;
        Ok(Some(material))
    }

    pub fn find_material_by_origin(
        &self,
        course_id: &str,
        origin: &str,
    ) -> Result<Option<ImportedMaterial>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let material = connection.query_row(
                "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,sha256,imported_at,source_key,source_kind,source_validator_host,source_etag,source_last_modified,text_sha256
                 FROM materials WHERE course_id=?1 AND origin=?2 ORDER BY imported_at DESC LIMIT 1",
                params![course_id, origin],
                imported_material_from_row,
            )
            .optional()
            .map_err(sql_error)?;
        let Some(mut material) = material else {
            return Ok(None);
        };
        material.associations = list_material_associations(&connection, &material.id)?;
        Ok(Some(material))
    }

    pub fn set_material_semester(&self, id: &str, semester: &str) -> Result<(), StorageError> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        let updated = transaction
            .execute(
                "UPDATE materials SET semester=?2 WHERE id=?1",
                params![id, semester],
            )
            .map_err(sql_error)?;
        if updated == 0 {
            return Err(StorageError::Sql("o material local não existe mais".into()));
        }
        transaction
            .execute(
                "UPDATE material_associations SET semester=?2 WHERE material_id=?1",
                params![id, semester],
            )
            .map_err(sql_error)?;
        transaction.commit().map_err(sql_error)
    }

    pub fn record_session(
        &self,
        id: &str,
        course_id: &str,
        material_id: &str,
        correct: u32,
        total: u32,
        percent: u8,
    ) -> Result<(), StorageError> {
        self.connection.lock().map_err(|error| StorageError::Sql(error.to_string()))?.execute(
          "INSERT INTO study_sessions(id,course_id,material_id,correct,total,percent) VALUES(?1,?2,?3,?4,?5,?6)",
          params![id, course_id, material_id, correct, total, percent]).map_err(sql_error)?;
        Ok(())
    }

    pub fn begin_lesson_session(
        &self,
        resource_path: &str,
        course_id: Option<&str>,
        course_title: &str,
        material_title: &str,
        section_count: usize,
    ) -> Result<LessonSession, StorageError> {
        if resource_path.is_empty() || resource_path.len() > 4096 {
            return Err(StorageError::Sql(
                "O identificador da aula é inválido.".into(),
            ));
        }
        if section_count == 0 || section_count > 100_000 {
            return Err(StorageError::Sql(
                "A quantidade de seções da aula é inválida.".into(),
            ));
        }
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        let existing = transaction
            .query_row(
                "SELECT id,resource_path,course_id,course_title,material_title,section_index,section_count,started_at,updated_at,completed_at
                 FROM lesson_sessions WHERE resource_path=?1 AND completed_at IS NULL ORDER BY updated_at DESC LIMIT 1",
                [resource_path],
                lesson_session_from_row,
            )
            .optional()
            .map_err(sql_error)?;
        let session = if let Some(mut session) = existing {
            transaction
                .execute(
                    "UPDATE lesson_sessions SET section_count=?2,updated_at=CURRENT_TIMESTAMP WHERE id=?1",
                    params![session.id, section_count as i64],
                )
                .map_err(sql_error)?;
            session.section_count = section_count;
            session.updated_at = transaction
                .query_row(
                    "SELECT updated_at FROM lesson_sessions WHERE id=?1",
                    [&session.id],
                    |row| row.get(0),
                )
                .map_err(sql_error)?;
            session
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            transaction
                .execute(
                    "INSERT INTO lesson_sessions(id,resource_path,course_id,course_title,material_title,section_count)
                     VALUES(?1,?2,?3,?4,?5,?6)",
                    params![id, resource_path, course_id, course_title, material_title, section_count as i64],
                )
                .map_err(sql_error)?;
            transaction
                .query_row(
                    "SELECT id,resource_path,course_id,course_title,material_title,section_index,section_count,started_at,updated_at,completed_at
                     FROM lesson_sessions WHERE id=?1",
                    [id],
                    lesson_session_from_row,
                )
                .map_err(sql_error)?
        };
        transaction.commit().map_err(sql_error)?;
        Ok(session)
    }

    pub fn save_lesson_progress(
        &self,
        session_id: &str,
        section_index: usize,
        section_count: usize,
        completed: bool,
    ) -> Result<LessonSession, StorageError> {
        if section_count == 0 || section_index >= section_count || section_count > 100_000 {
            return Err(StorageError::Sql("O progresso da aula é inválido.".into()));
        }
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let changed = connection
            .execute(
                "UPDATE lesson_sessions SET section_index=?2,section_count=?3,updated_at=CURRENT_TIMESTAMP,
                   completed_at=CASE WHEN ?4=1 THEN COALESCE(completed_at,CURRENT_TIMESTAMP) ELSE completed_at END
                 WHERE id=?1",
                params![session_id, section_index as i64, section_count as i64, if completed { 1_i64 } else { 0_i64 }],
            )
            .map_err(sql_error)?;
        if changed == 0 {
            return Err(StorageError::Sql(
                "A sessão de aula não existe mais.".into(),
            ));
        }
        connection
            .query_row(
                "SELECT id,resource_path,course_id,course_title,material_title,section_index,section_count,started_at,updated_at,completed_at
                 FROM lesson_sessions WHERE id=?1",
                [session_id],
                lesson_session_from_row,
            )
            .map_err(sql_error)
    }

    pub fn get_lesson_session(
        &self,
        session_id: &str,
    ) -> Result<Option<LessonSession>, StorageError> {
        if session_id.is_empty() || session_id.len() > 128 {
            return Err(StorageError::Sql(
                "O identificador da sessão é inválido.".into(),
            ));
        }
        self.connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .query_row(
                "SELECT id,resource_path,course_id,course_title,material_title,section_index,section_count,started_at,updated_at,completed_at
                 FROM lesson_sessions WHERE id=?1",
                [session_id],
                lesson_session_from_row,
            )
            .optional()
            .map_err(sql_error)
    }

    pub fn save_lesson_interaction(
        &self,
        session_id: &str,
        activity: &TutorActivity,
        response: &str,
        assessment: &TutorAssessment,
    ) -> Result<LessonInteraction, StorageError> {
        if session_id.is_empty()
            || session_id.len() > 128
            || activity.session_id != session_id
            || activity.id.is_empty()
            || activity.id.len() > 512
            || activity.prompt.is_empty()
            || activity.prompt.len() > 2_000
            || response.len() > 16_384
            || activity.source_heading.len() > 512
            || activity.source_excerpt.chars().count() > 600
            || !valid_sha256(&activity.source_sha256)
            || activity.source_page == Some(0)
        {
            return Err(StorageError::Sql(
                "A resposta ou a fonte da atividade excede os limites permitidos.".into(),
            ));
        }
        match (&assessment.status, assessment.correct) {
            (TutorAssessmentStatus::Correct, Some(true))
            | (TutorAssessmentStatus::SelfAssessed, None)
            | (TutorAssessmentStatus::Pending, None) => {}
            _ => {
                return Err(StorageError::Sql(
                    "A avaliação da atividade é inconsistente.".into(),
                ));
            }
        }

        let id = uuid::Uuid::new_v4().to_string();
        let assessment_json = serde_json::to_string(assessment)
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        let section_count = transaction
            .query_row(
                "SELECT section_count FROM lesson_sessions WHERE id=?1",
                [session_id],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(sql_error)?
            .ok_or_else(|| StorageError::Sql("A sessão de aula não existe.".into()))?;
        if activity.section_index >= section_count.max(0) as usize {
            return Err(StorageError::Sql(
                "A seção da atividade não pertence à sessão.".into(),
            ));
        }
        transaction
            .execute(
                "INSERT INTO lesson_interactions(
                   id,session_id,activity_id,section_index,kind,prompt,response,assessment,
                   source_page,source_heading,source_excerpt,source_sha256
                 ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
                params![
                    id,
                    session_id,
                    activity.id,
                    activity.section_index as i64,
                    activity.kind.as_str(),
                    activity.prompt,
                    response,
                    assessment_json,
                    activity.source_page.map(i64::from),
                    activity.source_heading,
                    activity.source_excerpt,
                    activity.source_sha256,
                ],
            )
            .map_err(sql_error)?;
        let interaction = transaction
            .query_row(
                "SELECT id,session_id,activity_id,section_index,kind,prompt,response,assessment,
                        source_page,source_heading,source_excerpt,source_sha256,attempted_at
                 FROM lesson_interactions WHERE id=?1",
                [&id],
                lesson_interaction_from_row,
            )
            .map_err(sql_error)?;
        transaction.commit().map_err(sql_error)?;
        Ok(interaction)
    }

    pub fn list_lesson_interactions(
        &self,
        session_id: &str,
    ) -> Result<Vec<LessonInteraction>, StorageError> {
        if session_id.is_empty() || session_id.len() > 128 {
            return Err(StorageError::Sql(
                "O identificador da sessão é inválido.".into(),
            ));
        }
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut statement = connection
            .prepare(
                "SELECT id,session_id,activity_id,section_index,kind,prompt,response,assessment,
                        source_page,source_heading,source_excerpt,source_sha256,attempted_at
                 FROM lesson_interactions WHERE session_id=?1 ORDER BY rowid ASC",
            )
            .map_err(sql_error)?;
        let rows = statement
            .query_map([session_id], lesson_interaction_from_row)
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn get_lesson_interaction(
        &self,
        session_id: &str,
        activity_id: &str,
        current_source_sha256: &str,
    ) -> Result<Option<LessonInteraction>, StorageError> {
        if session_id.is_empty()
            || session_id.len() > 128
            || activity_id.is_empty()
            || activity_id.len() > 512
            || !valid_sha256(current_source_sha256)
        {
            return Err(StorageError::Sql(
                "A referência de revisão é inválida.".into(),
            ));
        }
        let mut interaction = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .query_row(
                "SELECT id,session_id,activity_id,section_index,kind,prompt,response,assessment,
                        source_page,source_heading,source_excerpt,source_sha256,attempted_at
                 FROM lesson_interactions
                 WHERE session_id=?1 AND activity_id=?2 ORDER BY rowid DESC LIMIT 1",
                params![session_id, activity_id],
                lesson_interaction_from_row,
            )
            .optional()
            .map_err(sql_error)?;
        if let Some(interaction) = &mut interaction {
            interaction.source_changed = Some(interaction.source_sha256 != current_source_sha256);
        }
        Ok(interaction)
    }

    pub fn list_study_history(
        &self,
        course_title: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StudyHistoryEntry>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut statement = connection
            .prepare(
                "SELECT id,kind,course_id,course_title,material_id,resource_path,material_title,correct,total,percent,
                        section_index,section_count,started_at,occurred_at,completed_at
                 FROM (
                   SELECT id,'lesson' AS kind,course_id,course_title,NULL AS material_id,resource_path,material_title,
                          NULL AS correct,NULL AS total,NULL AS percent,section_index,section_count,started_at,updated_at AS occurred_at,completed_at
                   FROM lesson_sessions
                   UNION ALL
                   SELECT ss.id,'practice' AS kind,ss.course_id,m.course_title,ss.material_id,m.path,m.title,
                          ss.correct,ss.total,ss.percent,NULL AS section_index,NULL AS section_count,ss.studied_at,ss.studied_at,NULL AS completed_at
                   FROM study_sessions ss LEFT JOIN materials m ON m.id=ss.material_id
                 ) history
                 WHERE ?1 IS NULL OR lower(COALESCE(course_title,'')) LIKE '%' || lower(?1) || '%'
                 ORDER BY occurred_at DESC,id DESC LIMIT ?2",
            )
            .map_err(sql_error)?;
        let rows = statement
            .query_map(params![course_title, limit.clamp(1, 500) as i64], |row| {
                Ok(StudyHistoryEntry {
                    id: row.get(0)?,
                    kind: row.get(1)?,
                    course_id: row.get(2)?,
                    course_title: row.get(3)?,
                    material_id: row.get(4)?,
                    resource_path: row.get(5)?,
                    material_title: row
                        .get::<_, Option<String>>(6)?
                        .unwrap_or_else(|| "Material local".into()),
                    correct: row.get::<_, Option<i64>>(7)?.map(|value| value as u32),
                    total: row.get::<_, Option<i64>>(8)?.map(|value| value as u32),
                    percent: row
                        .get::<_, Option<i64>>(9)?
                        .map(|value| value.clamp(0, 100) as u8),
                    section_index: row
                        .get::<_, Option<i64>>(10)?
                        .map(|value| value.max(0) as usize),
                    section_count: row
                        .get::<_, Option<i64>>(11)?
                        .map(|value| value.max(0) as usize),
                    started_at: row.get(12)?,
                    occurred_at: row.get(13)?,
                    completed_at: row.get(14)?,
                })
            })
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn save_questions(
        &self,
        material_id: &str,
        questions: &[LessonQuestion],
    ) -> Result<(), StorageError> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let transaction = connection.transaction().map_err(sql_error)?;
        transaction
            .execute(
                "DELETE FROM practice_questions WHERE material_id=?1",
                [material_id],
            )
            .map_err(sql_error)?;
        for (position, question) in questions.iter().enumerate() {
            transaction.execute("INSERT INTO practice_questions(material_id,position,prompt,answer,source_page) VALUES(?1,?2,?3,?4,?5)",
                params![material_id, position as i64, question.prompt, question.answer, question.source_page as i64]).map_err(sql_error)?;
        }
        transaction.commit().map_err(sql_error)
    }

    pub fn load_questions(&self, material_id: &str) -> Result<Vec<LessonQuestion>, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let mut statement = connection.prepare("SELECT prompt,answer,source_page FROM practice_questions WHERE material_id=?1 ORDER BY position").map_err(sql_error)?;
        let rows = statement
            .query_map([material_id], |row| {
                Ok(LessonQuestion {
                    prompt: row.get(0)?,
                    answer: row.get(1)?,
                    source_page: row.get::<_, i64>(2)? as u32,
                })
            })
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn overview(&self) -> Result<Overview, StorageError> {
        let connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let current_courses = connection
            .query_row(
                "SELECT COUNT(*) FROM courses WHERE kind='current'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(sql_error)? as usize;
        let past_courses = connection
            .query_row(
                "SELECT COUNT(*) FROM courses WHERE kind='past'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(sql_error)? as usize;
        let materials = connection
            .query_row("SELECT COUNT(*) FROM materials", [], |row| {
                row.get::<_, i64>(0)
            })
            .map_err(sql_error)? as usize;
        let (sessions, percent) = connection
            .query_row(
                "SELECT COUNT(*), COALESCE(AVG(percent),0) FROM study_sessions",
                [],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?)),
            )
            .map_err(sql_error)?;
        let library_path = self
            .library_path
            .lock()
            .map_err(|error| StorageError::Io(error.to_string()))?
            .display()
            .to_string();
        Ok(Overview {
            current_courses,
            past_courses,
            materials,
            study_sessions: sessions as usize,
            accuracy_percent: percent.round().clamp(0.0, 100.0) as u8,
            library_path,
        })
    }

    pub fn library_path(&self) -> Result<PathBuf, StorageError> {
        self.library_path
            .lock()
            .map(|path| path.clone())
            .map_err(|error| StorageError::Io(error.to_string()))
    }

    pub fn set_library_path(&self, path: PathBuf) -> Result<(), StorageError> {
        std::fs::create_dir_all(&path).map_err(|error| StorageError::Io(error.to_string()))?;
        let path = path
            .canonicalize()
            .map_err(|error| StorageError::Io(error.to_string()))?;
        let destination_path = path.join(".estuda-uva.sqlite3");
        let mut active_connection = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?;
        let current_database_path = self.database_path()?;
        let destination_is_current = destination_path
            .canonicalize()
            .is_ok_and(|candidate| candidate == current_database_path);
        if !destination_is_current {
            let destination_exists = destination_path.is_file()
                && std::fs::metadata(&destination_path)
                    .map(|metadata| metadata.len() > 0)
                    .unwrap_or(false);
            let mut destination = Connection::open(&destination_path).map_err(sql_error)?;
            if !destination_exists {
                let backup_result =
                    Backup::new(&active_connection, &mut destination).and_then(|backup| {
                        backup.run_to_completion(64, std::time::Duration::from_millis(20), None)
                    });
                if let Err(error) = backup_result {
                    drop(destination);
                    let _ = std::fs::remove_file(&destination_path);
                    return Err(sql_error(error));
                }
            }
            initialize_storage_schema(&destination)?;
            destination
                .execute(
                    "INSERT INTO settings(key,value) VALUES('library_path',?1)
                     ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                    [path.to_string_lossy().as_ref()],
                )
                .map_err(sql_error)?;
            *active_connection = destination;
            *self
                .database_path
                .lock()
                .map_err(|error| StorageError::Io(error.to_string()))? = destination_path;
        } else {
            active_connection
                .execute(
                    "INSERT INTO settings(key,value) VALUES('library_path',?1)
                     ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                    [path.to_string_lossy().as_ref()],
                )
                .map_err(sql_error)?;
        }
        *self
            .library_path
            .lock()
            .map_err(|error| StorageError::Io(error.to_string()))? = path;
        Ok(())
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), StorageError> {
        self.connection.lock().map_err(|error| StorageError::Sql(error.to_string()))?.execute(
            "INSERT INTO settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![key, value]
        ).map_err(sql_error)?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, StorageError> {
        self.connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .query_row("SELECT value FROM settings WHERE key=?1", [key], |row| {
                row.get::<_, String>(0)
            })
            .optional()
            .map_err(sql_error)
    }

    fn load_library_setting(&self) -> Result<(), StorageError> {
        let setting = self.get_setting("library_path")?;
        if let Some(path) = setting {
            let path = PathBuf::from(path);
            std::fs::create_dir_all(&path).map_err(|error| StorageError::Io(error.to_string()))?;
            *self
                .library_path
                .lock()
                .map_err(|error| StorageError::Io(error.to_string()))? = path;
        }
        Ok(())
    }
}

fn legacy_library_path(database_path: &Path) -> Option<PathBuf> {
    if !database_path.is_file() {
        return None;
    }
    let connection =
        Connection::open_with_flags(database_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .ok()?;
    connection
        .query_row(
            "SELECT value FROM settings WHERE key='library_path'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .ok()?
        .map(PathBuf::from)
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn sql_error(error: rusqlite::Error) -> StorageError {
    StorageError::Sql(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tutor::{
        TutorActivity, TutorActivityKind, TutorAssessment, TutorAssessmentStatus,
        hash_section_markdown,
    };

    fn event(id: &str, semester: &str) -> CalendarEvent {
        CalendarEvent {
            id: id.into(),
            semester: semester.into(),
            course_id: Some("8765".into()),
            course_title: Some("Desenvolvimento Web".into()),
            title: format!("Entrega {semester}"),
            starts_at: "2026-12-10T23:59:00-03:00".into(),
            all_day: false,
            source_kind: "canvas_assignment".into(),
            source_url: "https://uva.instructure.com/courses/8765/assignments/20".into(),
            source_label: "Canvas · Desenvolvimento Web".into(),
        }
    }

    #[test]
    fn local_state_reopens_and_calendar_replacement_is_scoped_to_semester() {
        let temporary = tempfile::tempdir().unwrap();
        let database = temporary.path().join("app/estuda.sqlite3");
        let library = temporary.path().join("Faculdade");
        let alternate_library = temporary.path().join("Acervo escolhido");
        let store = AcademicStore::open(&database, library.clone()).unwrap();
        store
            .replace_course_calendar_events("2026.2", "8765", &[event("event-2026", "2026.2")])
            .unwrap();
        store
            .replace_course_calendar_events("2027.1", "8765", &[event("event-2027", "2027.1")])
            .unwrap();
        store
            .replace_course_calendar_events("2026.2", "8765", &[])
            .unwrap();
        assert!(store.list_calendar_events("2026.2").unwrap().is_empty());
        assert_eq!(store.list_calendar_events("2027.1").unwrap().len(), 1);
        store
            .record_session("session-1", "8765", "material-1", 3, 4, 75)
            .unwrap();
        store.set_library_path(alternate_library.clone()).unwrap();
        let alternate_library = alternate_library.canonicalize().unwrap();
        let workspace_database = alternate_library.join(".estuda-uva.sqlite3");
        assert_eq!(store.database_path().unwrap(), workspace_database);
        assert!(workspace_database.is_file());
        drop(store);

        let reopened = AcademicStore::open(&workspace_database, alternate_library.clone()).unwrap();
        assert_eq!(reopened.list_calendar_events("2027.1").unwrap().len(), 1);
        let overview = reopened.overview().unwrap();
        assert_eq!(overview.study_sessions, 1);
        assert_eq!(overview.accuracy_percent, 75);
        assert_eq!(
            overview.library_path,
            alternate_library.display().to_string()
        );
    }

    #[test]
    fn material_semester_is_persisted_and_unknown_does_not_erase_it() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            AcademicStore::open(&temporary.path().join("app/estuda.sqlite3"), library).unwrap();
        let material = ImportedMaterial {
            id: "material-1".into(),
            course_id: "course-1".into(),
            course_title: "Bases de Programação".into(),
            module_title: "Unidade 1".into(),
            title: "Introdução".into(),
            path: "materias/bases/introducao.md".into(),
            lesson_path: "materias/bases/AULA.md".into(),
            origin: "https://uva.instructure.com/courses/1/files/2".into(),
            pages: 3,
            semester: Some("2026.2".into()),
            imported_at: String::new(),
            ..ImportedMaterial::default()
        };

        store.save_material(&material, "hash").unwrap();
        assert_eq!(
            store
                .find_material("material-1")
                .unwrap()
                .unwrap()
                .semester
                .as_deref(),
            Some("2026.2")
        );
        assert_eq!(
            store.list_materials().unwrap()[0].semester.as_deref(),
            Some("2026.2")
        );

        let mut unknown_semester = material;
        unknown_semester.semester = None;
        store.save_material(&unknown_semester, "hash").unwrap();
        assert_eq!(
            store
                .find_material("material-1")
                .unwrap()
                .unwrap()
                .semester
                .as_deref(),
            Some("2026.2")
        );
    }

    #[test]
    fn selecting_a_workspace_with_an_existing_database_preserves_its_local_state() {
        let temporary = tempfile::tempdir().unwrap();
        let first_library = temporary.path().join("primeiro");
        let second_library = temporary.path().join("segundo");
        let first = AcademicStore::open(
            &first_library.join(".estuda-uva.sqlite3"),
            first_library.clone(),
        )
        .unwrap();
        first
            .record_session("session-first", "course", "material", 1, 1, 100)
            .unwrap();
        let second = AcademicStore::open(
            &second_library.join(".estuda-uva.sqlite3"),
            second_library.clone(),
        )
        .unwrap();
        second
            .record_session("session-second", "course", "material", 0, 1, 0)
            .unwrap();
        drop(second);

        first.set_library_path(second_library.clone()).unwrap();

        let overview = first.overview().unwrap();
        assert_eq!(overview.study_sessions, 1);
        assert_eq!(overview.accuracy_percent, 0);
        assert_eq!(
            first.database_path().unwrap(),
            second_library
                .canonicalize()
                .unwrap()
                .join(".estuda-uva.sqlite3")
        );
    }

    #[test]
    fn lesson_sessions_resume_until_completed_and_remain_distinct_from_practice() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            AcademicStore::open(&library.join(".estuda-uva.sqlite3"), library.clone()).unwrap();
        store
            .record_session("practice-1", "course-1", "material-1", 2, 3, 67)
            .unwrap();

        let first = store
            .begin_lesson_session("materias/aula.md", Some("course-1"), "Bases", "Aula", 3)
            .unwrap();
        let progress = store.save_lesson_progress(&first.id, 1, 3, false).unwrap();
        let resumed = store
            .begin_lesson_session("materias/aula.md", Some("course-1"), "Bases", "Aula", 3)
            .unwrap();

        assert_eq!(resumed.id, first.id);
        assert_eq!(resumed.section_index, 1);
        assert_eq!(resumed.section_index, progress.section_index);

        store.save_lesson_progress(&resumed.id, 2, 3, true).unwrap();
        let next_review = store
            .begin_lesson_session("materias/aula.md", Some("course-1"), "Bases", "Aula", 3)
            .unwrap();
        assert_ne!(next_review.id, resumed.id);

        let history = store.list_study_history(None, 200).unwrap();
        assert_eq!(history.len(), 3);
        assert!(history.iter().any(|item| item.id == "practice-1"
            && item.kind == "practice"
            && item.percent == Some(67)));
        assert!(history.iter().any(|item| item.id == resumed.id
            && item.kind == "lesson"
            && item.completed_at.is_some()));
        assert!(history.iter().any(|item| item.id == next_review.id
            && item.kind == "lesson"
            && item.completed_at.is_none()));
    }

    fn tutor_activity(session_id: &str, section_index: usize, markdown: &str) -> TutorActivity {
        TutorActivity {
            id: format!("{session_id}:{section_index}:activity"),
            session_id: session_id.into(),
            section_index,
            kind: TutorActivityKind::SelfAssessment,
            prompt: "Explique a ideia principal.".into(),
            reference_answer: None,
            source_page: Some(2),
            source_heading: "Página 2".into(),
            source_excerpt: markdown.chars().take(600).collect(),
            source_sha256: hash_section_markdown(markdown),
        }
    }

    fn tutor_assessment() -> TutorAssessment {
        TutorAssessment {
            status: TutorAssessmentStatus::SelfAssessed,
            correct: None,
        }
    }

    fn capture_run(id: &str) -> crate::canvas_capture::CaptureRun {
        use crate::canvas_capture::{
            CanvasCaptureLimits, CaptureCounts, CaptureRun, CaptureRunStatus,
        };

        CaptureRun {
            id: id.into(),
            scope: "all_visible".into(),
            limits_snapshot: CanvasCaptureLimits::default(),
            status: CaptureRunStatus::Running,
            counts: CaptureCounts {
                courses_total: 1,
                ..CaptureCounts::default()
            },
            current_course_id: None,
            current_course_title: None,
            current_module_title: None,
            current_item_title: None,
            warning: None,
            started_at: "2026-10-15T00:00:00Z".into(),
            updated_at: "2026-10-15T00:00:00Z".into(),
            finished_at: None,
        }
    }

    fn capture_item(
        reference_key: &str,
        status: crate::canvas_capture::CaptureItemStatus,
        source_url: &str,
    ) -> crate::canvas_capture::CaptureItem {
        use crate::canvas_capture::CaptureItem;

        CaptureItem {
            position: 0,
            reference_key: reference_key.into(),
            content_key: Some("pdf:shared-source".into()),
            canvas_item_id: Some("82".into()),
            course_id: "41".into(),
            course_title: "Bases de Programação".into(),
            module_title: "Unidade 1".into(),
            title: "Apostila".into(),
            source_kind: "pdf".into(),
            source_url: source_url.into(),
            status,
            retryable: status == crate::canvas_capture::CaptureItemStatus::Failed,
            attempts: 0,
            material_id: None,
            message: None,
            updated_at: "2026-10-15T00:00:00Z".into(),
        }
    }

    #[test]
    fn capture_schema_migrates_v6_to_v7_preserves_materials_and_adds_capture_tables() {
        let temporary = tempfile::tempdir().unwrap();
        let database = temporary.path().join("app/estuda.sqlite3");
        let library = temporary.path().join("Faculdade");
        std::fs::create_dir_all(database.parent().unwrap()).unwrap();
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE materials (
                   id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL,
                   module_title TEXT NOT NULL, title TEXT NOT NULL, path TEXT NOT NULL,
                   lesson_path TEXT NOT NULL DEFAULT '', origin TEXT NOT NULL, pages INTEGER NOT NULL,
                   semester TEXT, sha256 TEXT NOT NULL, imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                 );
                 INSERT INTO materials(id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,sha256)
                   VALUES('material-v6','41','Bases de Programação','Unidade 1','Apostila','materias/apostila.md','materias/AULA.md','https://uva.instructure.com/courses/41/files/82?download_frd=1#page=2',4,'2026.2','pdf-hash');
                 PRAGMA user_version=6;",
            )
            .unwrap();
        drop(connection);

        let store = AcademicStore::open(&database, library).unwrap();
        let connection = store.connection.lock().unwrap();
        let version = connection
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap();
        let material_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM materials WHERE id='material-v6'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let source_key: String = connection
            .query_row(
                "SELECT source_key FROM materials WHERE id='material-v6'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let association_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM material_associations WHERE material_id='material-v6'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let capture_tables: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('capture_runs','capture_items')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(version, 7);
        assert_eq!(material_count, 1);
        assert!(source_key.starts_with("pdf:"));
        assert_eq!(association_count, 1);
        assert_eq!(capture_tables, 2);
    }

    #[test]
    fn capture_run_counts_follow_item_state_changes() {
        use crate::canvas_capture::{CaptureItemStatus, CaptureRunStatus};

        let temporary = tempfile::tempdir().unwrap();
        let store = AcademicStore::open(
            &temporary.path().join("app/estuda.sqlite3"),
            temporary.path().join("Faculdade"),
        )
        .unwrap();
        let run = capture_run("run-counts");
        store.create_capture_run(&run).unwrap();
        let pending = capture_item(
            "ref-counts",
            CaptureItemStatus::Pending,
            "https://ead.uva.br/material.pdf",
        );
        store.upsert_capture_item(&run.id, &pending).unwrap();
        let current = store.get_capture_run(&run.id).unwrap().unwrap();
        assert_eq!(current.status, CaptureRunStatus::Running);
        assert_eq!(current.counts.pending, 1);
        assert_eq!(current.counts.courses_done, 0);

        let mut captured = pending;
        captured.status = CaptureItemStatus::Captured;
        captured.material_id = Some("material-1".into());
        store.upsert_capture_item(&run.id, &captured).unwrap();
        let current = store.get_capture_run(&run.id).unwrap().unwrap();
        assert_eq!(current.counts.captured, 1);
        assert_eq!(current.counts.pending, 0);
        assert_eq!(current.counts.courses_done, 1);
    }

    #[test]
    fn capture_latest_incomplete_run_is_resumable() {
        use crate::canvas_capture::CaptureRunStatus;

        let temporary = tempfile::tempdir().unwrap();
        let store = AcademicStore::open(
            &temporary.path().join("app/estuda.sqlite3"),
            temporary.path().join("Faculdade"),
        )
        .unwrap();
        let mut completed = capture_run("run-completed");
        completed.status = CaptureRunStatus::Completed;
        store.create_capture_run(&completed).unwrap();
        store.update_capture_run(&completed).unwrap();
        let mut interrupted = capture_run("run-interrupted");
        interrupted.status = CaptureRunStatus::Cancelled;
        store.create_capture_run(&interrupted).unwrap();
        store.update_capture_run(&interrupted).unwrap();

        assert_eq!(
            store.latest_incomplete_capture().unwrap().unwrap().id,
            "run-interrupted"
        );
    }

    #[test]
    fn capture_limits_default_and_reject_values_outside_bounds() {
        use crate::canvas_capture::CanvasCaptureLimits;

        let temporary = tempfile::tempdir().unwrap();
        let store = AcademicStore::open(
            &temporary.path().join("app/estuda.sqlite3"),
            temporary.path().join("Faculdade"),
        )
        .unwrap();
        let defaults = CanvasCaptureLimits::default();
        assert_eq!(defaults.max_course_pages, 100);
        assert_eq!(defaults.max_courses, 200);
        assert_eq!(defaults.max_items, 10_000);
        assert_eq!(defaults.max_pdf_bytes, 60 * 1024 * 1024);
        assert_eq!(defaults.max_redirects, 4);
        assert_eq!(defaults.item_timeout_seconds, 45);
        assert_eq!(store.get_capture_limits().unwrap(), defaults);
        store.save_capture_limits(&defaults).unwrap();

        let invalid = CanvasCaptureLimits {
            max_course_pages: 0,
            ..defaults.clone()
        };
        assert!(store.save_capture_limits(&invalid).is_err());
        assert_eq!(store.get_capture_limits().unwrap(), defaults);
    }

    #[test]
    fn repeated_content_adds_a_course_association_without_duplicate_material() {
        use crate::canvas_capture::MaterialAssociation;

        let temporary = tempfile::tempdir().unwrap();
        let store = AcademicStore::open(
            &temporary.path().join("app/estuda.sqlite3"),
            temporary.path().join("Faculdade"),
        )
        .unwrap();
        let source_key = crate::canvas::content_key(
            Some("82"),
            "https://uva.instructure.com/courses/41/files/82",
            "pdf",
        )
        .unwrap();
        let material = ImportedMaterial {
            id: "material-shared".into(),
            course_id: "41".into(),
            course_title: "Bases de Programação".into(),
            module_title: "Unidade 1".into(),
            title: "Apostila".into(),
            path: "materias/bases/apostila.md".into(),
            lesson_path: "materias/bases/AULA.md".into(),
            origin: "https://uva.instructure.com/courses/41/files/82".into(),
            pages: 4,
            sha256: "pdf-hash".into(),
            semester: Some("2026.2".into()),
            imported_at: String::new(),
            source_key: source_key.clone(),
            source_kind: "pdf".into(),
            source_validator_host: None,
            source_etag: None,
            source_last_modified: None,
            text_sha256: Some("same-text-hash".into()),
            associations: Vec::new(),
        };
        store.save_material(&material, "pdf-hash").unwrap();
        let reused = store
            .find_material_by_source_key_and_text_sha256(&source_key, "same-text-hash")
            .unwrap()
            .unwrap();
        store
            .save_material_association(&MaterialAssociation {
                material_id: reused.id,
                course_id: "42".into(),
                course_title: "Cálculo".into(),
                module_title: "Unidade 2".into(),
                item_title: "Apostila compartilhada".into(),
                semester: Some("2026.2".into()),
            })
            .unwrap();

        let materials = store.list_materials().unwrap();
        assert_eq!(materials.len(), 1);
        assert_eq!(materials[0].associations.len(), 2);
    }

    #[test]
    fn capture_item_source_url_drops_query_and_fragment_on_persist() {
        use crate::canvas_capture::{CaptureItemStatus, CaptureRunStatus};

        let temporary = tempfile::tempdir().unwrap();
        let store = AcademicStore::open(
            &temporary.path().join("app/estuda.sqlite3"),
            temporary.path().join("Faculdade"),
        )
        .unwrap();
        let run = capture_run("run-sanitized-url");
        store.create_capture_run(&run).unwrap();
        let item = capture_item(
            "ref-sanitized-url",
            CaptureItemStatus::Pending,
            "https://ead.uva.br/material.pdf?signature=temporary#page=4",
        );
        store.upsert_capture_item(&run.id, &item).unwrap();
        let stored = store
            .list_capture_items(&run.id, 0, 50)
            .unwrap()
            .pop()
            .unwrap();
        let current = store.get_capture_run(&run.id).unwrap().unwrap();

        assert_eq!(current.status, CaptureRunStatus::Running);
        assert_eq!(stored.source_url, "https://ead.uva.br/material.pdf");
        assert!(!stored.source_url.contains("signature"));
        assert!(!stored.source_url.contains("page=4"));
    }

    fn study_summary(id: &str, source_hash: &str) -> StudySummary {
        StudySummary {
            id: id.into(),
            course_id: "enrollment-1".into(),
            course_title: "Bases de Programação".into(),
            semester: "2026.2".into(),
            source_hash: source_hash.into(),
            source_manifest_json: "[]".into(),
            short_text: format!("Resumo curto {source_hash}"),
            long_text: format!("Resumo longo {source_hash}"),
            model: "claude-haiku-5-5".into(),
            input_tokens: 120,
            output_tokens: 900,
            created_at: "2026-10-14T12:00:00Z".into(),
        }
    }

    #[test]
    fn study_summary_schema_migrates_v4_to_v7_and_preserves_prior_generations() {
        let temporary = tempfile::tempdir().unwrap();
        let database = temporary.path().join("app/estuda.sqlite3");
        let library = temporary.path().join("Faculdade");
        std::fs::create_dir_all(database.parent().unwrap()).unwrap();
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE lesson_sessions (
                   id TEXT PRIMARY KEY, resource_path TEXT NOT NULL, course_id TEXT, course_title TEXT NOT NULL,
                   material_title TEXT NOT NULL, section_index INTEGER NOT NULL DEFAULT 0, section_count INTEGER NOT NULL,
                   started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                   completed_at TEXT
                 );
                 PRAGMA user_version=4;",
            )
            .unwrap();
        drop(connection);

        let store = AcademicStore::open(&database, library).unwrap();
        store
            .save_study_summary(&study_summary("summary-a", "source-a"))
            .unwrap();
        store
            .save_study_summary(&study_summary("summary-b", "source-b"))
            .unwrap();

        let connection = store.connection.lock().unwrap();
        let schema_version = connection
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap();
        let stored_row_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM study_summaries", [], |row| row.get(0))
            .unwrap();
        drop(connection);
        let latest = store
            .latest_study_summary("enrollment-1", "2026.2")
            .unwrap()
            .unwrap();

        assert_eq!(schema_version, 7);
        assert_eq!(stored_row_count, 2);
        assert_eq!(latest.source_hash, "source-b");
    }

    #[test]
    fn study_summary_schema_migrates_v5_to_v7() {
        let temporary = tempfile::tempdir().unwrap();
        let database = temporary.path().join("app/estuda.sqlite3");
        let library = temporary.path().join("Faculdade");
        std::fs::create_dir_all(database.parent().unwrap()).unwrap();
        let connection = Connection::open(&database).unwrap();
        connection.execute_batch("PRAGMA user_version=5;").unwrap();
        drop(connection);

        let store = AcademicStore::open(&database, library).unwrap();
        let schema_version = store
            .connection
            .lock()
            .unwrap()
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap();
        let table_exists: i64 = store.connection.lock().unwrap()
            .query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='study_summaries')", [], |row| row.get(0))
            .unwrap();

        assert_eq!(schema_version, 7);
        assert_eq!(table_exists, 1);
    }

    #[test]
    fn schema_v4_migrates_to_v7_without_losing_local_tables() {
        let temporary = tempfile::tempdir().unwrap();
        let database = temporary.path().join("app/estuda.sqlite3");
        let library = temporary.path().join("Faculdade");
        std::fs::create_dir_all(database.parent().unwrap()).unwrap();
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE lesson_sessions (
                   id TEXT PRIMARY KEY, resource_path TEXT NOT NULL, course_id TEXT, course_title TEXT NOT NULL,
                   material_title TEXT NOT NULL, section_index INTEGER NOT NULL DEFAULT 0, section_count INTEGER NOT NULL,
                   started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                   completed_at TEXT
                 );
                 INSERT INTO lesson_sessions(id,resource_path,course_title,material_title,section_count)
                   VALUES('kept-session','materias/aula.md','Bases','Aula',2);
                 PRAGMA user_version=4;",
            )
            .unwrap();
        drop(connection);

        let store = AcademicStore::open(&database, library).unwrap();

        let version = store
            .connection
            .lock()
            .unwrap()
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap();
        assert_eq!(version, 7);
        assert_eq!(
            store
                .get_lesson_session("kept-session")
                .unwrap()
                .unwrap()
                .id,
            "kept-session"
        );
        let table_exists: i64 = store.connection.lock().unwrap()
            .query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='lesson_interactions')", [], |row| row.get(0)).unwrap();
        assert_eq!(table_exists, 1);
        let summaries_exist: i64 = store.connection.lock().unwrap()
            .query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='study_summaries')", [], |row| row.get(0)).unwrap();
        assert_eq!(summaries_exist, 1);
    }

    #[test]
    fn lesson_interactions_keep_repeated_attempts_and_detect_changed_source() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            AcademicStore::open(&library.join(".estuda-uva.sqlite3"), library.clone()).unwrap();
        let session = store
            .begin_lesson_session("materias/aula.md", Some("course-1"), "Bases", "Aula", 2)
            .unwrap();
        let activity = tutor_activity(
            &session.id,
            1,
            "## Página 2\nEntropia é uma medida da incerteza.",
        );

        let first = store
            .save_lesson_interaction(
                &session.id,
                &activity,
                "A incerteza dos resultados",
                &tutor_assessment(),
            )
            .unwrap();
        let second = store
            .save_lesson_interaction(
                &session.id,
                &activity,
                "Mede a incerteza",
                &tutor_assessment(),
            )
            .unwrap();
        let attempts = store.list_lesson_interactions(&session.id).unwrap();
        let current_hash = hash_section_markdown("## Página 2\nConteúdo atualizado.");
        let review = store
            .get_lesson_interaction(&session.id, &activity.id, &current_hash)
            .unwrap()
            .unwrap();

        assert_ne!(first.id, second.id);
        assert_eq!(attempts.len(), 2);
        assert_eq!(attempts[0].response, "A incerteza dos resultados");
        assert_eq!(attempts[1].response, "Mede a incerteza");
        assert_eq!(attempts[0].source_sha256, activity.source_sha256);
        assert_eq!(review.source_changed, Some(true));
        assert!(
            store
                .get_lesson_interaction("outra-sessao", &activity.id, &current_hash)
                .unwrap()
                .is_none()
        );
        assert_eq!(
            store.get_lesson_session(&session.id).unwrap().unwrap().id,
            session.id
        );
    }

    #[test]
    fn lesson_interaction_rejects_unknown_sessions_mismatched_activities_and_oversized_answers() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            AcademicStore::open(&library.join(".estuda-uva.sqlite3"), library.clone()).unwrap();
        let session = store
            .begin_lesson_session("materias/aula.md", Some("course-1"), "Bases", "Aula", 1)
            .unwrap();
        let valid = tutor_activity(&session.id, 0, "Conteúdo.");
        let mismatched = tutor_activity("outra-sessao", 0, "Conteúdo.");
        let outside_range = tutor_activity(&session.id, 1, "Conteúdo.");

        assert!(
            store
                .save_lesson_interaction("nao-existe", &valid, "resposta", &tutor_assessment())
                .is_err()
        );
        assert!(
            store
                .save_lesson_interaction(&session.id, &mismatched, "resposta", &tutor_assessment())
                .is_err()
        );
        assert!(
            store
                .save_lesson_interaction(
                    &session.id,
                    &outside_range,
                    "resposta",
                    &tutor_assessment()
                )
                .is_err()
        );
        assert!(
            store
                .save_lesson_interaction(
                    &session.id,
                    &valid,
                    &"x".repeat(16_385),
                    &tutor_assessment()
                )
                .is_err()
        );
        assert!(
            store
                .list_lesson_interactions(&session.id)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn legacy_import_preserves_practice_history_and_current_schema_version() {
        let temporary = tempfile::tempdir().unwrap();
        let legacy_database = temporary.path().join("legacy.sqlite3");
        let connection = Connection::open(&legacy_database).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE courses(id TEXT PRIMARY KEY,title TEXT NOT NULL,url TEXT NOT NULL,kind TEXT NOT NULL,seen_at TEXT NOT NULL);
                 CREATE TABLE materials(id TEXT PRIMARY KEY,course_id TEXT NOT NULL,course_title TEXT NOT NULL,module_title TEXT NOT NULL,title TEXT NOT NULL,path TEXT NOT NULL,lesson_path TEXT NOT NULL,origin TEXT NOT NULL,pages INTEGER NOT NULL,sha256 TEXT NOT NULL,imported_at TEXT NOT NULL);
                 CREATE TABLE study_sessions(id TEXT PRIMARY KEY,course_id TEXT NOT NULL,material_id TEXT NOT NULL,correct INTEGER NOT NULL,total INTEGER NOT NULL,percent INTEGER NOT NULL,studied_at TEXT NOT NULL);
                 CREATE TABLE practice_questions(material_id TEXT NOT NULL,position INTEGER NOT NULL,prompt TEXT NOT NULL,answer TEXT NOT NULL,source_page INTEGER NOT NULL,PRIMARY KEY(material_id,position));
                 CREATE TABLE academic_events(id TEXT PRIMARY KEY,semester TEXT NOT NULL,course_id TEXT,course_title TEXT,title TEXT NOT NULL,starts_at TEXT NOT NULL,all_day INTEGER NOT NULL,source_kind TEXT NOT NULL,source_url TEXT NOT NULL,source_label TEXT NOT NULL,verified_at TEXT NOT NULL);
                 CREATE TABLE settings(key TEXT PRIMARY KEY,value TEXT NOT NULL);
                 INSERT INTO study_sessions(id,course_id,material_id,correct,total,percent,studied_at)
                   VALUES('legacy-practice','course-1','material-1',2,3,67,'2026-10-10 12:00:00');",
            )
            .unwrap();
        drop(connection);

        let library = temporary.path().join("Faculdade");
        let store = AcademicStore::open_workspace(&legacy_database, library).unwrap();

        let schema_version = store
            .connection
            .lock()
            .unwrap()
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap();
        assert_eq!(schema_version, CURRENT_SCHEMA_VERSION);
        let history = store.list_study_history(None, 100).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, "legacy-practice");
        assert_eq!(history[0].kind, "practice");
        assert_eq!(history[0].percent, Some(67));
    }

    #[test]
    fn schema_upgrade_from_v3_adds_academic_projection_without_losing_local_records() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        std::fs::create_dir_all(&library).unwrap();
        let database = library.join(".estuda-uva.sqlite3");
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE materials (
                   id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL, module_title TEXT NOT NULL,
                   title TEXT NOT NULL, path TEXT NOT NULL, lesson_path TEXT NOT NULL DEFAULT '', origin TEXT NOT NULL,
                   pages INTEGER NOT NULL, semester TEXT, sha256 TEXT NOT NULL, imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                 );
                 INSERT INTO materials(id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,sha256)
                   VALUES('material-before-v4','course-1','Bases','Unidade 1','Aula local','materias/aula.md','materias/AULA.md','local',1,'2026.1','hash');",
            )
            .unwrap();
        connection.pragma_update(None, "user_version", 3).unwrap();
        drop(connection);

        let store = AcademicStore::open(&database, library).unwrap();
        let connection = store.connection.lock().unwrap();
        let schema_version = connection
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .unwrap();
        let academic_tables: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN (
                   'academic_institutions','academic_programs','academic_curricula','academic_disciplines',
                   'academic_offerings','academic_discipline_offerings','academic_relations',
                   'academic_trajectories','academic_enrollments','academic_outcomes'
                 )",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let (material_count, title, semester): (i64, String, String) = connection
            .query_row(
                "SELECT COUNT(*),MAX(title),MAX(semester) FROM materials WHERE id='material-before-v4'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();

        assert_eq!(schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(academic_tables, 10);
        assert_eq!(material_count, 1);
        assert_eq!(title, "Aula local");
        assert_eq!(semester, "2026.1");
    }

    #[test]
    fn schema_upgrade_adds_material_semester_without_replacing_existing_materials() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        std::fs::create_dir_all(&library).unwrap();
        let database = library.join(".estuda-uva.sqlite3");
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE materials (
                   id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL, module_title TEXT NOT NULL,
                   title TEXT NOT NULL, path TEXT NOT NULL, lesson_path TEXT NOT NULL DEFAULT '', origin TEXT NOT NULL,
                   pages INTEGER NOT NULL, sha256 TEXT NOT NULL, imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                 );
                 INSERT INTO materials(id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,sha256)
                   VALUES('material-before-migration','course-1','Bases','Unidade 1','Aula local','materias/aula.md','materias/AULA.md','local',1,'hash');",
            )
            .unwrap();
        connection.pragma_update(None, "user_version", 2).unwrap();
        drop(connection);

        let store = AcademicStore::open(&database, library).unwrap();

        let connection = store.connection.lock().unwrap();
        let has_semester = connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('materials') WHERE name='semester')",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap();
        let (count, title): (i64, String) = connection
            .query_row(
                "SELECT COUNT(*),MAX(title) FROM materials WHERE id='material-before-migration'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(has_semester, 1);
        assert_eq!(count, 1);
        assert_eq!(title, "Aula local");
        assert_eq!(
            connection
                .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
                .unwrap(),
            CURRENT_SCHEMA_VERSION
        );
    }

    #[test]
    fn unsupported_destination_schema_is_left_untouched() {
        let temporary = tempfile::tempdir().unwrap();
        let source_library = temporary.path().join("origem");
        let destination = temporary.path().join("destino");
        let store = AcademicStore::open(
            &source_library.join(".estuda-uva.sqlite3"),
            source_library.clone(),
        )
        .unwrap();
        std::fs::create_dir_all(&destination).unwrap();
        let destination_database = destination.join(".estuda-uva.sqlite3");
        let connection = Connection::open(&destination_database).unwrap();
        connection.pragma_update(None, "user_version", 99).unwrap();
        drop(connection);
        let original_bytes = std::fs::read(&destination_database).unwrap();

        let error = store.set_library_path(destination).unwrap_err();

        assert!(error.to_string().contains("mais nova"));
        assert_eq!(std::fs::read(destination_database).unwrap(), original_bytes);
        assert_eq!(store.library_path().unwrap(), source_library);
    }
}
