//! Escáner e indexador del vault.
//!
//! El índice es reconstruible: si se borra SQLite, Markdown sigue intacto y
//! este módulo vuelve a poblar el catálogo sin modificar ninguna nota.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rusqlite::{OptionalExtension, params};
use serde::Serialize;
use sha2::{Digest, Sha256};
use walkdir::{DirEntry, WalkDir};

use crate::config::Config;
use crate::error::BasicError;
use crate::infra::markdown;
use crate::infra::sqlite::Database;

/// Resultado de una pasada del indexador.
#[derive(Debug, Clone, Default, Serialize)]
pub struct IndexReport {
    pub total: usize,
    pub added: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub missing: usize,
    pub errors: Vec<String>,
    pub duration_ms: u128,
}

impl IndexReport {
    pub fn summary(&self) -> String {
        format!(
            "{} documentos · {} nuevos · {} actualizados · {} sin cambios · {} ausentes · {} errores · {} ms",
            self.total,
            self.added,
            self.updated,
            self.unchanged,
            self.missing,
            self.errors.len(),
            self.duration_ms
        )
    }
}

/// Recorre el vault y actualiza el catálogo.
pub fn index_vault(database: &mut Database, config: &Config) -> Result<IndexReport, BasicError> {
    let started = Instant::now();
    let vault = &config.vault;
    if !vault.is_dir() {
        return Err(BasicError::InvalidConfig {
            path: vault.clone(),
            message: "el vault no existe o no es un directorio".to_string(),
        });
    }

    let excludes = &config.scan.exclude;
    let walker = WalkDir::new(vault)
        .follow_links(config.scan.follow_symlinks)
        .into_iter()
        .filter_entry(|entry| keep_entry(entry, vault, excludes));

    let mut report = IndexReport::default();
    let mut seen = HashSet::new();
    let now = unix_now();

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                report.errors.push(error.to_string());
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if !is_markdown(path) {
            continue;
        }
        let Some(relative) = relative_path(vault, path) else {
            continue;
        };
        seen.insert(relative.clone());

        match index_one(database, path, &relative, now) {
            Ok(Indexed::Added) => {
                report.total += 1;
                report.added += 1;
            }
            Ok(Indexed::Updated) => {
                report.total += 1;
                report.updated += 1;
            }
            Ok(Indexed::Unchanged) => {
                report.total += 1;
                report.unchanged += 1;
            }
            Err(error) => report.errors.push(format!("{relative}: {error}")),
        }
    }

    report.missing = mark_missing(database, &seen, now)?;
    resolve_links(database, vault)?;
    report.duration_ms = started.elapsed().as_millis();
    Ok(report)
}

enum Indexed {
    Added,
    Updated,
    Unchanged,
}

