use std::path::PathBuf;

use thiserror::Error;

/// Errores tipados de la aplicación.
///
/// En el borde del binario se convierten a [`anyhow::Error`] para añadir
/// contexto; los módulos internos conservan estos tipos para poder probarlos.
#[derive(Debug, Error)]
pub enum BasicError {
    #[error("no se pudo determinar el directorio personal del usuario; define HOME")]
    NoHome,

    #[error("no se pudo leer {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("no se pudo escribir {path}: {source}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("no se pudo crear el directorio {path}: {source}")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("configuración inválida en {path}: {message}")]
    InvalidConfig { path: PathBuf, message: String },

    #[error("{path} no es TOML válido: {source}")]
    ParseConfig {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("no se pudo serializar la configuración: {0}")]
    SerializeConfig(#[from] toml::ser::Error),

    #[error(
        "la base de datos usa el esquema v{found}, pero esta versión de basic solo entiende hasta v{supported}"
    )]
    DatabaseTooNew { found: u32, supported: u32 },

    #[error(
        "secuencia de migraciones rota: se esperaba la versión {expected} y se encontró {found}"
    )]
    MigrationSequence { expected: u32, found: u32 },

    #[error("la migración {version} ({name}) falló: {source}")]
    Migration {
        version: u32,
        name: &'static str,
        #[source]
        source: rusqlite::Error,
    },

    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
