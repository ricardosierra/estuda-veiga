//! Motor de leitura acadêmica do Estuda UVA.
//!
//! Regras estruturais deste crate:
//! - **Somente leitura** nos sistemas da universidade ([`policy`]).
//! - **Sem UI e sem SQLite.** Nada aqui abre banco; a persistência é uma *proposta*
//!   ([`plan`]) que só é entregue a um [`plan::PersistenceSink`] com aprovação vinculada
//!   ao resumo exato do plano.
//! - **Desconhecido não é zero.** Ausência de informação nunca vira "não entregue",
//!   "sem prova" ou "sem nota" ([`verification`], [`exam`], [`p4`]).
//! - **Privado por padrão.** Só um [`privacy::PublicRecord`] entra no cache/catálogo público.

pub mod changes;
pub mod crosscheck;
pub mod exam;
pub mod extract;
pub mod fetch;
pub mod link;
pub mod p4;
pub mod plan;
pub mod policy;
pub mod privacy;
pub mod reader;
pub mod source;
pub mod store;
pub mod text;
pub mod verification;
