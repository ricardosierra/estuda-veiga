use crate::calendar::CalendarEvent;
use crate::canvas::{Course, CourseKind};
use crate::lesson::LessonQuestion;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

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

pub struct AcademicStore {
    connection: Mutex<Connection>,
    library_path: Mutex<PathBuf>,
}

impl AcademicStore {
    pub fn open(database_path: &Path, library_path: PathBuf) -> Result<Self, StorageError> {
        if let Some(parent) = database_path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| StorageError::Io(error.to_string()))?;
        }
        std::fs::create_dir_all(&library_path)
            .map_err(|error| StorageError::Io(error.to_string()))?;
        let connection = Connection::open(database_path).map_err(sql_error)?;
        connection.execute_batch("PRAGMA journal_mode=WAL;
            CREATE TABLE IF NOT EXISTS courses (
              id TEXT PRIMARY KEY, title TEXT NOT NULL, url TEXT NOT NULL, kind TEXT NOT NULL, seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS materials (
              id TEXT PRIMARY KEY, course_id TEXT NOT NULL, course_title TEXT NOT NULL, module_title TEXT NOT NULL,
              title TEXT NOT NULL, path TEXT NOT NULL, lesson_path TEXT NOT NULL DEFAULT '', origin TEXT NOT NULL, pages INTEGER NOT NULL,
              sha256 TEXT NOT NULL, imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS study_sessions (
              id TEXT PRIMARY KEY, course_id TEXT NOT NULL, material_id TEXT NOT NULL,
              correct INTEGER NOT NULL, total INTEGER NOT NULL, percent INTEGER NOT NULL,
              studied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
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
        let store = Self {
            connection: Mutex::new(connection),
            library_path: Mutex::new(library_path),
        };
        store.load_library_setting()?;
        Ok(store)
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
          "INSERT INTO materials(id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,sha256) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
           ON CONFLICT(id) DO UPDATE SET course_title=excluded.course_title,module_title=excluded.module_title,title=excluded.title,path=excluded.path,lesson_path=excluded.lesson_path,origin=excluded.origin,pages=excluded.pages,sha256=excluded.sha256,imported_at=CURRENT_TIMESTAMP",
          params![material.id, material.course_id, material.course_title, material.module_title, material.title, material.path, material.lesson_path, material.origin, material.pages as i64, hash])
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
        let mut statement = connection.prepare("SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,imported_at FROM materials ORDER BY imported_at DESC")
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
                    imported_at: row.get(9)?,
                })
            })
            .map_err(sql_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(sql_error)
    }

    pub fn find_material(&self, id: &str) -> Result<Option<ImportedMaterial>, StorageError> {
        self.connection.lock().map_err(|error| StorageError::Sql(error.to_string()))?.query_row(
          "SELECT id,course_id,course_title,module_title,title,path,lesson_path,origin,pages,imported_at FROM materials WHERE id=?1", [id], |row| Ok(ImportedMaterial {
            id: row.get(0)?, course_id: row.get(1)?, course_title: row.get(2)?, module_title: row.get(3)?, title: row.get(4)?,
            path: row.get(5)?, lesson_path: row.get(6)?, origin: row.get(7)?, pages: row.get::<_, i64>(8)? as usize, imported_at: row.get(9)?,
          })).optional().map_err(sql_error)
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
        let value = path.to_string_lossy().to_string();
        self.set_setting("library_path", &value)?;
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

trait OptionalRow<T> {
    fn optional(self) -> Result<Option<T>, rusqlite::Error>;
}
impl<T> OptionalRow<T> for Result<T, rusqlite::Error> {
    fn optional(self) -> Result<Option<T>, rusqlite::Error> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error),
        }
    }
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
        drop(store);

        let reopened = AcademicStore::open(&database, library).unwrap();
        assert_eq!(reopened.list_calendar_events("2027.1").unwrap().len(), 1);
        let overview = reopened.overview().unwrap();
        assert_eq!(overview.study_sessions, 1);
        assert_eq!(overview.accuracy_percent, 75);
        assert_eq!(
            overview.library_path,
            alternate_library.display().to_string()
        );
    }
}
