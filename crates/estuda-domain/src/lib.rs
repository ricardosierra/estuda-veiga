//! Domínio do Estuda UVA, independente de interface e de persistência.
//!
//! Três responsabilidades, todas sem I/O:
//! - [`ids`]: identificadores estáveis e tipados (um `SubjectId` nunca é aceito onde se espera `LmsCourseId`);
//! - [`confidence`]: separação entre dado confirmado, estimado, desconhecido e conflitante;
//! - [`publication`]: fronteira entre camada privada e camada compartilhável, negando por padrão.

pub mod confidence;
pub mod ids;
pub mod publication;
