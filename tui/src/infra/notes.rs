//! Creación de notas nuevas.
//!
//! Solo se escriben archivos que no existen; las notas del usuario nunca se
//! reescriben automáticamente.

use std::path::{Component, Path, PathBuf};

use chrono::Local;

use crate::config::Config;
use crate::error::BasicError;
use crate::infra::fs::atomic_write;

/// Resultado de crear una nota.
#[derive(Debug, Clone)]
pub struct CreatedNote {
    pub path: PathBuf,
    pub relative: String,
}

/// Crea una nota Markdown con plantilla y frontmatter opcional.
pub fn create_note(
    config: &Config,
    relative: &str,
    title: Option<&str>,
) -> Result<CreatedNote, BasicError> {
    let relative = normalize_note_path(relative)?;
    let title = title
        .map(ToString::to_string)
        .filter(|title| !title.trim().is_empty())
        .unwrap_or_else(|| prettify_stem(&relative));
    let body = render_note(config, &title);
    create_note_with_body(config, &relative, &body)
}

/// Crea una nota con un cuerpo dado. Nunca sobrescribe una nota existente.
pub fn create_note_with_body(
    config: &Config,
    relative: &str,
    body: &str,
) -> Result<CreatedNote, BasicError> {
    let relative = normalize_note_path(relative)?;
    let absolute = config.vault.join(&relative);
    if absolute.exists() {
        return Err(BasicError::InvalidConfig {
            path: absolute,
            message: "la nota ya existe; elige otra ruta".to_string(),
        });
    }
    atomic_write(&absolute, body.as_bytes())?;
    Ok(CreatedNote {
        path: absolute,
        relative,
    })
}

/// Normaliza una ruta relativa al vault y añade `.md` cuando falta.
pub fn normalize_note_path(value: &str) -> Result<String, BasicError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(BasicError::InvalidConfig {
            path: PathBuf::from(value),
            message: "la ruta de la nota no puede estar vacía".to_string(),
        });
    }
    let raw = Path::new(value);
    if raw.is_absolute() {
        return Err(BasicError::InvalidConfig {
            path: raw.to_path_buf(),
            message: "la ruta debe ser relativa al vault".to_string(),
        });
    }

    let mut parts = Vec::new();
    for component in raw.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            Component::CurDir => {}
            _ => {
                return Err(BasicError::InvalidConfig {
                    path: raw.to_path_buf(),
                    message: "la ruta no puede salir del vault".to_string(),
                });
            }
        }
    }
    if parts.is_empty() {
        return Err(BasicError::InvalidConfig {
            path: raw.to_path_buf(),
            message: "la ruta de la nota no puede estar vacía".to_string(),
        });
    }

    let mut relative = parts.join("/");
    if !relative.to_lowercase().ends_with(".md") {
        relative.push_str(".md");
    }
    Ok(relative)
}

fn render_note(config: &Config, title: &str) -> String {
    render_note_body(config, title, "")
}

/// Añade el frontmatter y el título configurados al cuerpo de una nota.
pub fn render_note_body(config: &Config, title: &str, body: &str) -> String {
    let escaped = title.replace('"', "\\\"");
    if config.frontmatter {
        format!(
            "---\ntitle: \"{escaped}\"\ncreated: {}\ntags: []\n---\n\n# {title}\n\n{body}",
            Local::now().format("%Y-%m-%d")
        )
    } else {
        format!("# {title}\n\n{body}")
    }
}

fn prettify_stem(relative: &str) -> String {
    let stem = Path::new(relative)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("Nueva nota");
    let mut chars = stem.replace(['_', '-'], " ").chars().collect::<Vec<_>>();
    if let Some(first) = chars.first_mut() {
        first.make_ascii_uppercase();
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::paths::Paths;

    fn config() -> (TempDir, Config) {
        let temp = TempDir::new().unwrap();
        let paths = Paths::from_dirs(
            temp.path(),
            &temp.path().join("config"),
            &temp.path().join("data"),
            &temp.path().join("state"),
            |_| None,
        );
        let mut config = Config::default_for(&paths);
        config.vault = temp.path().join("vault");
        fs::create_dir_all(&config.vault).unwrap();
        (temp, config)
    }

    #[test]
    fn creates_a_note_with_frontmatter() {
        let (_temp, config) = config();
        let created = create_note(&config, "linux/atajos-vim", None).unwrap();
        assert_eq!(created.relative, "linux/atajos-vim.md");
        let text = fs::read_to_string(&created.path).unwrap();
        assert!(text.starts_with("---\ntitle: \"Atajos vim\""));
        assert!(text.contains("# Atajos vim"));
    }

    #[test]
    fn refuses_paths_outside_the_vault_and_existing_notes() {
        let (_temp, config) = config();
        assert!(create_note(&config, "../fuera", None).is_err());
        assert!(create_note(&config, "/absoluta", None).is_err());
        let created = create_note(&config, "nota", None).unwrap();
        assert!(create_note(&config, "nota", None).is_err());
        assert!(created.path.exists());
    }
}
