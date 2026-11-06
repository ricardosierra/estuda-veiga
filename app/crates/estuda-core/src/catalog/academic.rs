use crate::sync::SyncEngine;
use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;
use serde_json::Value;

pub const CURRICULUM_PATHS: [&str; 2] = [
    "materias/catalogo/curriculos/uva-ciencia-computacao.json",
    "materias/catalogo/curriculos/uva-engenharia-computacao.json",
];
const DISCIPLINES_PATH: &str = "materias/catalogo/disciplinas.json";
const RELATIONS_PATH: &str = "materias/catalogo/relacoes.json";
const TRAJECTORIES_PATH: &str = "materias/catalogo/trajetorias.json";
const CATALOG_PATHS: [&str; 5] = [
    CURRICULUM_PATHS[0],
    CURRICULUM_PATHS[1],
    DISCIPLINES_PATH,
    RELATIONS_PATH,
    TRAJECTORIES_PATH,
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicCatalog {
    pub curriculums: Vec<Value>,
    pub disciplines: Vec<Value>,
    pub relations: Vec<Value>,
    pub trajectories: Vec<Value>,
}

pub fn initialize_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "PRAGMA foreign_keys=ON;
             CREATE TABLE IF NOT EXISTS academic_institutions (
               id TEXT PRIMARY KEY
             );
             CREATE TABLE IF NOT EXISTS academic_programs (
               id TEXT PRIMARY KEY,
               institution_id TEXT NOT NULL REFERENCES academic_institutions(id),
               title TEXT NOT NULL,
               code TEXT NOT NULL,
               source_json TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS academic_curricula (
               id TEXT PRIMARY KEY,
               program_id TEXT NOT NULL REFERENCES academic_programs(id),
               version TEXT NOT NULL,
               source_json TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS academic_disciplines (
               id TEXT PRIMARY KEY,
               title TEXT NOT NULL,
               identity_status TEXT,
               source_json TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS academic_offerings (
               id TEXT PRIMARY KEY,
               curriculum_id TEXT NOT NULL REFERENCES academic_curricula(id),
               canonical_discipline_id TEXT REFERENCES academic_disciplines(id),
               code TEXT NOT NULL,
               title TEXT NOT NULL,
               workload_hours INTEGER NOT NULL,
               status TEXT NOT NULL,
               source_json TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS academic_offerings_by_curriculum
               ON academic_offerings(curriculum_id, code);
             CREATE INDEX IF NOT EXISTS academic_offerings_by_discipline
               ON academic_offerings(canonical_discipline_id);
             CREATE TABLE IF NOT EXISTS academic_discipline_offerings (
               discipline_id TEXT NOT NULL REFERENCES academic_disciplines(id),
               offering_id TEXT NOT NULL REFERENCES academic_offerings(id),
               PRIMARY KEY(discipline_id, offering_id)
             );
             CREATE TABLE IF NOT EXISTS academic_relations (
               id TEXT PRIMARY KEY,
               relation_type TEXT NOT NULL,
               cc_offering_id TEXT REFERENCES academic_offerings(id),
               ec_offering_id TEXT REFERENCES academic_offerings(id),
               canonical_discipline_id TEXT REFERENCES academic_disciplines(id),
               equivalence_confirmed INTEGER NOT NULL CHECK(equivalence_confirmed IN (0,1)),
               source_json TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS academic_trajectories (
               id TEXT PRIMARY KEY,
               program_id TEXT NOT NULL REFERENCES academic_programs(id),
               curriculum_id TEXT REFERENCES academic_curricula(id),
               status TEXT NOT NULL,
               semester TEXT,
               source_json TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS academic_enrollments (
               trajectory_id TEXT NOT NULL REFERENCES academic_trajectories(id),
               id TEXT NOT NULL,
               title TEXT NOT NULL,
               section TEXT,
               start_date TEXT,
               status TEXT NOT NULL,
               semester TEXT,
               source_json TEXT NOT NULL,
               PRIMARY KEY(trajectory_id, id)
             );
             CREATE TABLE IF NOT EXISTS academic_outcomes (
               trajectory_id TEXT NOT NULL REFERENCES academic_trajectories(id),
               position INTEGER NOT NULL,
               course_title TEXT NOT NULL,
               status TEXT NOT NULL,
               source_json TEXT NOT NULL,
               PRIMARY KEY(trajectory_id, position)
             );",
        )
        .map_err(|error| error.to_string())
}

fn required_string(value: &Value, field: &str, entity: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("{entity} não contém o campo textual {field}."))
}

fn optional_string(value: &Value, field: &str) -> Option<String> {
    value.get(field).and_then(Value::as_str).map(str::to_owned)
}

fn json_text(value: &Value) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}

pub fn replace_projection(
    connection: &mut Connection,
    catalog: &AcademicCatalog,
) -> Result<(), String> {
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    transaction
        .execute_batch(
            "DELETE FROM academic_outcomes;
             DELETE FROM academic_enrollments;
             DELETE FROM academic_trajectories;
             DELETE FROM academic_relations;
             DELETE FROM academic_discipline_offerings;
             DELETE FROM academic_offerings;
             DELETE FROM academic_disciplines;
             DELETE FROM academic_curricula;
             DELETE FROM academic_programs;
             DELETE FROM academic_institutions;",
        )
        .map_err(|error| error.to_string())?;

    for curriculum in &catalog.curriculums {
        let entity = "Currículo";
        let institution_id = required_string(curriculum, "institutionId", entity)?;
        let program_id = required_string(curriculum, "programId", entity)?;
        let curriculum_id = required_string(curriculum, "id", entity)?;
        let title = required_string(curriculum, "programTitle", entity)?;
        let code = required_string(curriculum, "programCode", entity)?;
        let version = required_string(curriculum, "curriculumVersion", entity)?;
        transaction
            .execute(
                "INSERT OR IGNORE INTO academic_institutions(id) VALUES(?1)",
                [&institution_id],
            )
            .map_err(|error| error.to_string())?;
        transaction
            .execute(
                "INSERT INTO academic_programs(id,institution_id,title,code,source_json)
                 VALUES(?1,?2,?3,?4,?5)
                 ON CONFLICT(id) DO UPDATE SET institution_id=excluded.institution_id,
                   title=excluded.title,code=excluded.code,source_json=excluded.source_json",
                params![
                    program_id,
                    institution_id,
                    title,
                    code,
                    json_text(curriculum)?
                ],
            )
            .map_err(|error| error.to_string())?;
        transaction
            .execute(
                "INSERT INTO academic_curricula(id,program_id,version,source_json) VALUES(?1,?2,?3,?4)",
                params![curriculum_id, program_id, version, json_text(curriculum)?],
            )
            .map_err(|error| error.to_string())?;
    }

    for discipline in &catalog.disciplines {
        let id = required_string(discipline, "id", "Disciplina canônica")?;
        transaction
            .execute(
                "INSERT INTO academic_disciplines(id,title,identity_status,source_json) VALUES(?1,?2,?3,?4)",
                params![
                    id,
                    required_string(discipline, "title", "Disciplina canônica")?,
                    optional_string(discipline, "identityStatus"),
                    json_text(discipline)?,
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    for curriculum in &catalog.curriculums {
        let curriculum_id = required_string(curriculum, "id", "Currículo")?;
        let offerings = curriculum
            .get("offerings")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("O currículo {curriculum_id} não contém a lista offerings."))?;
        for offering in offerings {
            let offering_id = required_string(offering, "id", "Oferta curricular")?;
            let canonical_discipline_id = optional_string(offering, "canonicalDisciplineId");
            let workload_hours = offering
                .get("workloadHours")
                .and_then(Value::as_i64)
                .ok_or_else(|| {
                    format!("A oferta {offering_id} não contém workloadHours válido.")
                })?;
            transaction
                .execute(
                    "INSERT INTO academic_offerings(id,curriculum_id,canonical_discipline_id,code,title,workload_hours,status,source_json)
                     VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",
                    params![
                        offering_id,
                        curriculum_id,
                        canonical_discipline_id,
                        required_string(offering, "code", "Oferta curricular")?,
                        required_string(offering, "title", "Oferta curricular")?,
                        workload_hours,
                        required_string(offering, "status", "Oferta curricular")?,
                        json_text(offering)?,
                    ],
                )
                .map_err(|error| error.to_string())?;
        }
    }

    for discipline in &catalog.disciplines {
        let id = required_string(discipline, "id", "Disciplina canônica")?;
        let offering_ids = discipline
            .get("offeringIds")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("A disciplina {id} não contém offeringIds."))?;
        for offering_id in offering_ids {
            let offering_id = offering_id.as_str().ok_or_else(|| {
                format!("A disciplina {id} contém uma referência de oferta inválida.")
            })?;
            transaction
                .execute(
                    "INSERT INTO academic_discipline_offerings(discipline_id,offering_id) VALUES(?1,?2)",
                    params![id, offering_id],
                )
                .map_err(|error| error.to_string())?;
        }
    }

    for relation in &catalog.relations {
        let id = required_string(relation, "id", "Relação curricular")?;
        transaction
            .execute(
                "INSERT INTO academic_relations(id,relation_type,cc_offering_id,ec_offering_id,canonical_discipline_id,equivalence_confirmed,source_json)
                 VALUES(?1,?2,?3,?4,?5,?6,?7)",
                params![
                    id,
                    required_string(relation, "type", "Relação curricular")?,
                    optional_string(relation, "ccOfferingId"),
                    optional_string(relation, "ecOfferingId"),
                    optional_string(relation, "canonicalDisciplineId"),
                    i64::from(relation.get("equivalenceConfirmed").and_then(Value::as_bool).unwrap_or(false)),
                    json_text(relation)?,
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    for trajectory in &catalog.trajectories {
        let id = required_string(trajectory, "id", "Trajetória acadêmica")?;
        transaction
            .execute(
                "INSERT INTO academic_trajectories(id,program_id,curriculum_id,status,semester,source_json)
                 VALUES(?1,?2,?3,?4,?5,?6)",
                params![
                    id,
                    required_string(trajectory, "programId", "Trajetória acadêmica")?,
                    optional_string(trajectory, "curriculumId"),
                    required_string(trajectory, "status", "Trajetória acadêmica")?,
                    optional_string(trajectory, "semester"),
                    json_text(trajectory)?,
                ],
            )
            .map_err(|error| error.to_string())?;

        for enrollment in trajectory
            .get("currentEnrollments")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            transaction
                .execute(
                    "INSERT INTO academic_enrollments(trajectory_id,id,title,section,start_date,status,semester,source_json)
                     VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",
                    params![
                        id,
                        required_string(enrollment, "id", "Matrícula acadêmica")?,
                        required_string(enrollment, "title", "Matrícula acadêmica")?,
                        optional_string(enrollment, "section"),
                        optional_string(enrollment, "startDate"),
                        required_string(enrollment, "status", "Matrícula acadêmica")?,
                        optional_string(enrollment, "semester"),
                        json_text(enrollment)?,
                    ],
                )
                .map_err(|error| error.to_string())?;
        }

        for (position, outcome) in trajectory
            .get("outcomes")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            transaction
                .execute(
                    "INSERT INTO academic_outcomes(trajectory_id,position,course_title,status,source_json)
                     VALUES(?1,?2,?3,?4,?5)",
                    params![
                        id,
                        position as i64,
                        required_string(outcome, "courseTitle", "Resultado acadêmico")?,
                        required_string(outcome, "status", "Resultado acadêmico")?,
                        json_text(outcome)?,
                    ],
                )
                .map_err(|error| error.to_string())?;
        }
    }

    transaction.commit().map_err(|error| error.to_string())
}

pub fn read_projection(connection: &Connection) -> Result<AcademicCatalog, String> {
    fn read_rows(connection: &Connection, table: &str) -> Result<Vec<Value>, String> {
        let mut statement = connection
            .prepare(&format!("SELECT source_json FROM {table} ORDER BY rowid"))
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|error| error.to_string())?;
        rows.map(|row| {
            let text = row.map_err(|error| error.to_string())?;
            serde_json::from_str(&text).map_err(|error| error.to_string())
        })
        .collect()
    }

    Ok(AcademicCatalog {
        curriculums: read_rows(connection, "academic_curricula")?,
        disciplines: read_rows(connection, "academic_disciplines")?,
        relations: read_rows(connection, "academic_relations")?,
        trajectories: read_rows(connection, "academic_trajectories")?,
    })
}

pub fn load_documents(connection: &Connection) -> Result<Option<AcademicCatalog>, String> {
    fn read_document(connection: &Connection, path: &str) -> Result<Option<Value>, String> {
        let content = connection
            .query_row(
                "SELECT content FROM sync_documents WHERE relative_path=?1",
                [path],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| error.to_string())?;
        let Some(content) = content else {
            return Ok(None);
        };
        let document: Value = serde_json::from_str(&content)
            .map_err(|error| format!("JSON inválido em {path}: {error}"))?;
        if document.get("schemaVersion").and_then(Value::as_u64) != Some(1) {
            return Err(format!("Versão de catálogo não suportada em {path}."));
        }
        Ok(Some(document))
    }

    let mut documents = Vec::with_capacity(CATALOG_PATHS.len());
    let mut found = 0;
    for path in CATALOG_PATHS {
        let document = read_document(connection, path)?;
        found += usize::from(document.is_some());
        documents.push(document);
    }
    if found == 0 {
        return Ok(None);
    }
    if found != CATALOG_PATHS.len() {
        return Err(
            "O catálogo acadêmico está incompleto no SQLite; a projeção anterior foi preservada."
                .into(),
        );
    }

    let mut documents = documents.into_iter().map(Option::unwrap);
    let curriculums = CURRICULUM_PATHS
        .iter()
        .map(|_| {
            let curriculum = documents.next().expect("curriculum documents were counted");
            if curriculum
                .get("offerings")
                .and_then(Value::as_array)
                .is_none()
            {
                return Err("Um currículo não contém a lista offerings.".to_owned());
            }
            Ok(curriculum)
        })
        .collect::<Result<Vec<_>, String>>()?;

    fn array(document: Value, path: &str, key: &str) -> Result<Vec<Value>, String> {
        document
            .get(key)
            .and_then(Value::as_array)
            .cloned()
            .ok_or_else(|| format!("O catálogo {path} não contém a lista {key}."))
    }

    Ok(Some(AcademicCatalog {
        curriculums,
        disciplines: array(
            documents.next().expect("discipline document was counted"),
            DISCIPLINES_PATH,
            "disciplines",
        )?,
        relations: array(
            documents.next().expect("relation document was counted"),
            RELATIONS_PATH,
            "relations",
        )?,
        trajectories: array(
            documents.next().expect("trajectory document was counted"),
            TRAJECTORIES_PATH,
            "trajectories",
        )?,
    }))
}

pub fn load(sync: &SyncEngine) -> Result<AcademicCatalog, String> {
    sync.read_academic_catalog()
}

#[cfg(test)]
mod tests {
    use super::{CURRICULUM_PATHS, load};
    use crate::sync::SyncEngine;
    use serde_json::{Value, json};
    use std::{fs, path::Path};

    fn write_json(root: &Path, relative_path: &str, value: &Value) {
        let path = root.join(relative_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    }

    fn initial_documents(root: &Path) {
        write_json(
            root,
            "materias/catalogo/curriculos/uva-ciencia-computacao.json",
            &json!({
                "schemaVersion": 1,
                "id": "uva:curriculum:cc",
                "institutionId": "uva:institution:uva",
                "programId": "uva:program:cc",
                "programTitle": "Ciência da Computação",
                "programCode": "CC",
                "curriculumVersion": "2022",
                "offerings": [{
                    "id": "uva:offering:cc:calculo-numerico",
                    "code": "CC1",
                    "title": "Cálculo Numérico",
                    "workloadHours": 66,
                    "status": "pending",
                    "canonicalDisciplineId": "uva:discipline:calculo-numerico"
                }]
            }),
        );
        write_json(
            root,
            "materias/catalogo/curriculos/uva-engenharia-computacao.json",
            &json!({
                "schemaVersion": 1,
                "id": "uva:curriculum:ec",
                "institutionId": "uva:institution:uva",
                "programId": "uva:program:ec",
                "programTitle": "Engenharia de Computação",
                "programCode": "EC",
                "curriculumVersion": "2022",
                "offerings": [{
                    "id": "uva:offering:ec:calculo-numerico",
                    "code": "EC1",
                    "title": "Cálculo Numérico",
                    "workloadHours": 66,
                    "status": "pending",
                    "canonicalDisciplineId": "uva:discipline:calculo-numerico"
                }]
            }),
        );
        write_json(
            root,
            "materias/catalogo/disciplinas.json",
            &json!({
                "schemaVersion": 1,
                "disciplines": [{
                    "id": "uva:discipline:calculo-numerico",
                    "title": "Cálculo Numérico",
                    "offeringIds": ["uva:offering:cc:calculo-numerico", "uva:offering:ec:calculo-numerico"],
                    "identityStatus": "same_title_and_workload_candidate"
                }]
            }),
        );
        write_json(
            root,
            "materias/catalogo/relacoes.json",
            &json!({ "schemaVersion": 1, "relations": [] }),
        );
        write_json(
            root,
            "materias/catalogo/trajetorias.json",
            &json!({ "schemaVersion": 1, "trajectories": [] }),
        );
    }

    #[test]
    fn catalog_loads_the_imported_sqlite_documents_and_shared_disciplines() {
        let root = tempfile::tempdir().unwrap();
        initial_documents(root.path());
        let sync = SyncEngine::open(&root.path().join(".estuda-uva.sqlite3"), root.path()).unwrap();

        let report = sync.reconcile_all().unwrap();
        assert!(report.errors.is_empty(), "{:?}", report.errors);
        let imported = load(&sync).unwrap();

        assert_eq!(imported.curriculums.len(), 2);
        assert_eq!(imported.disciplines.len(), 1);
        assert_eq!(
            imported.disciplines[0]["offeringIds"],
            json!([
                "uva:offering:cc:calculo-numerico",
                "uva:offering:ec:calculo-numerico"
            ])
        );

        let mut edited_by_sqlite: Value =
            serde_json::from_slice(&fs::read(root.path().join(CURRICULUM_PATHS[0])).unwrap())
                .unwrap();
        edited_by_sqlite["programTitle"] = json!("Ciência da Computação editada");
        sync.set_content(
            CURRICULUM_PATHS[0],
            &serde_json::to_string_pretty(&edited_by_sqlite).unwrap(),
        )
        .unwrap();
        assert_eq!(
            load(&sync).unwrap().curriculums[0]["programTitle"],
            "Ciência da Computação"
        );
        sync.reconcile_all().unwrap();
        assert_eq!(
            load(&sync).unwrap().curriculums[0]["programTitle"],
            "Ciência da Computação editada"
        );

        let source: Value =
            serde_json::from_slice(&fs::read(root.path().join(CURRICULUM_PATHS[0])).unwrap())
                .unwrap();
        assert_eq!(source["programTitle"], "Ciência da Computação editada");
    }

    #[test]
    fn reconcile_materializes_shared_curriculum_entities_in_normalized_sqlite_tables() {
        let root = tempfile::tempdir().unwrap();
        initial_documents(root.path());
        let database_path = root.path().join(".estuda-uva.sqlite3");
        let sync = SyncEngine::open(&database_path, root.path()).unwrap();

        let report = sync.reconcile_all().unwrap();
        assert!(report.errors.is_empty(), "{:?}", report.errors);

        let connection = rusqlite::Connection::open(database_path).unwrap();
        let curriculum_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM academic_curricula", [], |row| {
                row.get(0)
            })
            .unwrap();
        let offering_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM academic_offerings", [], |row| {
                row.get(0)
            })
            .unwrap();
        let shared_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM (
                   SELECT d.id
                   FROM academic_disciplines d
                   JOIN academic_offerings o ON o.canonical_discipline_id=d.id
                   JOIN academic_curricula c ON c.id=o.curriculum_id
                   GROUP BY d.id
                   HAVING COUNT(DISTINCT c.program_id) > 1
                 )",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(curriculum_count, 2);
        assert_eq!(offering_count, 2);
        assert_eq!(shared_count, 1);
    }
}