fn index_one(
    database: &mut Database,
    absolute: &Path,
    relative: &str,
    now: i64,
) -> Result<Indexed, BasicError> {
    let raw = fs::read_to_string(absolute).map_err(|source| BasicError::ReadFile {
        path: absolute.to_path_buf(),
        source,
    })?;
    let metadata = fs::metadata(absolute)?;
    let parsed = markdown::parse_document(&raw, relative);
    let hash = sha256_hex(raw.as_bytes());
    let mtime_ns = metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_nanos() as i64)
        .unwrap_or(0);
    let size_bytes = metadata.len() as i64;
    let (category, subcategory) = split_category(relative);
    let kind = classify(relative, &parsed.title);
    let reading_minutes = ((parsed.word_count as f64) / 200.0).ceil() as i64;

    let connection = database.connection_mut();
    let existing = connection
        .query_row(
            "SELECT id, content_hash FROM documents WHERE path = ?1",
            [relative],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?)),
        )
        .optional()?;

    let (id, state) = match existing {
        Some((id, Some(existing_hash))) if existing_hash == hash => {
            connection.execute(
                "UPDATE documents
                 SET mtime_ns = ?1, size_bytes = ?2, missing_at = NULL
                 WHERE id = ?3",
                params![mtime_ns, size_bytes, id],
            )?;
            (id, Indexed::Unchanged)
        }
        Some((id, _)) => {
            connection.execute(
                "UPDATE documents
                 SET title = ?1, category = ?2, subcategory = ?3, kind = ?4,
                     size_bytes = ?5, mtime_ns = ?6, content_hash = ?7,
                     word_count = ?8, reading_minutes = ?9, excerpt = ?10,
                     is_stub = ?11, indexed_at = ?12, missing_at = NULL
                 WHERE id = ?13",
                params![
                    parsed.title,
                    category,
                    subcategory,
                    kind,
                    size_bytes,
                    mtime_ns,
                    hash,
                    parsed.word_count as i64,
                    reading_minutes,
                    parsed.excerpt,
                    parsed.is_stub as i64,
                    now,
                    id
                ],
            )?;
            (id, Indexed::Updated)
        }
        None => {
            let id = connection.query_row(
                "INSERT INTO documents
                 (path, title, category, subcategory, kind, size_bytes, mtime_ns,
                  content_hash, word_count, reading_minutes, excerpt, is_stub, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                 RETURNING id",
                params![
                    relative,
                    parsed.title,
                    category,
                    subcategory,
                    kind,
                    size_bytes,
                    mtime_ns,
                    hash,
                    parsed.word_count as i64,
                    reading_minutes,
                    parsed.excerpt,
                    parsed.is_stub as i64,
                    now
                ],
                |row| row.get(0),
            )?;
            (id, Indexed::Added)
        }
    };

    if matches!(state, Indexed::Unchanged) {
        return Ok(state);
    }

    connection.execute("DELETE FROM documents_fts WHERE rowid = ?1", [id])?;
    connection.execute(
        "INSERT INTO documents_fts (rowid, title, body, path) VALUES (?1, ?2, ?3, ?4)",
        params![id, parsed.title, parsed.body, relative],
    )?;

    connection.execute("DELETE FROM document_tags WHERE document_id = ?1", [id])?;
    for tag in &parsed.tags {
        connection.execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", [tag])?;
        let tag_id: i64 =
            connection.query_row("SELECT id FROM tags WHERE name = ?1", [tag], |row| {
                row.get(0)
            })?;
        connection.execute(
            "INSERT OR IGNORE INTO document_tags (document_id, tag_id) VALUES (?1, ?2)",
            params![id, tag_id],
        )?;
    }

    connection.execute("DELETE FROM links WHERE source_id = ?1", [id])?;
    for link in &parsed.links {
        connection.execute(
            "INSERT INTO links (source_id, target_path, kind, text, resolved)
             VALUES (?1, ?2, ?3, ?4, 0)",
            params![id, link.target, link.kind, link.text],
        )?;
    }

    connection.execute(
        "INSERT OR IGNORE INTO learning_state (document_id, updated_at) VALUES (?1, ?2)",
        params![id, now],
    )?;

    Ok(state)
}

fn mark_missing(
    database: &mut Database,
    seen: &HashSet<String>,
    now: i64,
) -> Result<usize, BasicError> {
    let connection = database.connection_mut();
    let mut statement =
        connection.prepare("SELECT id, path FROM documents WHERE missing_at IS NULL")?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    drop(statement);

    let mut missing = 0;
    for (id, path) in rows {
        if !seen.contains(&path) {
            connection.execute(
                "UPDATE documents SET missing_at = ?1 WHERE id = ?2",
                params![now, id],
            )?;
            missing += 1;
        }
    }
    Ok(missing)
}

/// Resuelve enlaces internos y marca los externos.
fn resolve_links(database: &mut Database, vault: &Path) -> Result<(), BasicError> {
    let connection = database.connection_mut();
    let mut statement = connection.prepare(
        "SELECT l.id, d.path, d.title, l.target_path, l.kind
         FROM links l JOIN documents d ON d.id = l.source_id",
    )?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    drop(statement);

    for (link_id, source_path, _source_title, target, kind) in rows {
        if kind == "external" {
            connection.execute(
                "UPDATE links SET target_id = NULL, resolved = 1 WHERE id = ?1",
                [link_id],
            )?;
            continue;
        }

        let target_id = if kind == "wiki" {
            resolve_wiki(connection, &target)?
        } else {
            resolve_relative(connection, &source_path, &target)?
        };

        if kind == "image" {
            let exists = target_id.is_some()
                || resolve_relative(connection, &source_path, &target)?.is_some()
                || image_exists(vault, &source_path, &target);
            connection.execute(
                "UPDATE links SET target_id = ?1, resolved = ?2 WHERE id = ?3",
                params![target_id, exists as i64, link_id],
            )?;
        } else {
            connection.execute(
                "UPDATE links SET target_id = ?1, resolved = ?2 WHERE id = ?3",
                params![target_id, target_id.is_some() as i64, link_id],
            )?;
        }
    }
    Ok(())
}

