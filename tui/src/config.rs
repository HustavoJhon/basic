use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::BasicError;
use crate::infra::fs::atomic_write;
use crate::paths::{Paths, expand_tilde};

/// Configuración completa de BASIC.
///
/// Todos los campos tienen valores por defecto razonables para que el archivo
/// sea opcional. Los campos desconocidos se rechazan para detectar erratas en
/// lugar de ignorarlas silenciosamente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Raíz del vault de Markdown.
    pub vault: PathBuf,
    /// Base de datos SQLite; por defecto vive fuera del vault.
    pub database: PathBuf,
    /// Editor externo usado por `basic edit` en fases posteriores.
    pub editor: String,
    /// Tema de la TUI (Gruvbox en F2).
    pub theme: String,
    /// Estilo de enlaces generado al crear notas.
    pub link_style: LinkStyle,
    /// Añadir frontmatter solo a notas nuevas.
    pub frontmatter: bool,
    pub scan: ScanConfig,
    pub pomodoro: PomodoroConfig,
    pub ui: UiConfig,
    pub code_lab: CodeLabConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vault: PathBuf::new(),
            database: PathBuf::new(),
            editor: "nvim".to_string(),
            theme: "gruvbox-dark".to_string(),
            link_style: LinkStyle::Markdown,
            frontmatter: true,
            scan: ScanConfig::default(),
            pomodoro: PomodoroConfig::default(),
            ui: UiConfig::default(),
            code_lab: CodeLabConfig::default(),
        }
    }
}

impl Config {
    /// Configuración inicial para una instalación nueva.
    ///
    /// Usa `BASIC_VAULT` o el directorio actual como vault, `EDITOR` para el
    /// editor y la base de datos XDG resuelta.
    pub fn default_for(paths: &Paths) -> Self {
        let mut config = Self {
            vault: default_vault(),
            database: paths.database.clone(),
            editor: default_editor(),
            ..Self::default()
        };
        config.normalize(paths);
        config
    }

    /// Carga la configuración o devuelve los valores por defecto.
    pub fn load(paths: &Paths) -> Result<LoadedConfig, BasicError> {
        if !paths.config_file.exists() {
            return Ok(LoadedConfig {
                config: Self::default_for(paths),
                source: ConfigSource::Defaults,
            });
        }

        let text =
            fs::read_to_string(&paths.config_file).map_err(|source| BasicError::ReadFile {
                path: paths.config_file.clone(),
                source,
            })?;
        let mut config: Config =
            toml::from_str(&text).map_err(|source| BasicError::ParseConfig {
                path: paths.config_file.clone(),
                source,
            })?;
        config.normalize(paths);

        Ok(LoadedConfig {
            config,
            source: ConfigSource::File(paths.config_file.clone()),
        })
    }

    /// Completa rutas vacías, expande `~` y convierte rutas relativas.
    fn normalize(&mut self, paths: &Paths) {
        if self.vault.as_os_str().is_empty() {
            self.vault = default_vault();
        }
        if self.database.as_os_str().is_empty() {
            self.database = paths.database.clone();
        }

        self.vault = normalize_path(paths, &self.vault);
        self.database = normalize_path(paths, &self.database);
    }

    /// Comprueba invariantes que no puede expresar el sistema de tipos.
    ///
    /// Devuelve todos los problemas encontrados para que `doctor` los muestre
    /// de una vez en lugar de obligar a corregir uno por uno.
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();

        if self.vault.as_os_str().is_empty() {
            issues.push("vault no puede estar vacío".to_string());
        } else if !self.vault.is_absolute() {
            issues.push(format!(
                "vault debe ser una ruta absoluta: {}",
                self.vault.display()
            ));
        }

        if self.database.as_os_str().is_empty() {
            issues.push("database no puede estar vacío".to_string());
        } else if !self.database.is_absolute() {
            issues.push(format!(
                "database debe ser una ruta absoluta: {}",
                self.database.display()
            ));
        }

        if self.editor.trim().is_empty() {
            issues.push("editor no puede estar vacío".to_string());
        }
        if self.theme.trim().is_empty() {
            issues.push("theme no puede estar vacío".to_string());
        }

        if self.scan.exclude.is_empty() {
            issues.push("scan.exclude no puede estar vacío".to_string());
        }
        for (index, pattern) in self.scan.exclude.iter().enumerate() {
            if pattern.trim().is_empty() {
                issues.push(format!("scan.exclude[{index}] está vacío"));
            }
            if Path::new(pattern).is_absolute() {
                issues.push(format!(
                    "scan.exclude[{index}] debe ser relativo al vault: {pattern}"
                ));
            }
        }

        let pomodoro = &self.pomodoro;
        if !(1..=180).contains(&pomodoro.focus_minutes) {
            issues.push("pomodoro.focus_minutes debe estar entre 1 y 180".to_string());
        }
        if !(1..=60).contains(&pomodoro.short_break_minutes) {
            issues.push("pomodoro.short_break_minutes debe estar entre 1 y 60".to_string());
        }
        if !(1..=120).contains(&pomodoro.long_break_minutes) {
            issues.push("pomodoro.long_break_minutes debe estar entre 1 y 120".to_string());
        }
        if !(1..=12).contains(&pomodoro.long_break_every) {
            issues.push("pomodoro.long_break_every debe estar entre 1 y 12".to_string());
        }

