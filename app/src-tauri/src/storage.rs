use crate::calendar::CalendarEvent;
use crate::canvas::{Course, CourseKind};
use crate::lesson::LessonQuestion;
use rusqlite::{Connection, OptionalExtension, Row, backup::Backup, params};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

const CURRENT_SCHEMA_VERSION: i64 = 4;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("não foi possível preparar o banco local: {0}")]
    Io(String),
    #[error("não foi possível salvar os dados locais: {0}")]
    Sql(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default)]
    pub semester: Option<String>,
    pub imported_at: String,
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
    if schema_version < CURRENT_SCHEMA_VERSION {
        connection
            .pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)
            .map_err(sql_error)?;
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
                       VALUES('__legacy_database_imported','1');
                     COMMIT;",
                )
                .map_err(sql_error)?;
            connection
                .pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)
                .map_err(sql_error)?;
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
        self.connection.lock().map_err(|error| StorageError::Sql(error.to_string()))?.execute(
          "INSERT INTO materials(id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,sha256,semester) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
           ON CONFLICT(id) DO UPDATE SET course_title=excluded.course_title,module_title=excluded.module_title,title=excluded.title,path=excluded.path,lesson_path=excluded.lesson_path,origin=excluded.origin,pages=excluded.pages,sha256=excluded.sha256,semester=COALESCE(excluded.semester,materials.semester),imported_at=CURRENT_TIMESTAMP",
          params![material.id, material.course_id, material.course_title, material.module_title, material.title, material.path, material.lesson_path, material.origin, material.pages as i64, hash, material.semester])
          .map_err(sql_error)?;
        Ok(())
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
        let mut statement = connection.prepare("SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,imported_at FROM materials ORDER BY imported_at DESC")
            .map_err(sql_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(ImportedMaterial {
                    id: row.get(0)?,
                    course_id: row.get(1)?,
                    course_title: row.get(2)?,
                    module_title: row.get(3)?,
                    title: row.get(4)?,
                    path: row.get(5)?,
                    lesson_path: row.get(6)?,
                    origin: row.get(7)?,
                    pages: row.get::<_, i64>(8)? as usize,
                    semester: row.get(9)?,
                    imported_at: row.get(10)?,
                })
            })
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn find_material(&self, id: &str) -> Result<Option<ImportedMaterial>, StorageError> {
        self.connection.lock().map_err(|error| StorageError::Sql(error.to_string()))?.query_row(
          "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,imported_at FROM materials WHERE id=?1", [id], |row| Ok(ImportedMaterial {
            id: row.get(0)?, course_id: row.get(1)?, course_title: row.get(2)?, module_title: row.get(3)?, title: row.get(4)?,
            path: row.get(5)?, lesson_path: row.get(6)?, origin: row.get(7)?, pages: row.get::<_, i64>(8)? as usize, semester: row.get(9)?, imported_at: row.get(10)?,
          })).optional().map_err(sql_error)
    }

    pub fn find_material_by_origin(
        &self,
        course_id: &str,
        origin: &str,
    ) -> Result<Option<ImportedMaterial>, StorageError> {
        self.connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .query_row(
                "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,semester,imported_at
                 FROM materials WHERE course_id=?1 AND origin=?2 ORDER BY imported_at DESC LIMIT 1",
                params![course_id, origin],
                |row| {
                    Ok(ImportedMaterial {
                        id: row.get(0)?,
                        course_id: row.get(1)?,
                        course_title: row.get(2)?,
                        module_title: row.get(3)?,
                        title: row.get(4)?,
                        path: row.get(5)?,
                        lesson_path: row.get(6)?,
                        origin: row.get(7)?,
                        pages: row.get::<_, i64>(8)? as usize,
                        semester: row.get(9)?,
                        imported_at: row.get(10)?,
                    })
                },
            )
            .optional()
            .map_err(sql_error)
    }

    pub fn set_material_semester(&self, id: &str, semester: &str) -> Result<(), StorageError> {
        let updated = self
            .connection
            .lock()
            .map_err(|error| StorageError::Sql(error.to_string()))?
            .execute(
                "UPDATE materials SET semester=?2 WHERE id=?1",
                params![id, semester],
            )
            .map_err(sql_error)?;
        if updated == 0 {
            return Err(StorageError::Sql("o material local não existe mais".into()));
        }
        Ok(())
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

fn sql_error(error: rusqlite::Error) -> StorageError {
    StorageError::Sql(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
