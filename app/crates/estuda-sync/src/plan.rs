//! Reconciliação em memória, plano de persistência e execução retomável.
//!
//! **Trava de banco.** Este crate não abre banco. O resultado de uma sincronização é um
//! [`PersistencePlan`] *proposto*. Só [`apply`] o entrega a um [`PersistenceSink`], e só com
//! uma [`Approval`] ligada ao resumo (SHA-256) exato do plano: qualquer alteração no plano
//! invalida a aprovação. Sem aprovação, nada é escrito e o plano segue como proposta.
//!
//! Dados privados e públicos vão para lotes separados e nunca se misturam num mesmo lote.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::source::SourceRef;
use crate::text::sha256_hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Target {
    Private,
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposedChange {
    pub target: Target,
    pub entity: String,
    pub key: String,
    pub before_hash: Option<String>,
    pub after_hash: String,
    pub summary: String,
    pub source: SourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictRecord {
    pub target: Target,
    pub entity: String,
    pub key: String,
    pub local_hash: String,
    pub incoming_hash: String,
    pub base_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistencePlan {
    pub run_id: String,
    pub changes: Vec<ProposedChange>,
    /// Conflitos preservados para revisão humana; nenhuma versão é descartada.
    pub conflicts: Vec<ConflictRecord>,
    pub unchanged: usize,
    pub kept_local_edits: usize,
}

impl PersistencePlan {
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty() && self.conflicts.is_empty()
    }

    pub fn digest(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).expect("plano é sempre serializável"))
    }
}

// ---------------------------------------------------------------------------------------
// Reconciliação de três vias
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredState {
    /// Hash do que está hoje no armazenamento local.
    pub current_hash: String,
    /// Hash da última versão vinda da fonte que foi sincronizada (a base).
    pub last_synced_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reconciled {
    New,
    Same,
    /// Local intocado desde a última sincronização e a fonte mudou: seguro atualizar.
    Updated,
    /// Edição local e fonte inalterada: manter o local.
    KeepLocalEdit,
    /// Edição local **e** mudança na fonte, com resultados diferentes: preservar os dois.
    Conflict,
}

pub fn reconcile_item(existing: Option<&StoredState>, incoming_hash: &str) -> Reconciled {
    let Some(s) = existing else {
        return Reconciled::New;
    };
    if s.current_hash == incoming_hash {
        return Reconciled::Same;
    }
    let local_edited = s.current_hash != s.last_synced_hash;
    let source_changed = incoming_hash != s.last_synced_hash;
    match (local_edited, source_changed) {
        (false, true) => Reconciled::Updated,
        (true, false) => Reconciled::KeepLocalEdit,
        (true, true) => Reconciled::Conflict,
        (false, false) => Reconciled::Same,
    }
}

#[derive(Debug, Clone)]
pub struct PlanInput {
    pub target: Target,
    pub entity: String,
    pub key: String,
    pub existing: Option<StoredState>,
    pub incoming_hash: String,
    pub summary: String,
    pub source: SourceRef,
}

/// Idempotente: mesmas entradas (em qualquer ordem) ⇒ mesmo `run_id` e mesmo plano.
pub fn build_plan(inputs: &[PlanInput]) -> PersistencePlan {
    let mut sorted: Vec<&PlanInput> = inputs.iter().collect();
    sorted.sort_by(|a, b| {
        (a.target as u8, &a.entity, &a.key).cmp(&(b.target as u8, &b.entity, &b.key))
    });
    let run_id = {
        let mut seed = String::new();
        for i in &sorted {
            seed.push_str(&format!(
                "{:?}|{}|{}|{}\n",
                i.target, i.entity, i.key, i.incoming_hash
            ));
        }
        sha256_hex(seed.as_bytes())[..16].to_owned()
    };
    let mut plan = PersistencePlan {
        run_id,
        changes: vec![],
        conflicts: vec![],
        unchanged: 0,
        kept_local_edits: 0,
    };
    for i in sorted {
        match reconcile_item(i.existing.as_ref(), &i.incoming_hash) {
            Reconciled::New | Reconciled::Updated => plan.changes.push(ProposedChange {
                target: i.target,
                entity: i.entity.clone(),
                key: i.key.clone(),
                before_hash: i.existing.as_ref().map(|s| s.current_hash.clone()),
                after_hash: i.incoming_hash.clone(),
                summary: i.summary.clone(),
                source: i.source.clone(),
            }),
            Reconciled::Same => plan.unchanged += 1,
            Reconciled::KeepLocalEdit => plan.kept_local_edits += 1,
            Reconciled::Conflict => {
                let s = i
                    .existing
                    .as_ref()
                    .expect("conflito exige estado existente");
                plan.conflicts.push(ConflictRecord {
                    target: i.target,
                    entity: i.entity.clone(),
                    key: i.key.clone(),
                    local_hash: s.current_hash.clone(),
                    incoming_hash: i.incoming_hash.clone(),
                    base_hash: s.last_synced_hash.clone(),
                });
            }
        }
    }
    plan
}

// ---------------------------------------------------------------------------------------
// Aprovação e aplicação
// ---------------------------------------------------------------------------------------

/// Aprovação explícita do usuário para **um** plano. Só a camada de interface, depois de
/// mostrar o SQL/mudanças ao usuário e receber o "sim", deve chamar [`Approval::for_plan`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    plan_digest: String,
}

