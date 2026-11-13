use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tracing_subscriber::EnvFilter;

use crate::error::BasicError;

/// Tamaño a partir del cual el log se rota al arrancar.
pub const MAX_LOG_BYTES: u64 = 10 * 1024 * 1024;

/// Inicializa el logging a archivo.
///
/// La salida estándar se reserva para la CLI y la TUI; los logs nunca deben
/// contaminar `--json`. Si ya hay un subscriber global (por ejemplo en pruebas)
/// la función no falla.
pub fn init(log_file: &Path) -> Result<(), BasicError> {
    if let Some(parent) = parent_dir(log_file) {
        fs::create_dir_all(parent).map_err(|source| BasicError::CreateDir {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    rotate_if_needed(log_file, MAX_LOG_BYTES)?;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .map_err(|source| BasicError::WriteFile {
            path: log_file.to_path_buf(),
            source,
        })?;

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("basic=info,warn"));

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .with_target(true)
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_writer(Mutex::new(file))
        .finish();

    // El único error posible es que ya exista un subscriber global.
    let _ = tracing::subscriber::set_global_default(subscriber);
    Ok(())
}

/// Rota el log si supera `max_bytes`.
///
/// Se conserva una sola generación en `<log>.1`. Es suficiente para un uso
/// personal durante años y evita añadir dependencias de rotación.
pub fn rotate_if_needed(log_file: &Path, max_bytes: u64) -> Result<bool, BasicError> {
    let metadata = match fs::metadata(log_file) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(source) => {
            return Err(BasicError::ReadFile {
                path: log_file.to_path_buf(),
                source,
            });
        }
    };

    if metadata.len() <= max_bytes {
        return Ok(false);
    }

    let rotated = rotated_path(log_file);
    match fs::remove_file(&rotated) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(BasicError::WriteFile {
                path: rotated,
                source,
            });
        }
    }

    fs::rename(log_file, &rotated).map_err(|source| BasicError::WriteFile {
        path: log_file.to_path_buf(),
        source,
    })?;

    Ok(true)
}

fn rotated_path(log_file: &Path) -> PathBuf {
    let mut path = log_file.as_os_str().to_owned();
    path.push(".1");
    PathBuf::from(path)
}

fn parent_dir(path: &Path) -> Option<&Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn rotates_only_when_over_limit() {
        let temp = TempDir::new().unwrap();
        let log = temp.path().join("basic.log");
        fs::write(&log, b"12345").unwrap();

        assert!(!rotate_if_needed(&log, 10).unwrap());
        assert_eq!(fs::read(&log).unwrap(), b"12345");

        assert!(rotate_if_needed(&log, 4).unwrap());
        assert!(!log.exists());
        assert_eq!(fs::read(temp.path().join("basic.log.1")).unwrap(), b"12345");

        // Una segunda rotación reemplaza la generación anterior.
        fs::write(&log, b"abcdefgh").unwrap();
        assert!(rotate_if_needed(&log, 4).unwrap());
        assert_eq!(
            fs::read(temp.path().join("basic.log.1")).unwrap(),
            b"abcdefgh"
        );
    }

    #[test]
    fn missing_log_is_not_an_error() {
        let temp = TempDir::new().unwrap();
        assert!(!rotate_if_needed(&temp.path().join("nope.log"), 1).unwrap());
    }

    #[test]
    fn rotated_path_preserves_the_full_name() {
        assert_eq!(
            rotated_path(Path::new("/tmp/basic.log")),
            Path::new("/tmp/basic.log.1")
        );
    }
}
