//! Espacios de trabajo efímeros para ejecutar código de usuario.

use std::fs;
use std::path::{Path, PathBuf};

use tempfile::{Builder, TempDir};

use crate::error::BasicError;

const WORKSPACE_ROOT: &str = "basic-code-lab";

/// Directorio temporal con una estructura mínima y predecible.
///
/// Se elimina automáticamente al caer. Nunca se crea dentro del vault.
#[derive(Debug)]
pub struct Workspace {
    _temp: TempDir,
    pub root: PathBuf,
    pub project_dir: PathBuf,
    pub nuget_source: PathBuf,
    pub home: PathBuf,
    pub tmp: PathBuf,
    pub logs: PathBuf,
}

impl Workspace {
    pub fn create(vault: &Path) -> Result<Self, BasicError> {
        let base = std::env::temp_dir().join(WORKSPACE_ROOT);
        fs::create_dir_all(&base).map_err(|source| BasicError::CreateDir {
            path: base.clone(),
            source,
        })?;

        if is_within(&base, vault) {
            return Err(BasicError::UnsafeWorkspace { path: base });
        }

        let temp = Builder::new()
            .prefix("run-")
            .tempdir_in(&base)
            .map_err(BasicError::Io)?;
        let root = temp.path().to_path_buf();
        let project_dir = root.join("project");
        let nuget_source = root.join("nuget-source");
        let home = root.join("home");
        let tmp = root.join("tmp");
        let logs = root.join("logs");

        for path in [&project_dir, &nuget_source, &home, &tmp, &logs] {
            fs::create_dir_all(path).map_err(|source| BasicError::CreateDir {
                path: path.clone(),
                source,
            })?;
        }

        Ok(Self {
            _temp: temp,
            root,
            project_dir,
            nuget_source,
            home,
            tmp,
            logs,
        })
    }

    pub fn write(&self, relative: &str, contents: &str) -> Result<PathBuf, BasicError> {
        let path = self.root.join(relative);
        write_file(&path, contents)?;
        Ok(path)
    }
}

/// Escritura pequeña y acotada para archivos generados por el runner.
pub(crate) fn write_file(path: &Path, contents: &str) -> Result<(), BasicError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| BasicError::CreateDir {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, contents).map_err(|source| BasicError::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}

/// Comparación léxica/canonizada sin seguir enlaces del workspace temporal.
pub(crate) fn is_within(path: &Path, root: &Path) -> bool {
    let path = comparable(path);
    let root = comparable(root);
    path.starts_with(root)
}

fn comparable(path: &Path) -> PathBuf {
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn detects_paths_inside_another_directory() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("vault");
        fs::create_dir_all(&root).unwrap();
        let inside = root.join("notas").join("a.md");
        let outside = temp.path().join("otro").join("a.md");
        assert!(is_within(&inside, &root));
        assert!(!is_within(&outside, &root));
    }

    #[test]
    fn workspace_is_created_outside_the_vault() {
        let vault = TempDir::new().unwrap();
        let workspace = Workspace::create(vault.path()).unwrap();
        assert!(workspace.root.exists());
        assert!(workspace.project_dir.exists());
        assert!(workspace.nuget_source.exists());
        assert!(!is_within(&workspace.root, vault.path()));
    }

    #[test]
    fn writes_relative_files_inside_the_workspace() {
        let vault = TempDir::new().unwrap();
        let workspace = Workspace::create(vault.path()).unwrap();
        let path = workspace.write("project/Program.cs", "class A {}").unwrap();
        assert_eq!(path, workspace.project_dir.join("Program.cs"));
        assert_eq!(fs::read_to_string(path).unwrap(), "class A {}");
    }
}
