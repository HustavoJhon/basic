use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};

use directories::BaseDirs;
use serde::Serialize;

use crate::error::BasicError;

/// Nombre de la aplicación dentro de los directorios XDG.
pub const APP_DIR: &str = "basic";
/// Nombre del archivo de configuración.
pub const CONFIG_FILE: &str = "config.toml";
/// Nombre del archivo SQLite.
pub const DATABASE_FILE: &str = "basic.db";
/// Nombre del archivo de logs.
pub const LOG_FILE: &str = "basic.log";

/// Rutas resueltas de configuración, datos y estado.
///
/// Se calculan una sola vez al arrancar. Las variables de entorno permiten
/// instalaciones portables y pruebas aisladas sin tocar el HOME real.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Paths {
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
    pub database: PathBuf,
    pub log_file: PathBuf,
}

impl Paths {
    /// Resuelve las rutas a partir de XDG y de las variables `BASIC_*`.
    pub fn resolve() -> Result<Self, BasicError> {
        let base = BaseDirs::new().ok_or(BasicError::NoHome)?;
        let state_root = base
            .state_dir()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| base.home_dir().join(".local/state"));

        Ok(Self::from_dirs(
            base.home_dir(),
            base.config_dir(),
            base.data_dir(),
            &state_root,
            |key| std::env::var_os(key),
        ))
    }

    /// Construye las rutas a partir de raíces explícitas.
    ///
    /// Es la parte pura de [`Paths::resolve`]: no lee el entorno ni el sistema
    /// de archivos, lo que permite probarla con directorios temporales.
    pub fn from_dirs(
        home_dir: &Path,
        config_root: &Path,
        data_root: &Path,
        state_root: &Path,
        env: impl Fn(&str) -> Option<OsString>,
    ) -> Self {
        let env_path = |key: &str| env_path(&env, key, home_dir);

        let config_dir = env_path("BASIC_CONFIG_DIR").unwrap_or_else(|| config_root.join(APP_DIR));
        let config_file = env_path("BASIC_CONFIG").unwrap_or_else(|| config_dir.join(CONFIG_FILE));
        let data_dir = env_path("BASIC_DATA_DIR").unwrap_or_else(|| data_root.join(APP_DIR));
        let state_dir = env_path("BASIC_STATE_DIR").unwrap_or_else(|| state_root.join(APP_DIR));
        let database = env_path("BASIC_DB").unwrap_or_else(|| data_dir.join(DATABASE_FILE));
        let log_file = state_dir.join(LOG_FILE);

        Self {
            home_dir: home_dir.to_path_buf(),
            config_dir,
            config_file,
            data_dir,
            state_dir,
            database,
            log_file,
        }
    }

    /// Expande `~` usando el HOME de esta resolución de rutas.
    pub fn expand_tilde(&self, path: &Path) -> PathBuf {
        expand_tilde(&self.home_dir, path)
    }
}

/// Expande únicamente `~` y `~/...`; no intenta interpretar `~usuario`.
pub fn expand_tilde(home: &Path, path: &Path) -> PathBuf {
    let mut components = path.components();
    if let Some(Component::Normal(first)) = components.next()
        && first == OsStr::new("~")
    {
        let rest = components.as_path();
        if rest.as_os_str().is_empty() {
            return home.to_path_buf();
        }
        return home.join(rest);
    }
    path.to_path_buf()
}

fn env_path(env: &impl Fn(&str) -> Option<OsString>, key: &str, home: &Path) -> Option<PathBuf> {
    env(key)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .map(|path| expand_tilde(home, &path))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn paths_with(env: &HashMap<&str, &str>) -> Paths {
        Paths::from_dirs(
            Path::new("/home/test"),
            Path::new("/home/test/.config"),
            Path::new("/home/test/.local/share"),
            Path::new("/home/test/.local/state"),
            |key| env.get(key).map(OsString::from),
        )
    }

    #[test]
    fn defaults_follow_xdg_layout() {
        let paths = paths_with(&HashMap::new());
        assert_eq!(paths.config_dir, Path::new("/home/test/.config/basic"));
        assert_eq!(
            paths.config_file,
            Path::new("/home/test/.config/basic/config.toml")
        );
        assert_eq!(paths.data_dir, Path::new("/home/test/.local/share/basic"));
        assert_eq!(
            paths.database,
            Path::new("/home/test/.local/share/basic/basic.db")
        );
        assert_eq!(
            paths.log_file,
            Path::new("/home/test/.local/state/basic/basic.log")
        );
    }

    #[test]
    fn environment_overrides_are_expanded() {
        let mut env = HashMap::new();
        env.insert("BASIC_CONFIG", "~/custom/config.toml");
        env.insert("BASIC_DATA_DIR", "/tmp/data");
        env.insert("BASIC_STATE_DIR", "/tmp/state");
        env.insert("BASIC_DB", "~/custom/basic.db");

        let paths = paths_with(&env);
        assert_eq!(
            paths.config_file,
            Path::new("/home/test/custom/config.toml")
        );
        assert_eq!(paths.database, Path::new("/home/test/custom/basic.db"));
        assert_eq!(paths.data_dir, Path::new("/tmp/data"));
        assert_eq!(paths.log_file, Path::new("/tmp/state/basic.log"));
    }

    #[test]
    fn empty_overrides_are_ignored() {
        let mut env = HashMap::new();
        env.insert("BASIC_CONFIG", "");
        env.insert("BASIC_DB", "");
        let paths = paths_with(&env);
        assert_eq!(
            paths.config_file,
            Path::new("/home/test/.config/basic/config.toml")
        );
        assert_eq!(
            paths.database,
            Path::new("/home/test/.local/share/basic/basic.db")
        );
    }

    #[test]
    fn tilde_expansion_is_conservative() {
        assert_eq!(
            expand_tilde(Path::new("/home/test"), Path::new("~")),
            Path::new("/home/test")
        );
        assert_eq!(
            expand_tilde(Path::new("/home/test"), Path::new("~/vault")),
            Path::new("/home/test/vault")
        );
        assert_eq!(
            expand_tilde(Path::new("/home/test"), Path::new("~otro/vault")),
            Path::new("~otro/vault")
        );
        assert_eq!(
            expand_tilde(Path::new("/home/test"), Path::new("notas/vault")),
            Path::new("notas/vault")
        );
    }
}
