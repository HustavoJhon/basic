//! BASIC — sistema personal de conocimiento y aprendizaje.
//!
//! F1 incluye configuración XDG, logging a archivo, CLI, migraciones SQLite y
//! el comando `basic doctor`. Markdown sigue siendo la fuente de verdad; este
//! paquete solo construye la infraestructura para indexarlo y estudiarlo.

pub mod cli;
pub mod config;
pub mod doctor;
pub mod error;
pub mod infra;
pub mod logging;
pub mod paths;

pub use error::BasicError;

/// Resultado con el error propio de BASIC.
pub type Result<T> = std::result::Result<T, BasicError>;