        if !(0..=20).contains(&self.code_lab.daily_lessons) {
            issues.push("code_lab.daily_lessons debe estar entre 0 y 20".to_string());
        }
        if !(0..=50).contains(&self.code_lab.daily_exercises) {
            issues.push("code_lab.daily_exercises debe estar entre 0 y 50".to_string());
        }

        issues
    }

    /// Serializa la configuración como TOML.
    pub fn to_toml(&self) -> Result<String, BasicError> {
        Ok(toml::to_string_pretty(self)?)
    }

    /// Genera el contenido de un archivo de configuración nuevo.
    pub fn render_template(&self) -> Result<String, BasicError> {
        let body = self.to_toml()?;
        Ok(format!(
            "# BASIC — configuración\n\
             # Este archivo se creó automáticamente; edítalo a mano.\n\
             # RUST_LOG=debug basic doctor para más detalle en el log.\n\n\
             {body}"
        ))
    }

    /// Escribe la configuración de forma atómica.
    pub fn save(&self, path: &Path) -> Result<(), BasicError> {
        let text = self.render_template()?;
        atomic_write(path, text.as_bytes())
    }
}

/// Configuración cargada junto con su procedencia.
#[derive(Debug, Clone)]
pub struct LoadedConfig {
    pub config: Config,
    pub source: ConfigSource,
}

impl LoadedConfig {
    pub fn validate(&self) -> Vec<String> {
        self.config.validate()
    }
}

/// De dónde salieron los valores efectivos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    Defaults,
    File(PathBuf),
}

impl ConfigSource {
    pub fn describe(&self) -> String {
        match self {
            Self::Defaults => "valores por defecto (no hay archivo de configuración)".to_string(),
            Self::File(path) => path.display().to_string(),
        }
    }
}

/// Estilo de enlaces para las notas nuevas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkStyle {
    Markdown,
    Wiki,
}

impl fmt::Display for LinkStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Markdown => formatter.write_str("markdown"),
            Self::Wiki => formatter.write_str("wiki"),
        }
    }
}

/// Qué recorrer al indexar el vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ScanConfig {
    /// Prefijos relativos al vault que se ignoran.
    pub exclude: Vec<String>,
    pub follow_symlinks: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            exclude: vec![
                ".git".to_string(),
                ".obsidian".to_string(),
                ".trash".to_string(),
                "tui".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
            ],
            follow_symlinks: false,
        }
    }
}

/// Parámetros del temporizador Pomodoro.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PomodoroConfig {
    pub focus_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub long_break_every: u32,
    pub auto_start_next: bool,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            focus_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            long_break_every: 4,
            auto_start_next: false,
        }
    }
}

/// Preferencias de presentación.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct UiConfig {
    pub icons: IconSet,
    pub mouse: bool,
    pub week_starts_on: WeekStart,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            icons: IconSet::Ascii,
            mouse: false,
            week_starts_on: WeekStart::Monday,
        }
    }
}

/// Objetivos diarios de práctica en Code Lab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CodeLabConfig {
    pub daily_lessons: i32,
    pub daily_exercises: i32,
}