impl Approval {
    pub fn for_plan(plan: &PersistencePlan) -> Self {
        Self {
            plan_digest: plan.digest(),
        }
    }
}

pub trait PersistenceSink {
    /// Deve ser atômico por lote: aplica tudo ou nada. `changes` pertence a um único alvo.
    fn apply_batch(
        &mut self,
        target: Target,
        changes: &[&ProposedChange],
        conflicts: &[&ConflictRecord],
    ) -> Result<(), String>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ApplyError {
    #[error("persistência bloqueada: falta aprovação explícita do usuário")]
    NotAuthorized,
    #[error("a aprovação não corresponde a este plano (o plano mudou)")]
    ApprovalMismatch,
    #[error("falha ao aplicar lote {target:?}: {reason}")]
    Sink { target: Target, reason: String },
    #[error("lote privado aplicado, mas o público falhou: {reason}")]
    PartialAfterPrivate { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyReport {
    pub private_applied: usize,
    pub public_applied: usize,
    pub conflicts_recorded: usize,
}

pub fn apply<S: PersistenceSink>(
    plan: &PersistencePlan,
    approval: Option<&Approval>,
    sink: &mut S,
) -> Result<ApplyReport, ApplyError> {
    let approval = approval.ok_or(ApplyError::NotAuthorized)?;
    if approval.plan_digest != plan.digest() {
        return Err(ApplyError::ApprovalMismatch);
    }
    let by = |t: Target| -> (Vec<&ProposedChange>, Vec<&ConflictRecord>) {
        (
            plan.changes.iter().filter(|c| c.target == t).collect(),
            plan.conflicts.iter().filter(|c| c.target == t).collect(),
        )
    };
    let (priv_changes, priv_conflicts) = by(Target::Private);
    let (pub_changes, pub_conflicts) = by(Target::Public);
    let mut report = ApplyReport {
        private_applied: 0,
        public_applied: 0,
        conflicts_recorded: 0,
    };
    if !priv_changes.is_empty() || !priv_conflicts.is_empty() {
        sink.apply_batch(Target::Private, &priv_changes, &priv_conflicts)
            .map_err(|reason| ApplyError::Sink {
                target: Target::Private,
                reason,
            })?;
        report.private_applied = priv_changes.len();
        report.conflicts_recorded += priv_conflicts.len();
    }
    if !pub_changes.is_empty() || !pub_conflicts.is_empty() {
        let result = sink.apply_batch(Target::Public, &pub_changes, &pub_conflicts);
        match result {
            Ok(()) => {
                report.public_applied = pub_changes.len();
                report.conflicts_recorded += pub_conflicts.len();
            }
            Err(reason) if report.private_applied > 0 || report.conflicts_recorded > 0 => {
                return Err(ApplyError::PartialAfterPrivate { reason });
            }
            Err(reason) => {
                return Err(ApplyError::Sink {
                    target: Target::Public,
                    reason,
                });
            }
        }
    }
    Ok(report)
}

// ---------------------------------------------------------------------------------------
// Execução retomável, idempotente e cancelável
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Step {
    ValidateSession,
    IdentifyLink,
    DiscoverSources,
    ConsultCache,
    Compare,
    Fetch,
    Extract,
    Validate,
    Classify,
    Reconcile,
    BuildPlan,
    Apply,
    UpdateIndexes,
    Report,
}

pub const STEPS: [Step; 14] = [
    Step::ValidateSession,
    Step::IdentifyLink,
    Step::DiscoverSources,
    Step::ConsultCache,
    Step::Compare,
    Step::Fetch,
    Step::Extract,
    Step::Validate,
    Step::Classify,
    Step::Reconcile,
    Step::BuildPlan,
    Step::Apply,
    Step::UpdateIndexes,
    Step::Report,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Done,
    Skipped(String),
    /// Aguardando algo externo (ex.: aprovação do banco). Não é falha.
    Blocked(String),
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepResult {
    Done,
    Skip(String),
    Block(String),
    Fail(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    InProgress,
    Completed,
    /// Tudo certo, mas a persistência aguarda aprovação: resultado é só proposta.
    CompletedPendingApproval,
    Cancelled,
    Failed(Step),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncRun {
    pub id: String,
    pub statuses: Vec<(Step, StepStatus)>,
    pub cancelled: bool,
}

impl SyncRun {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            statuses: STEPS.iter().map(|s| (*s, StepStatus::Pending)).collect(),
            cancelled: false,
        }
    }

    pub fn request_cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn status(&self, step: Step) -> &StepStatus {
        &self
            .statuses
            .iter()
            .find(|(s, _)| *s == step)
            .expect("todas as etapas existem")
            .1
    }

    fn set(&mut self, step: Step, status: StepStatus) {
        if let Some(slot) = self.statuses.iter_mut().find(|(s, _)| *s == step) {
            slot.1 = status;
        }
    }

    pub fn next_pending(&self) -> Option<Step> {
        self.statuses
            .iter()
            .find(|(_, st)| *st == StepStatus::Pending)
            .map(|(s, _)| *s)
    }

    /// Retomada: limpa falhas e cancelamento; etapas concluídas não se repetem.
    pub fn resume(&mut self) {
        self.cancelled = false;
        for (_, st) in &mut self.statuses {
            if matches!(st, StepStatus::Failed(_)) {
                *st = StepStatus::Pending;
            }
        }
    }

    pub fn outcome(&self) -> RunOutcome {
        if let Some((s, _)) = self
            .statuses
            .iter()
            .find(|(_, st)| matches!(st, StepStatus::Failed(_)))
        {
            return RunOutcome::Failed(*s);
        }
        if self.next_pending().is_some() {
            return if self.cancelled {
                RunOutcome::Cancelled
            } else {
                RunOutcome::InProgress
            };
        }
        if matches!(self.status(Step::Apply), StepStatus::Blocked(_)) {
            RunOutcome::CompletedPendingApproval
        } else {
            RunOutcome::Completed
        }
    }
}

/// Executa etapas pendentes até concluir, falhar, bloquear a persistência ou cancelar.
/// O cancelamento vale entre etapas; o executor deve consultar o seu próprio `CancelToken`
/// dentro de etapas longas.
pub fn drive(run: &mut SyncRun, mut execute: impl FnMut(Step) -> StepResult) -> RunOutcome {
    while let Some(step) = run.next_pending() {
        if run.cancelled {
            break;
        }
        match execute(step) {
            StepResult::Done => run.set(step, StepStatus::Done),
            StepResult::Skip(why) => run.set(step, StepStatus::Skipped(why)),
            StepResult::Block(why) => {
                run.set(step, StepStatus::Blocked(why));
                if step == Step::Apply {
                    run.set(
                        Step::UpdateIndexes,
                        StepStatus::Skipped("plano não aplicado".into()),
                    );
                }
            }
            StepResult::Fail(why) => {
                run.set(step, StepStatus::Failed(why));
                break;
            }
        }
    }
    run.outcome()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceSystem;

    fn src() -> SourceRef {
        SourceRef::new(SourceSystem::Lyceum, None, "2026-11-06T10:00:00Z")
    }

    fn input(
        target: Target,
        key: &str,
        existing: Option<(&str, &str)>,
        incoming: &str,
    ) -> PlanInput {
        PlanInput {
            target,
            entity: "grade".into(),
            key: key.into(),
            existing: existing.map(|(c, l)| StoredState {
                current_hash: c.into(),
                last_synced_hash: l.into(),
            }),
            incoming_hash: incoming.into(),
            summary: format!("mudança em {key}"),
            source: src(),
        }
    }

    #[test]
    fn three_way_reconciliation() {
        let st = |c: &str, l: &str| StoredState {
            current_hash: c.into(),
            last_synced_hash: l.into(),
        };
        assert_eq!(reconcile_item(None, "x"), Reconciled::New);
        assert_eq!(reconcile_item(Some(&st("a", "a")), "a"), Reconciled::Same);
        assert_eq!(
            reconcile_item(Some(&st("a", "a")), "b"),
            Reconciled::Updated
        );
        assert_eq!(
            reconcile_item(Some(&st("edit", "a")), "a"),
            Reconciled::KeepLocalEdit
        );
        assert_eq!(
            reconcile_item(Some(&st("edit", "a")), "b"),
            Reconciled::Conflict
        );
        assert_eq!(
            reconcile_item(Some(&st("same", "a")), "same"),
            Reconciled::Same
        );
    }

    #[test]
    fn conflicts_keep_both_versions_and_never_overwrite() {
        let plan = build_plan(&[input(
            Target::Private,
            "k",
            Some(("minha-edicao", "base")),
            "da-fonte",
        )]);
        assert!(plan.changes.is_empty());
        assert_eq!(plan.conflicts.len(), 1);
        let c = &plan.conflicts[0];
        assert_eq!(
            (
                c.local_hash.as_str(),
                c.incoming_hash.as_str(),
                c.base_hash.as_str()
            ),
            ("minha-edicao", "da-fonte", "base")
        );
    }

    #[test]
    fn plan_is_idempotent_and_order_independent() {
        let a = input(Target::Private, "a", None, "h1");
        let b = input(Target::Public, "b", Some(("x", "x")), "h2");
        let p1 = build_plan(&[a.clone(), b.clone()]);
        let p2 = build_plan(&[b, a]);
        assert_eq!(p1, p2);
        assert_eq!(p1.digest(), p2.digest());
        assert_eq!(p1.changes.len(), 2);
    }

    #[derive(Default)]
    struct RecordingSink {
        batches: Vec<(Target, usize, usize)>,
        fail_on: Option<Target>,
    }
    impl PersistenceSink for RecordingSink {
        fn apply_batch(
            &mut self,
            target: Target,
            changes: &[&ProposedChange],
            conflicts: &[&ConflictRecord],
        ) -> Result<(), String> {
            if self.fail_on == Some(target) {
                return Err("disco cheio".into());
            }
            self.batches.push((target, changes.len(), conflicts.len()));
            Ok(())
        }
    }

    #[test]
    fn nothing_is_written_without_approval() {
        let plan = build_plan(&[input(Target::Private, "a", None, "h")]);
        let mut sink = RecordingSink::default();
        assert_eq!(
            apply(&plan, None, &mut sink),
            Err(ApplyError::NotAuthorized)
        );
        assert!(sink.batches.is_empty());
    }

    #[test]
    fn approval_is_bound_to_the_exact_plan() {
        let plan = build_plan(&[input(Target::Private, "a", None, "h")]);
        let approval = Approval::for_plan(&plan);
        let changed = build_plan(&[input(Target::Private, "a", None, "h-diferente")]);
        let mut sink = RecordingSink::default();
        assert_eq!(
            apply(&changed, Some(&approval), &mut sink),
            Err(ApplyError::ApprovalMismatch)
        );
        assert!(sink.batches.is_empty());
        assert!(apply(&plan, Some(&approval), &mut sink).is_ok());
    }

    #[test]
    fn private_and_public_go_in_separate_batches() {
        let plan = build_plan(&[
            input(Target::Private, "p", None, "h1"),
            input(Target::Public, "q", None, "h2"),
            input(Target::Private, "c", Some(("e", "b")), "s"),
        ]);
        let mut sink = RecordingSink::default();
        let report = apply(&plan, Some(&Approval::for_plan(&plan)), &mut sink).unwrap();
        assert_eq!(
            sink.batches,
            vec![(Target::Private, 1, 1), (Target::Public, 1, 0)]
        );
        assert_eq!(
            (
                report.private_applied,
                report.public_applied,
                report.conflicts_recorded
            ),
            (1, 1, 1)
        );
    }

    #[test]
    fn partial_failure_is_reported_honestly() {
        let plan = build_plan(&[
            input(Target::Private, "p", None, "h1"),
            input(Target::Public, "q", None, "h2"),
        ]);
        let mut sink = RecordingSink {
            fail_on: Some(Target::Public),
            ..Default::default()
        };
        let err = apply(&plan, Some(&Approval::for_plan(&plan)), &mut sink).unwrap_err();
        assert!(matches!(err, ApplyError::PartialAfterPrivate { .. }));
        let mut sink = RecordingSink {
            fail_on: Some(Target::Private),
            ..Default::default()
        };
        assert!(matches!(
            apply(&plan, Some(&Approval::for_plan(&plan)), &mut sink),
            Err(ApplyError::Sink {
                target: Target::Private,
                ..
            })
        ));
        assert!(sink.batches.is_empty());
    }

    #[test]
    fn blocked_persistence_leaves_a_proposal_not_an_error() {
        let mut run = SyncRun::new("r1");
        let outcome = drive(&mut run, |step| {
            if step == Step::Apply {
                StepResult::Block("aguardando aprovação do banco".into())
            } else {
                StepResult::Done
            }
        });
        assert_eq!(outcome, RunOutcome::CompletedPendingApproval);
        assert!(matches!(
            run.status(Step::UpdateIndexes),
            StepStatus::Skipped(_)
        ));
        assert_eq!(run.status(Step::Report), &StepStatus::Done);
    }

    #[test]
    fn partially_completed_run_resumes_without_repeating_done_steps() {
        let mut run = SyncRun::new("r2");
        let mut calls: Vec<Step> = vec![];
        let outcome = drive(&mut run, |s| {
            calls.push(s);
            if s == Step::Fetch {
                StepResult::Fail("rede".into())
            } else {
                StepResult::Done
            }
        });
        assert_eq!(outcome, RunOutcome::Failed(Step::Fetch));
        let done_before = calls.len();
        run.resume();
        let mut second: Vec<Step> = vec![];
        let outcome = drive(&mut run, |s| {
            second.push(s);
            StepResult::Done
        });
        assert_eq!(outcome, RunOutcome::Completed);
        assert_eq!(second.first(), Some(&Step::Fetch));
        assert!(second.len() == STEPS.len() - (done_before - 1));
        assert!(!second.contains(&Step::ValidateSession));
    }

    #[test]
    fn cancellation_stops_between_steps_and_resume_continues() {
        let mut run = SyncRun::new("r3");
        run.request_cancel();
        assert_eq!(
            drive(&mut run, |_| panic!("não deveria executar")),
            RunOutcome::Cancelled
        );
        run.resume();
        assert_eq!(drive(&mut run, |_| StepResult::Done), RunOutcome::Completed);
    }

    #[test]
    fn run_state_survives_serialization_for_resume() {
        let mut run = SyncRun::new("r4");
        drive(&mut run, |s| {
            if s == Step::Extract {
                StepResult::Fail("layout".into())
            } else {
                StepResult::Done
            }
        });
        let json = serde_json::to_string(&run).unwrap();
        let mut restored: SyncRun = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, run);
        restored.resume();
        assert_eq!(restored.next_pending(), Some(Step::Extract));
    }
}
