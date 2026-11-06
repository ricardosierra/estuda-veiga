//! Detecção de mudanças entre duas leituras e resumo local para notificação.
//!
//! Ausência numa leitura **não** é remoção: itens que sumiram são contados, não emitidos.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemKind {
    Activity,
    AcademicStatus,
    Exam,
    Material,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemSnapshot {
    pub kind: ItemKind,
    pub title: String,
    /// Hash do conteúdo normalizado; igual ⇒ sem alteração, sem olhar campos.
    pub content_hash: String,
    /// Campos relevantes: `due_at`, `grade`, `status`, `date`, `time`, `location`, `state`.
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Change {
    NewActivity {
        key: String,
        title: String,
    },
    DeadlineChanged {
        key: String,
        title: String,
        from: String,
        to: String,
    },
    GradePublished {
        key: String,
        title: String,
    },
    AcademicStatusChanged {
        key: String,
        title: String,
        from: String,
        to: String,
    },
    NewExam {
        key: String,
        title: String,
    },
    ExamChanged {
        key: String,
        title: String,
        field: String,
        from: String,
        to: String,
    },
    NewMaterial {
        key: String,
        title: String,
    },
    ContentChanged {
        key: String,
        title: String,
    },
    DataConflict {
        key: String,
        title: String,
    },
    SyncFailure {
        source: String,
        reason: String,
    },
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diff {
    pub changes: Vec<Change>,
    pub unchanged: usize,
    /// Presentes antes e ausentes agora: nunca viram remoção automática.
    pub missing_now: usize,
}

pub type Snapshot = BTreeMap<String, ItemSnapshot>;

pub fn detect(previous: &Snapshot, current: &Snapshot) -> Diff {
    let mut diff = Diff::default();
    for (key, now) in current {
        let k = key.clone();
        let title = now.title.clone();
        if now.fields.get("state").is_some_and(|s| s == "conflicting") {
            diff.changes.push(Change::DataConflict { key: k, title });
            continue;
        }
        let Some(before) = previous.get(key) else {
            diff.changes.push(match now.kind {
                ItemKind::Activity => Change::NewActivity { key: k, title },
                ItemKind::Exam => Change::NewExam { key: k, title },
                ItemKind::Material => Change::NewMaterial { key: k, title },
                ItemKind::AcademicStatus => Change::AcademicStatusChanged {
                    key: k,
                    title,
                    from: String::new(),
                    to: now.fields.get("status").cloned().unwrap_or_default(),
                },
            });
            continue;
        };
        if before.content_hash == now.content_hash {
            diff.unchanged += 1;
            continue;
        }
        let field = |s: &ItemSnapshot, name: &str| s.fields.get(name).cloned();
        let start = diff.changes.len();
        match now.kind {
            ItemKind::Activity => {
                if field(before, "due_at") != field(now, "due_at") {
                    diff.changes.push(Change::DeadlineChanged {
                        key: k.clone(),
                        title: title.clone(),
                        from: field(before, "due_at").unwrap_or_default(),
                        to: field(now, "due_at").unwrap_or_default(),
                    });
                }
                if field(before, "grade").is_none() && field(now, "grade").is_some() {
                    diff.changes.push(Change::GradePublished {
                        key: k.clone(),
                        title: title.clone(),
                    });
                }
            }
            ItemKind::AcademicStatus => {
                if field(before, "status") != field(now, "status") {
                    diff.changes.push(Change::AcademicStatusChanged {
                        key: k.clone(),
                        title: title.clone(),
                        from: field(before, "status").unwrap_or_default(),
                        to: field(now, "status").unwrap_or_default(),
                    });
                }
            }
            ItemKind::Exam => {
                for name in ["date", "time", "location"] {
                    if field(before, name) != field(now, name) {
                        diff.changes.push(Change::ExamChanged {
                            key: k.clone(),
                            title: title.clone(),
                            field: name.to_owned(),
                            from: field(before, name).unwrap_or_default(),
                            to: field(now, name).unwrap_or_default(),
                        });
                    }
                }
            }
            ItemKind::Material => {}
        }
        // Hash mudou, mas nenhuma regra específica explicou: ainda é mudança de conteúdo.
        if diff.changes.len() == start {
            diff.changes.push(Change::ContentChanged { key: k, title });
        }
    }
    diff.missing_now = previous
        .keys()
        .filter(|k| !current.contains_key(*k))
        .count();
    diff
}

/// Resumo em português para notificação local (dados privados; nunca sai da máquina).
pub fn summarize(changes: &[Change]) -> Vec<String> {
    changes
        .iter()
        .map(|c| match c {
            Change::NewActivity { title, .. } => format!("Nova atividade: {title}"),
            Change::DeadlineChanged {
                title, from, to, ..
            } => format!("Prazo alterado em {title}: {from} → {to}"),
            Change::GradePublished { title, .. } => format!("Nota publicada: {title}"),
            Change::AcademicStatusChanged {
                title, from, to, ..
            } => {
                format!("Situação acadêmica alterada em {title}: {from} → {to}")
            }
            Change::NewExam { title, .. } => format!("Nova prova disponível: {title}"),
            Change::ExamChanged {
                title,
                field,
                from,
                to,
                ..
            } => {
                format!("Prova {title}: {field} alterado de {from} para {to}")
            }
            Change::NewMaterial { title, .. } => format!("Novo material: {title}"),
            Change::ContentChanged { title, .. } => format!("Conteúdo alterado: {title}"),
            Change::DataConflict { title, .. } => format!("Dados conflitantes, revisar: {title}"),
            Change::SyncFailure { source, reason } => {
                format!("Falha ao sincronizar {source}: {reason}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(kind: ItemKind, title: &str, hash: &str, fields: &[(&str, &str)]) -> ItemSnapshot {
        ItemSnapshot {
            kind,
            title: title.into(),
            content_hash: hash.into(),
            fields: fields
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
        }
    }
    fn snap(items: Vec<(&str, ItemSnapshot)>) -> Snapshot {
        items.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
    }

    #[test]
    fn new_items_by_kind() {
        let now = snap(vec![
            ("a", item(ItemKind::Activity, "AVA 1", "h", &[])),
            ("e", item(ItemKind::Exam, "A2", "h", &[])),
            ("m", item(ItemKind::Material, "Aula 3", "h", &[])),
        ]);
        let d = detect(&Snapshot::new(), &now);
        assert!(d.changes.contains(&Change::NewActivity {
            key: "a".into(),
            title: "AVA 1".into()
        }));
        assert!(d.changes.contains(&Change::NewExam {
            key: "e".into(),
            title: "A2".into()
        }));
        assert!(d.changes.contains(&Change::NewMaterial {
            key: "m".into(),
            title: "Aula 3".into()
        }));
    }

    #[test]
    fn deadline_and_grade_changes() {
        let before = snap(vec![(
            "a",
            item(
                ItemKind::Activity,
                "AVA 1",
                "h1",
                &[("due_at", "2026-11-20")],
            ),
        )]);
        let after = snap(vec![(
            "a",
            item(
                ItemKind::Activity,
                "AVA 1",
                "h2",
                &[("due_at", "2026-11-27"), ("grade", "8,5")],
            ),
        )]);
        let d = detect(&before, &after);
        assert_eq!(d.changes.len(), 2);
        assert!(
            matches!(&d.changes[0], Change::DeadlineChanged { from, to, .. } if from == "2026-11-20" && to == "2026-11-27")
        );
        assert!(matches!(d.changes[1], Change::GradePublished { .. }));
    }

    #[test]
    fn unchanged_hash_is_skipped_and_missing_is_only_counted() {
        let before = snap(vec![
            ("a", item(ItemKind::Activity, "AVA 1", "h", &[])),
            ("gone", item(ItemKind::Material, "Velho", "x", &[])),
        ]);
        let after = snap(vec![("a", item(ItemKind::Activity, "AVA 1", "h", &[]))]);
        let d = detect(&before, &after);
        assert!(d.changes.is_empty());
        assert_eq!((d.unchanged, d.missing_now), (1, 1));
    }

    #[test]
    fn exam_field_changes_status_changes_and_conflicts() {
        let before = snap(vec![
            (
                "e",
                item(ItemKind::Exam, "A2", "1", &[("date", "2026-12-10")]),
            ),
            (
                "s",
                item(
                    ItemKind::AcademicStatus,
                    "Vínculo",
                    "1",
                    &[("status", "Ativo")],
                ),
            ),
            ("c", item(ItemKind::Exam, "A3", "1", &[])),
        ]);
        let after = snap(vec![
            (
                "e",
                item(ItemKind::Exam, "A2", "2", &[("date", "2026-12-12")]),
            ),
            (
                "s",
                item(
                    ItemKind::AcademicStatus,
                    "Vínculo",
                    "2",
                    &[("status", "Trancado")],
                ),
            ),
            (
                "c",
                item(ItemKind::Exam, "A3", "2", &[("state", "conflicting")]),
            ),
        ]);
        let d = detect(&before, &after);
        assert!(
            d.changes
                .iter()
                .any(|c| matches!(c, Change::ExamChanged { field, .. } if field == "date"))
        );
        assert!(
            d.changes
                .iter()
                .any(|c| matches!(c, Change::AcademicStatusChanged { to, .. } if to == "Trancado"))
        );
        assert!(
            d.changes
                .iter()
                .any(|c| matches!(c, Change::DataConflict { .. }))
        );
    }

    #[test]
    fn unexplained_hash_change_is_content_changed() {
        let before = snap(vec![("m", item(ItemKind::Material, "Aula", "1", &[]))]);
        let after = snap(vec![("m", item(ItemKind::Material, "Aula", "2", &[]))]);
        assert!(matches!(
            detect(&before, &after).changes[0],
            Change::ContentChanged { .. }
        ));
    }

    #[test]
    fn summaries_are_portuguese() {
        let s = summarize(&[Change::SyncFailure {
            source: "Canvas".into(),
            reason: "sessão expirada".into(),
        }]);
        assert_eq!(s, ["Falha ao sincronizar Canvas: sessão expirada"]);
    }
}