impl Default for CodeLabConfig {
    fn default() -> Self {
        Self {
            daily_lessons: 1,
            daily_exercises: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IconSet {
    Ascii,
    Nerd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WeekStart {
    Monday,
    Sunday,
}

fn default_editor() -> String {
    env::var("EDITOR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "nvim".to_string())
}

/// Vault por defecto: `BASIC_VAULT` o la raíz de Obsidian más cercana.
///
/// Buscar `.obsidian` hacia arriba permite ejecutar `basic config init` desde
/// cualquier subdirectorio del vault sin configurar mal la raíz.
fn default_vault() -> PathBuf {
    if let Some(value) = env::var_os("BASIC_VAULT").filter(|value| !value.is_empty()) {
        return PathBuf::from(value);
    }
    let cwd = env::current_dir().unwrap_or_default();
    discover_vault(&cwd)
}

/// Sube desde `start` hasta encontrar un directorio `.obsidian`.
///
/// Si no hay ninguno en los primeros 10 niveles, devuelve `start`.
pub(crate) fn discover_vault(start: &Path) -> PathBuf {
    for ancestor in start.ancestors().take(10) {
        if ancestor.join(".obsidian").is_dir() {
            return ancestor.to_path_buf();
        }
    }
    start.to_path_buf()
}

fn normalize_path(paths: &Paths, path: &Path) -> PathBuf {
    let expanded = expand_tilde(&paths.home_dir, path);
    if expanded.is_absolute() {
        expanded
    } else {
        std::path::absolute(&expanded).unwrap_or(expanded)
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use tempfile::TempDir;

    use super::*;

    fn test_paths() -> Paths {
        Paths::from_dirs(
            Path::new("/home/test"),
            Path::new("/home/test/.config"),
            Path::new("/home/test/.local/share"),
            Path::new("/home/test/.local/state"),
            |_| None::<OsString>,
        )
    }

    #[test]
    fn minimal_file_inherits_defaults_and_database_path() {
        let paths = test_paths();
        let mut config: Config = toml::from_str("vault = \"~/vault\"\n").unwrap();
        config.normalize(&paths);

        assert_eq!(config.vault, Path::new("/home/test/vault"));
        assert_eq!(config.database, paths.database);
        assert_eq!(config.editor, "nvim");
        assert_eq!(config.theme, "gruvbox-dark");
        assert_eq!(config.link_style, LinkStyle::Markdown);
        assert_eq!(config.pomodoro.focus_minutes, 25);
        assert_eq!(config.ui.icons, IconSet::Ascii);
        assert!(config.validate().is_empty());
    }

    #[test]
    fn code_lab_goals_have_defaults_and_are_validated() {
        let config: Config = toml::from_str("vault = \"~/vault\"\n").unwrap();
        assert_eq!(config.code_lab.daily_lessons, 1);
        assert_eq!(config.code_lab.daily_exercises, 2);

        let bad = Config {
            code_lab: CodeLabConfig {
                daily_lessons: -1,
                ..CodeLabConfig::default()
            },
            ..Config::default()
        };
        assert!(
            bad.validate()
                .iter()
                .any(|issue| issue.contains("daily_lessons"))
        );
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let result = toml::from_str::<Config>("vault = \".\"\nunknown = true\n");
        assert!(result.is_err());
    }

    #[test]
    fn validation_reports_all_problems() {
        let config = Config {
            vault: PathBuf::from("relative"),
            database: PathBuf::new(),
            editor: "  ".to_string(),
            theme: String::new(),
            scan: ScanConfig {
                exclude: vec!["/absolute".to_string(), String::new()],
                ..ScanConfig::default()
            },
            pomodoro: PomodoroConfig {
                focus_minutes: 0,
                short_break_minutes: 0,
                long_break_minutes: 0,
                long_break_every: 0,
                ..PomodoroConfig::default()
            },
            ..Config::default()
        };

        let issues = config.validate();
        assert!(issues.len() >= 8, "{issues:?}");
        assert!(issues.iter().any(|issue| issue.contains("vault")));
        assert!(issues.iter().any(|issue| issue.contains("database")));
        assert!(issues.iter().any(|issue| issue.contains("editor")));
        assert!(issues.iter().any(|issue| issue.contains("theme")));
        assert!(issues.iter().any(|issue| issue.contains("scan.exclude")));
        assert!(issues.iter().any(|issue| issue.contains("focus_minutes")));
    }

    #[test]
    fn save_and_load_round_trip() {
        let temp = TempDir::new().unwrap();
        let paths = Paths::from_dirs(
            Path::new("/home/test"),
            temp.path(),
            temp.path(),
            temp.path(),
            |_| None::<OsString>,
        );
        let mut config = Config::default_for(&paths);
        config.vault = temp.path().join("vault");
        config.pomodoro.focus_minutes = 45;

        config.save(&paths.config_file).unwrap();
        let text = fs::read_to_string(&paths.config_file).unwrap();
        assert!(text.starts_with("# BASIC — configuración"));

        let loaded = Config::load(&paths).unwrap();
        assert_eq!(loaded.source, ConfigSource::File(paths.config_file));
        assert_eq!(loaded.config.vault, config.vault);
        assert_eq!(loaded.config.pomodoro.focus_minutes, 45);
        assert!(loaded.validate().is_empty());
    }

    #[test]
    fn missing_file_uses_defaults() {
        let temp = TempDir::new().unwrap();
        let paths = Paths::from_dirs(temp.path(), temp.path(), temp.path(), temp.path(), |_| {
            None::<OsString>
        });
        let loaded = Config::load(&paths).unwrap();
        assert_eq!(loaded.source, ConfigSource::Defaults);
        assert_eq!(loaded.config.database, paths.database);
    }

    #[test]
    fn discovers_the_nearest_obsidian_root() {
        let temp = TempDir::new().unwrap();
        let vault = temp.path().join("vault");
        let nested = vault.join("linux/vim");
        fs::create_dir_all(vault.join(".obsidian")).unwrap();
        fs::create_dir_all(&nested).unwrap();

        assert_eq!(discover_vault(&nested), vault);

        let plain = temp.path().join("plain");
        fs::create_dir_all(&plain).unwrap();
        assert_eq!(discover_vault(&plain), plain);
    }

    #[test]
    fn invalid_toml_reports_the_path() {
        let temp = TempDir::new().unwrap();
        let paths = Paths::from_dirs(temp.path(), temp.path(), temp.path(), temp.path(), |_| {
            None::<OsString>
        });
        fs::create_dir_all(&paths.config_dir).unwrap();
        fs::write(&paths.config_file, "vault = [").unwrap();

        let error = Config::load(&paths).unwrap_err();
        match error {
            BasicError::ParseConfig { path, .. } => assert_eq!(path, paths.config_file),
            other => panic!("error inesperado: {other:?}"),
        }
    }
}