fn resolve_wiki(
    connection: &rusqlite::Connection,
    target: &str,
) -> Result<Option<i64>, BasicError> {
    let target = target.trim();
    let mut candidates = vec![target.to_string()];
    if !target.to_lowercase().ends_with(".md") {
        candidates.push(format!("{target}.md"));
    }
    for candidate in &candidates {
        if let Some(id) = lookup_path(connection, candidate)? {
            return Ok(Some(id));
        }
    }
    Ok(connection
        .query_row(
            "SELECT id FROM documents WHERE lower(title) = lower(?1) AND missing_at IS NULL LIMIT 1",
            [target],
            |row| row.get(0),
        )
        .optional()?)
}

fn resolve_relative(
    connection: &rusqlite::Connection,
    source_path: &str,
    target: &str,
) -> Result<Option<i64>, BasicError> {
    let target = target
        .split('#')
        .next()
        .unwrap_or(target)
        .replace("%20", " ")
        .trim()
        .trim_start_matches("./")
        .trim_start_matches('/')
        .to_string();
    if target.is_empty() {
        return Ok(None);
    }
    let base = source_path
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or("");
    let normalized = normalize_relative(base, &target);

    let mut candidates = vec![normalized.clone()];
    if !normalized.to_lowercase().ends_with(".md") {
        candidates.push(format!("{normalized}.md"));
        candidates.push(format!("{normalized}/README.md"));
        candidates.push(format!("{normalized}/readme.md"));
    }
    for candidate in candidates {
        if let Some(id) = lookup_path(connection, &candidate)? {
            return Ok(Some(id));
        }
    }
    Ok(None)
}

fn lookup_path(connection: &rusqlite::Connection, path: &str) -> Result<Option<i64>, BasicError> {
    Ok(connection
        .query_row(
            "SELECT id FROM documents WHERE path = ?1 AND missing_at IS NULL LIMIT 1",
            [path],
            |row| row.get(0),
        )
        .optional()?)
}

fn image_exists(vault: &Path, source_path: &str, target: &str) -> bool {
    let base = source_path
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or("");
    let normalized = normalize_relative(base, target);
    vault.join(normalized).is_file()
}

fn normalize_relative(base: &str, target: &str) -> String {
    let mut parts: Vec<&str> = base.split('/').filter(|part| !part.is_empty()).collect();
    for part in target.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}

fn keep_entry(entry: &DirEntry, vault: &Path, excludes: &[String]) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    let Some(relative) = relative_path(vault, entry.path()) else {
        return false;
    };
    !is_excluded(&relative, excludes)
}

fn is_excluded(relative: &str, excludes: &[String]) -> bool {
    excludes.iter().any(|exclude| {
        let exclude = exclude.trim_matches('/');
        relative == exclude || relative.starts_with(&format!("{exclude}/"))
    })
}

