//! Estado epistêmico de um dado acadêmico.

/// O que se sabe sobre um valor. Estimativa nunca é promovida a confirmação sem fonte oficial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Confidence {
    /// Visto em fonte oficial (Portal/Lyceum), com fonte e data registradas.
    Confirmed,
    /// Calculado ou inferido localmente (ex.: nota final ponderada pelo app).
    Estimated,
    /// Ausente nas fontes. Representado como `None` com pendência, nunca como `0` ou `""`.
    Unknown,
    /// Fontes divergentes. Fica preservado para revisão humana; nenhuma versão vence sozinha.
    Conflicting,
}

impl Confidence {
    /// Só `Confirmed` pode alimentar cálculos que o aluno vá tratar como oficiais.
    pub fn is_official(self) -> bool {
        matches!(self, Confidence::Confirmed)
    }

    /// Combina duas observações do mesmo campo: iguais mantêm o estado; diferentes viram conflito.
    pub fn reconcile(self, other: Confidence, values_equal: bool) -> Confidence {
        match (self, other) {
            (Confidence::Unknown, x) | (x, Confidence::Unknown) => x,
            (a, b) if a == b && values_equal => a,
            (Confidence::Confirmed, Confidence::Estimated)
            | (Confidence::Estimated, Confidence::Confirmed)
                if values_equal =>
            {
                Confidence::Confirmed
            }
            _ => Confidence::Conflicting,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Confidence::*;

    #[test]
    fn only_confirmed_is_official() {
        assert!(Confirmed.is_official());
        assert!(![Estimated, Unknown, Conflicting].iter().any(|c| c.is_official()));
    }

    #[test]
    fn unknown_yields_to_any_observation() {
        assert_eq!(Unknown.reconcile(Estimated, false), Estimated);
        assert_eq!(Confirmed.reconcile(Unknown, false), Confirmed);
    }

    #[test]
    fn different_values_conflict_and_never_pick_a_winner() {
        assert_eq!(Confirmed.reconcile(Confirmed, false), Conflicting);
        assert_eq!(Confirmed.reconcile(Estimated, false), Conflicting);
    }

    #[test]
    fn equal_estimate_confirmed_by_official_source() {
        assert_eq!(Estimated.reconcile(Confirmed, true), Confirmed);
    }
}
