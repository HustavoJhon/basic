use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::BasicError;

/// Escribe un archivo de forma atómica: escribe a un temporal en el mismo
/// directorio, sincroniza y renombra.
///
/// El renombrado en el mismo sistema de archivos es atómico, así que un corte
/// de energía no deja la configuración a medias. Si algo falla, el temporal se
/// elimina.
pub fn atomic_write(path: &Path, contents: &[u8]) -> Result<(), BasicError> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    fs::create_dir_all(parent).map_err(|source| BasicError::CreateDir {
        path: parent.to_path_buf(),
        source,
    })?;

    let temporary = temporary_path(path);
    let write_result = (|| -> io::Result<()> {
        let mut file = File::create(&temporary)?;
        file.write_all(contents)?;
        file.flush()?;
        file.sync_all()?;
        fs::rename(&temporary, path)?;
        // Persistir también la entrada de directorio en sistemas POSIX.
        if let Ok(directory) = File::open(parent) {
            let _ = directory.sync_all();
        }
        Ok(())
    })();

    if let Err(source) = write_result {
        let _ = fs::remove_file(&temporary);
        return Err(BasicError::WriteFile {
            path: path.to_path_buf(),
            source,
        });
    }

    Ok(())
}

fn temporary_path(path: &Path) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    let mut name = OsString::from(".");
    name.push(path.file_name().unwrap_or_else(|| OsStr::new("basic")));
    name.push(format!(".tmp-{}-{nanos}", std::process::id()));

    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn writes_and_replaces_content() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("nested/config.toml");

        atomic_write(&path, b"first").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"first");

        atomic_write(&path, b"second").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"second");
    }

    #[test]
    fn leaves_no_temporary_files() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("config.toml");
        atomic_write(&path, b"content").unwrap();

        let entries: Vec<_> = fs::read_dir(temp.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(entries, vec![OsString::from("config.toml")]);
    }

    #[test]
    fn reports_write_errors_with_context() {
        let temp = TempDir::new().unwrap();
        let directory = temp.path().join("dir");
        fs::create_dir(&directory).unwrap();

        let error = atomic_write(&directory, b"nope").unwrap_err();
        match error {
            BasicError::WriteFile { path, .. } => assert_eq!(path, directory),
            other => panic!("error inesperado: {other:?}"),
        }
    }
}