fn relative_path(vault: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(vault).ok()?;
    let mut parts = Vec::new();
    for component in relative.components() {
        parts.push(component.as_os_str().to_string_lossy());
    }
    Some(parts.join("/"))
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

fn split_category(relative: &str) -> (String, Option<String>) {
    let parts = relative.split('/').collect::<Vec<_>>();
    if parts.len() < 2 {
        return ("(raíz)".to_string(), None);
    }
    let category = parts[0].to_string();
    // Solo hay subcategoría cuando la nota está en un tercer nivel; el nombre
    // del archivo nunca es una subcategoría.
    let subcategory = if parts.len() >= 3 {
        parts.get(1).map(|part| (*part).to_string())
    } else {
        None
    };
    (category, subcategory)
}

fn classify(relative: &str, title: &str) -> &'static str {
    let lower = relative.to_lowercase();
    let stem = Path::new(relative)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("")
        .to_lowercase();
    let title = title.to_lowercase();
    if lower.contains("proyecto") || lower.contains("project") {
        "project"
    } else if stem == "readme" || stem == "index" || title.starts_with("índice") {
        "index"
    } else if lower.starts_with("resources/") {
        "resource"
    } else if lower.contains("diario") || lower.contains("journal") {
        "journal"
    } else {
        "note"
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::config::Config;
    use crate::paths::Paths;

    fn fixture() -> (TempDir, Config) {
        let temp = TempDir::new().unwrap();
        let vault = temp.path().join("vault");
        fs::create_dir_all(vault.join("linux/vim")).unwrap();
        fs::create_dir_all(vault.join("tui")).unwrap();
        fs::write(vault.join("README.md"), "# Vault\n\nNota raíz.\n").unwrap();
        fs::write(
            vault.join("linux/vim/atajos.md"),
            "# Atajos de Vim\n\n`dd`, `yy` y `p`.\n\n#vim\n",
        )
        .unwrap();
        fs::write(vault.join("tui/README.md"), "# TUI\n\nNo indexar.\n").unwrap();

        let paths = Paths::from_dirs(
            temp.path(),
            &temp.path().join("config"),
            &temp.path().join("data"),
            &temp.path().join("state"),
            |_| None,
        );
        let mut config = Config::default_for(&paths);
        config.vault = vault;
        config.database = paths.database;
        (temp, config)
    }

    #[test]
    fn indexes_markdown_and_skips_excluded_directories() {
        let (_temp, config) = fixture();
        let mut database = Database::open_in_memory().unwrap();
        let report = index_vault(&mut database, &config).unwrap();

        assert_eq!(report.added, 2, "{report:?}");
        assert_eq!(report.errors.len(), 0);
        let count: i64 = database
            .connection()
            .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
        let fts: i64 = database
            .connection()
            .query_row("SELECT count(*) FROM documents_fts", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fts, 2);
        let tags: i64 = database
            .connection()
            .query_row("SELECT count(*) FROM document_tags", [], |row| row.get(0))
            .unwrap();
        assert_eq!(tags, 1);
    }

    #[test]
    fn second_pass_is_incremental_and_detects_missing_files() {
        let (_temp, config) = fixture();
        let mut database = Database::open_in_memory().unwrap();
        index_vault(&mut database, &config).unwrap();
        let report = index_vault(&mut database, &config).unwrap();
        assert_eq!(report.unchanged, 2);
        assert_eq!(report.added, 0);

        fs::remove_file(config.vault.join("linux/vim/atajos.md")).unwrap();
        let report = index_vault(&mut database, &config).unwrap();
        assert_eq!(report.missing, 1);
        let missing: i64 = database
            .connection()
            .query_row(
                "SELECT count(*) FROM documents WHERE missing_at IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(missing, 1);
    }

    #[test]
    fn category_never_uses_the_file_name_as_subcategory() {
        assert_eq!(split_category("README.md"), ("(raíz)".to_string(), None));
        assert_eq!(split_category("linux/vim.md"), ("linux".to_string(), None));
        assert_eq!(
            split_category("linux/vim/atajos.md"),
            ("linux".to_string(), Some("vim".to_string()))
        );
        assert_eq!(
            split_category("linux/vim/README.md"),
            ("linux".to_string(), Some("vim".to_string()))
        );
    }

    #[test]
    fn resolves_relative_links() {
        let (_temp, config) = fixture();
        fs::write(
            config.vault.join("linux/vim/atajos.md"),
            "# Atajos\n\n[Vault](../../README.md)\n",
        )
        .unwrap();
        let mut database = Database::open_in_memory().unwrap();
        index_vault(&mut database, &config).unwrap();
        let resolved: i64 = database
            .connection()
            .query_row("SELECT count(*) FROM links WHERE resolved = 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(resolved, 1);
    }
}
