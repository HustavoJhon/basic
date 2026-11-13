use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{SecondsFormat, Utc};
use serde::Serialize;
use walkdir::{DirEntry, WalkDir};

use crate::config::{Config, LoadedConfig};
use crate::error::BasicError;
use crate::infra::sqlite::{Database, EXPECTED_TABLES, LATEST_SCHEMA_VERSION};
use crate::logging::MAX_LOG_BYTES;
use crate::paths::Paths;

/// Límite defensivo para que un vault enorme no bloquee `doctor`.
const MAX_SCAN_ENTRIES: usize = 200_000;

/// Resultado de una comprobación.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Warn,
    Fail,
    Skip,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Self::Ok => " OK ",
            Self::Warn => "WARN",
            Self::Fail => "FAIL",
            Self::Skip => "SKIP",
        }
    }
}

/// Una comprobación individual del diagnóstico.
#[derive(Debug, Clone, Serialize)]
pub struct Check {
    pub id: &'static str,
    pub title: &'static str,
    pub status: Status,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Recuento final por estado.
#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub ok: usize,
    pub warn: usize,
    pub fail: usize,
    pub skip: usize,
    pub healthy: bool,
}

/// Informe completo de `basic doctor`.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub version: String,
    pub generated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault: Option<String>,
    pub checks: Vec<Check>,
    pub summary: Summary,
}

impl Report {
    fn new(checks: Vec<Check>, vault: Option<String>) -> Self {
        let summary = Summary {
            ok: count(&checks, Status::Ok),
            warn: count(&checks, Status::Warn),
            fail: count(&checks, Status::Fail),
            skip: count(&checks, Status::Skip),
            healthy: count(&checks, Status::Fail) == 0,
        };

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            generated_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            vault,
            checks,
            summary,
        }
    }

    /// Código de salida: 1 si hay fallos, o avisos con `--strict`.
    pub fn exit_code(&self, strict: bool) -> i32 {
        if self.summary.fail > 0 || (strict && self.summary.warn > 0) {
            1
        } else {
            0
        }
    }

    /// Representación legible para la terminal.
    pub fn render(&self, strict: bool) -> String {
        let mut output = String::new();
        output.push_str(&format!("BASIC {} — diagnóstico\n", self.version));
        if let Some(vault) = &self.vault {
            output.push_str(&format!("Vault: {vault}\n"));
        }
        output.push_str(&format!("Fecha: {}\n\n", self.generated_at));

        for check in &self.checks {
            output.push_str(&format!(
                "[{}] {:<20} {}\n",
                check.status.label(),
                check.title,
                check.detail
            ));
            if let Some(hint) = &check.hint {
                output.push_str(&format!("       → {hint}\n"));
            }
        }

        output.push_str(&format!(
            "\nResumen: {} ok, {} avisos, {} fallos, {} omitidos\n",
            self.summary.ok, self.summary.warn, self.summary.fail, self.summary.skip
        ));

        if self.summary.fail > 0 {
            output.push_str("Estado: requiere atención.\n");
        } else if self.summary.warn > 0 {
            if strict {
                output.push_str("Estado: fallo por avisos (--strict).\n");
            } else {
                output.push_str("Estado: correcto con avisos.\n");
            }
        } else {
            output.push_str("Estado: correcto.\n");
        }

        output
    }
}

/// Ejecuta todas las comprobaciones sin modificar el vault ni la base real.
///
/// Se permiten escrituras efímeras en los directorios de datos y estado para
/// verificar permisos. La base de datos solo se abre en modo lectura.
pub fn run(paths: &Paths, loaded: &Result<LoadedConfig, BasicError>) -> Report {
    let fallback = Config::default_for(paths);
    let config = match loaded {
        Ok(loaded) => &loaded.config,
        Err(_) => &fallback,
    };

    let mut checks = Vec::new();

    check_environment(&mut checks);
    check_config_source(&mut checks, loaded);
    check_config_values(&mut checks, loaded);
    check_config_dir(&mut checks, paths);
    check_writable_dir(
        &mut checks,
        "paths.data_dir",
        "datos",
        &paths.data_dir,
        "Revisa permisos o define BASIC_DATA_DIR.",
    );
    check_writable_dir(
        &mut checks,
        "paths.state_dir",
        "estado",
        &paths.state_dir,
        "Revisa permisos o define BASIC_STATE_DIR.",
    );
    check_vault(&mut checks, config);
    check_markdown(&mut checks, config);
    check_exclusions(&mut checks, config);
    check_database_location(&mut checks, config);
    check_migrations(&mut checks);
    check_database_file(&mut checks, paths);
    check_logging(&mut checks, paths);
    check_editor(&mut checks, config);
    check_obsidian(&mut checks, config);

    let vault = if config.vault.as_os_str().is_empty() {
        None
    } else {
        Some(config.vault.display().to_string())
    };

    Report::new(checks, vault)
}

fn count(checks: &[Check], status: Status) -> usize {
    checks.iter().filter(|check| check.status == status).count()
}

fn push(
    checks: &mut Vec<Check>,
    id: &'static str,
    title: &'static str,
    status: Status,
    detail: impl Into<String>,
    hint: Option<String>,
) {
    checks.push(Check {
        id,
        title,
        status,
        detail: detail.into(),
        hint,
    });
}

fn check_environment(checks: &mut Vec<Check>) {
    push(
        checks,
        "environment",
        "entorno",
        Status::Ok,
        format!(
            "{} {} · basic {}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            env!("CARGO_PKG_VERSION")
        ),
        None,
    );
}

fn check_config_source(checks: &mut Vec<Check>, loaded: &Result<LoadedConfig, BasicError>) {
    match loaded {
        Ok(loaded) => match &loaded.source {
            crate::config::ConfigSource::Defaults => push(
                checks,
                "config.source",
                "configuracion",
                Status::Warn,
                "no hay archivo; se usan valores por defecto",
                Some("Crea una configuración con `basic config init`.".to_string()),
            ),
            crate::config::ConfigSource::File(path) => push(
                checks,
                "config.source",
                "configuracion",
                Status::Ok,
                format!("archivo: {}", path.display()),
                None,
            ),
        },
        Err(error) => push(
            checks,
            "config.source",
            "configuracion",
            Status::Fail,
            error.to_string(),
            Some("Corrige el TOML o recréalo con `basic config init --force`.".to_string()),
        ),
    }
}

fn check_config_values(checks: &mut Vec<Check>, loaded: &Result<LoadedConfig, BasicError>) {
    match loaded {
        Ok(loaded) => {
            let issues = loaded.validate();
            if issues.is_empty() {
                push(
                    checks,
                    "config.values",
                    "valores",
                    Status::Ok,
                    "configuración válida",
                    None,
                );
            } else {
                push(
                    checks,
                    "config.values",
                    "valores",
                    Status::Fail,
                    issues.join("; "),
                    Some("Corrige el archivo y vuelve a ejecutar `basic doctor`.".to_string()),
                );
            }
        }
        Err(_) => push(
            checks,
            "config.values",
            "valores",
            Status::Skip,
            "no evaluados porque la configuración no se pudo cargar",
            None,
        ),
    }
}

fn check_config_dir(checks: &mut Vec<Check>, paths: &Paths) {
    // La existencia del archivo ya la cubre `config.source`; aquí solo se
    // comprueba el directorio para no duplicar el mismo aviso.
    if paths.config_dir.is_dir() {
        push(
            checks,
            "paths.config",
            "config dir",
            Status::Ok,
            paths.config_dir.display().to_string(),
            None,
        );
    } else {
        push(
            checks,
            "paths.config",
            "config dir",
            Status::Skip,
            format!("no existe {}", paths.config_dir.display()),
            None,
        );
    }
}

fn check_writable_dir(
    checks: &mut Vec<Check>,
    id: &'static str,
    title: &'static str,
    directory: &Path,
    hint: &str,
) {
    match probe_writable(directory) {
        Ok(()) => push(
            checks,
            id,
            title,
            Status::Ok,
            directory.display().to_string(),
            None,
        ),
        Err(error) => push(
            checks,
            id,
            title,
            Status::Fail,
            error,
            Some(hint.to_string()),
        ),
    }
}

fn probe_writable(directory: &Path) -> Result<(), String> {
    fs::create_dir_all(directory)
        .map_err(|error| format!("no se pudo crear {}: {error}", directory.display()))?;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let probe = directory.join(format!(".basic-doctor-{}-{nanos}", std::process::id()));

    fs::write(&probe, b"basic")
        .map_err(|error| format!("no se pudo escribir en {}: {error}", directory.display()))?;
    fs::remove_file(&probe).map_err(|error| {
        format!(
            "no se pudo limpiar la prueba en {}: {error}",
            directory.display()
        )
    })
}

fn check_vault(checks: &mut Vec<Check>, config: &Config) {
    let vault = &config.vault;
    if vault.as_os_str().is_empty() {
        push(
            checks,
            "vault.exists",
            "vault",
            Status::Fail,
            "la ruta del vault está vacía",
            Some("Define `vault` en la configuración o BASIC_VAULT.".to_string()),
        );
        return;
    }

    match fs::metadata(vault) {
        Ok(metadata) if metadata.is_dir() => match fs::read_dir(vault) {
            Ok(mut entries) => {
                let _ = entries.next();
                push(
                    checks,
                    "vault.exists",
                    "vault",
                    Status::Ok,
                    format!("{} (directorio legible)", vault.display()),
                    None,
                );
            }
            Err(error) => push(
                checks,
                "vault.exists",
                "vault",
                Status::Fail,
                format!("{} no es legible: {error}", vault.display()),
                Some("Revisa permisos del vault.".to_string()),
            ),
        },
        Ok(_) => push(
            checks,
            "vault.exists",
            "vault",
            Status::Fail,
            format!("{} no es un directorio", vault.display()),
            Some("Apunta `vault` a la raíz del vault de Obsidian.".to_string()),
        ),
        Err(error) => push(
            checks,
            "vault.exists",
            "vault",
            Status::Fail,
            format!("{} no existe o no es accesible: {error}", vault.display()),
            Some("Ajusta `vault` o define BASIC_VAULT.".to_string()),
        ),
    }
}

#[derive(Debug, Default)]
struct VaultStats {
    files: usize,
    empty: usize,
    bytes: u64,
    categories: BTreeSet<String>,
    errors: usize,
    truncated: bool,
}

fn check_markdown(checks: &mut Vec<Check>, config: &Config) {
    if config.vault.as_os_str().is_empty() || !config.vault.is_dir() {
        push(
            checks,
            "vault.markdown",
            "markdown",
            Status::Skip,
            "no se puede contar sin un vault legible",
            None,
        );
        return;
    }

    match scan_markdown(config) {
        Ok(stats) => {
            let mut status = if stats.files == 0 {
                Status::Warn
            } else {
                Status::Ok
            };
            let mut detail = format!(
                "{} archivos · {} vacíos · {} · {} categorías",
                stats.files,
                stats.empty,
                human_bytes(stats.bytes),
                stats.categories.len()
            );
            if stats.errors > 0 {
                status = Status::Warn;
                detail.push_str(&format!(" · {} errores de lectura", stats.errors));
            }
            if stats.truncated {
                status = Status::Warn;
                detail.push_str(" · escaneo truncado");
            }

            let hint = if stats.files == 0 {
                Some("El vault no contiene Markdown; revisa la ruta.".to_string())
            } else if stats.errors > 0 {
                Some("Hay entradas que no se pudieron leer; revisa permisos.".to_string())
            } else {
                None
            };
            push(checks, "vault.markdown", "markdown", status, detail, hint);
        }
        Err(error) => push(
            checks,
            "vault.markdown",
            "markdown",
            Status::Fail,
            error,
            Some("Revisa permisos del vault.".to_string()),
        ),
    }
}

fn scan_markdown(config: &Config) -> Result<VaultStats, String> {
    let vault = &config.vault;
    let excludes = normalized_excludes(config);
    let walker = WalkDir::new(vault)
        .follow_links(config.scan.follow_symlinks)
        .into_iter()
        .filter_entry(|entry| !is_excluded(entry, vault, &excludes));

    let mut stats = VaultStats::default();

    for (visited, entry) in walker.enumerate() {
        if visited >= MAX_SCAN_ENTRIES {
            stats.truncated = true;
            break;
        }

        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                stats.errors += 1;
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }
        if !entry
            .path()
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        {
            continue;
        }

        stats.files += 1;
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => {
                stats.errors += 1;
                continue;
            }
        };
        stats.bytes += metadata.len();
        if metadata.len() == 0 {
            stats.empty += 1;
        }

        if let Ok(relative) = entry.path().strip_prefix(vault) {
            let mut components = relative.components();
            let first = components.next();
            let second = components.next();
            let category = match (first, second) {
                (Some(Component::Normal(name)), Some(_)) => name.to_string_lossy().into_owned(),
                _ => "(raíz)".to_string(),
            };
            stats.categories.insert(category);
        }
    }

    Ok(stats)
}

fn normalized_excludes(config: &Config) -> Vec<PathBuf> {
    config
        .scan
        .exclude
        .iter()
        .filter_map(|pattern| {
            let trimmed = pattern
                .trim()
                .trim_start_matches("./")
                .trim_end_matches('/');
            if trimmed.is_empty() {
                None
            } else {
                Some(PathBuf::from(trimmed))
            }
        })
        .collect()
}

fn is_excluded(entry: &DirEntry, vault: &Path, excludes: &[PathBuf]) -> bool {
    if entry.depth() == 0 {
        return false;
    }

    // Directorios de máquina: nunca forman parte del conocimiento.
    let name = entry.file_name();
    if name == ".git" || name == ".obsidian" {
        return true;
    }

    let Ok(relative) = entry.path().strip_prefix(vault) else {
        return false;
    };
    excludes
        .iter()
        .any(|excluded| relative == excluded || relative.starts_with(excluded))
}

fn check_exclusions(checks: &mut Vec<Check>, config: &Config) {
    let excludes_tui = config
        .scan
        .exclude
        .iter()
        .any(|pattern| is_tui_exclusion(pattern));

    if excludes_tui {
        push(
            checks,
            "vault.exclusions",
            "exclusiones",
            Status::Ok,
            "tui/ y su target quedan fuera del indexado",
            None,
        );
    } else {
        push(
            checks,
            "vault.exclusions",
            "exclusiones",
            Status::Warn,
            "scan.exclude no excluye el paquete Rust",
            Some("Añade `tui` a `scan.exclude` para no indexar el código.".to_string()),
        );
    }
}

fn is_tui_exclusion(pattern: &str) -> bool {
    let normalized = pattern
        .trim()
        .trim_start_matches("./")
        .trim_end_matches('/');
    normalized == "tui" || normalized.starts_with("tui/")
}

fn check_database_location(checks: &mut Vec<Check>, config: &Config) {
    let database = &config.database;
    if database.as_os_str().is_empty() {
        push(
            checks,
            "database.location",
            "base de datos",
            Status::Fail,
            "la ruta de la base de datos está vacía",
            Some("Define `database` o BASIC_DB.".to_string()),
        );
        return;
    }

    if !config.vault.as_os_str().is_empty() && database.starts_with(&config.vault) {
        push(
            checks,
            "database.location",
            "base de datos",
            Status::Fail,
            format!("{} está dentro del vault", database.display()),
            Some("Mueve la base fuera del vault para no interferir con Obsidian Sync.".to_string()),
        );
    } else {
        push(
            checks,
            "database.location",
            "base de datos",
            Status::Ok,
            format!("{} (fuera del vault)", database.display()),
            None,
        );
    }
}

fn check_migrations(checks: &mut Vec<Check>) {
    match validate_schema_in_memory() {
        Ok(detail) => push(
            checks,
            "database.migrations",
            "migraciones",
            Status::Ok,
            detail,
            None,
        ),
        Err(error) => push(
            checks,
            "database.migrations",
            "migraciones",
            Status::Fail,
            error,
            Some("Recompila con `rusqlite` bundled (incluye FTS5).".to_string()),
        ),
    }
}

fn validate_schema_in_memory() -> Result<String, String> {
    let database = Database::open_in_memory().map_err(|error| error.to_string())?;

    let version = database
        .schema_version()
        .map_err(|error| error.to_string())?;
    if version != LATEST_SCHEMA_VERSION {
        return Err(format!(
            "el esquema recién creado es v{version}, se esperaba v{LATEST_SCHEMA_VERSION}"
        ));
    }

    if !database
        .fts5_compiled()
        .map_err(|error| error.to_string())?
    {
        return Err("SQLite se compiló sin FTS5".to_string());
    }

    let mut missing = Vec::new();
    for table in EXPECTED_TABLES {
        if !database
            .has_table(table)
            .map_err(|error| error.to_string())?
        {
            missing.push(*table);
        }
    }
    if !missing.is_empty() {
        return Err(format!("faltan tablas: {}", missing.join(", ")));
    }

    database
        .fts_document_count()
        .map_err(|error| format!("el índice FTS5 no responde: {error}"))?;
    database
        .connection()
        .query_row(
            "SELECT count(*) FROM documents_fts WHERE documents_fts MATCH 'basic'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| format!("la consulta FTS5 falla: {error}"))?;

    let violations = database
        .foreign_key_violations()
        .map_err(|error| error.to_string())?;
    if violations > 0 {
        return Err(format!(
            "el esquema recién creado tiene {violations} violaciones de claves foráneas"
        ));
    }

    let integrity = database.quick_check().map_err(|error| error.to_string())?;
    if integrity != "ok" {
        return Err(format!("quick_check devolvió: {integrity}"));
    }

    Ok(format!(
        "esquema v{version} · {} tablas · FTS5 disponible",
        EXPECTED_TABLES.len()
    ))
}

fn check_database_file(checks: &mut Vec<Check>, paths: &Paths) {
    let path = &paths.database;
    if !path.exists() {
        // F1 todavía no crea la base; que falte no es un problema de entorno.
        // Se omite para que `--strict` sea utilizable antes de F3.
        push(
            checks,
            "database.existing",
            "db existente",
            Status::Skip,
            format!("{} aún no existe", path.display()),
            Some("Se creará automáticamente cuando exista la capa de datos (F3).".to_string()),
        );
        return;
    }

    let size = fs::metadata(path)
        .map(|metadata| human_bytes(metadata.len()))
        .unwrap_or_else(|_| "tamaño desconocido".to_string());

    let database = match Database::open_read_only(path) {
        Ok(database) => database,
        Err(error) => {
            push(
                checks,
                "database.existing",
                "db existente",
                Status::Fail,
                format!("{} ({size}) no se pudo abrir: {error}", path.display()),
                Some("Comprueba permisos o mueve la base dañada a un lado.".to_string()),
            );
            return;
        }
    };

    let version = database
        .schema_version()
        .map(|version| version as i64)
        .unwrap_or(-1);
    let integrity = database
        .quick_check()
        .unwrap_or_else(|error| format!("error: {error}"));
    let journal = database
        .journal_mode()
        .map(|mode| mode.to_lowercase())
        .unwrap_or_else(|_| "desconocido".to_string());
    let detail = format!(
        "{} · {size} · esquema v{version} · journal {journal}",
        path.display()
    );

    if version > i64::from(LATEST_SCHEMA_VERSION) {
        push(
            checks,
            "database.existing",
            "db existente",
            Status::Fail,
            detail,
            Some("La base es más nueva que este binario; actualiza basic.".to_string()),
        );
    } else if integrity != "ok" {
        push(
            checks,
            "database.existing",
            "db existente",
            Status::Fail,
            format!("{detail} · integridad: {integrity}"),
            Some("Haz una copia y considera reconstruir el índice.".to_string()),
        );
    } else if version < i64::from(LATEST_SCHEMA_VERSION) {
        push(
            checks,
            "database.existing",
            "db existente",
            Status::Warn,
            detail,
            Some("Hay migraciones pendientes; se aplicarán al abrir la base.".to_string()),
        );
    } else {
        push(
            checks,
            "database.existing",
            "db existente",
            Status::Ok,
            detail,
            None,
        );
    }
}

fn check_logging(checks: &mut Vec<Check>, paths: &Paths) {
    let log = &paths.log_file;
    let Some(parent) = log.parent() else {
        push(
            checks,
            "logging",
            "logging",
            Status::Fail,
            "el archivo de log no tiene directorio padre",
            None,
        );
        return;
    };

    if let Err(error) = probe_writable(parent) {
        push(
            checks,
            "logging",
            "logging",
            Status::Fail,
            error,
            Some("Define BASIC_STATE_DIR en un directorio escribible.".to_string()),
        );
        return;
    }

    match fs::metadata(log) {
        Ok(metadata) if metadata.len() > MAX_LOG_BYTES => push(
            checks,
            "logging",
            "logging",
            Status::Warn,
            format!("{} · {}", log.display(), human_bytes(metadata.len())),
            Some("Se rotará a basic.log.1 en el próximo arranque.".to_string()),
        ),
        Ok(metadata) => push(
            checks,
            "logging",
            "logging",
            Status::Ok,
            format!("{} · {}", log.display(), human_bytes(metadata.len())),
            None,
        ),
        Err(_) => push(
            checks,
            "logging",
            "logging",
            Status::Ok,
            format!("{} se creará al iniciar", log.display()),
            None,
        ),
    }
}

fn check_editor(checks: &mut Vec<Check>, config: &Config) {
    let editor = config.editor.trim();
    if editor.is_empty() {
        push(
            checks,
            "editor",
            "editor",
            Status::Fail,
            "editor está vacío",
            Some("Define `editor` o la variable EDITOR.".to_string()),
        );
        return;
    }

    let program = editor.split_whitespace().next().unwrap_or(editor);
    match resolve_program(program) {
        Some(path) => push(
            checks,
            "editor",
            "editor",
            Status::Ok,
            format!("{editor} → {}", path.display()),
            None,
        ),
        None => push(
            checks,
            "editor",
            "editor",
            Status::Warn,
            format!("`{program}` no se encontró en PATH"),
            Some("Solo es necesario para editar notas desde basic (F6).".to_string()),
        ),
    }
}

fn resolve_program(program: &str) -> Option<PathBuf> {
    let candidate = Path::new(program);
    if candidate.is_absolute() || candidate.components().count() > 1 {
        return candidate.is_file().then(|| candidate.to_path_buf());
    }

    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join(program))
        .find(|candidate| is_executable(candidate))
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn check_obsidian(checks: &mut Vec<Check>, config: &Config) {
    let directory = config.vault.join(".obsidian");
    if !directory.is_dir() {
        push(
            checks,
            "obsidian",
            "obsidian",
            Status::Skip,
            "no se detectó un vault de Obsidian",
            None,
        );
        return;
    }

    let app_json = directory.join("app.json");
    let text = match fs::read_to_string(&app_json) {
        Ok(text) => text,
        Err(error) => {
            push(
                checks,
                "obsidian",
                "obsidian",
                Status::Warn,
                format!("no se pudo leer {}: {error}", app_json.display()),
                Some("Añade \"tui/\" a Archivos excluidos en Obsidian.".to_string()),
            );
            return;
        }
    };

    let value: serde_json::Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            push(
                checks,
                "obsidian",
                "obsidian",
                Status::Warn,
                format!("{} no es JSON válido: {error}", app_json.display()),
                Some("Añade \"tui/\" a Archivos excluidos en Obsidian.".to_string()),
            );
            return;
        }
    };

    let excluded = value
        .get("userIgnoreFilters")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|filters| {
            filters
                .iter()
                .filter_map(serde_json::Value::as_str)
                .any(is_tui_exclusion)
        });

    if excluded {
        push(
            checks,
            "obsidian",
            "obsidian",
            Status::Ok,
            "tui/ está excluido en Obsidian",
            None,
        );
    } else {
        push(
            checks,
            "obsidian",
            "obsidian",
            Status::Warn,
            "tui/ no está excluido en Obsidian",
            Some(
                "Añade \"tui/\" en Ajustes → Archivos y enlaces → Archivos excluidos.".to_string(),
            ),
        );
    }
}

fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KiB", "MiB", "GiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use tempfile::TempDir;

    use super::*;
    use crate::config::{ConfigSource, LoadedConfig};

    fn fixture(temp: &TempDir) -> (Paths, LoadedConfig, PathBuf) {
        let paths = Paths::from_dirs(
            temp.path(),
            &temp.path().join("config"),
            &temp.path().join("data"),
            &temp.path().join("state"),
            |_| None::<OsString>,
        );
        let vault = temp.path().join("vault");
        fs::create_dir_all(&vault).unwrap();

        let config = Config {
            vault: vault.clone(),
            database: paths.database.clone(),
            editor: std::env::current_exe().unwrap().display().to_string(),
            ..Config::default()
        };

        let loaded = LoadedConfig {
            config,
            source: ConfigSource::Defaults,
        };
        (paths, loaded, vault)
    }

    fn find<'a>(report: &'a Report, id: &str) -> &'a Check {
        report
            .checks
            .iter()
            .find(|check| check.id == id)
            .unwrap_or_else(|| panic!("no existe la comprobación {id}"))
    }

    #[test]
    fn reports_a_healthy_minimal_vault() {
        let temp = TempDir::new().unwrap();
        let (paths, loaded, vault) = fixture(&temp);
        fs::write(vault.join("nota.md"), "# Nota\n").unwrap();
        fs::write(vault.join("vacia.md"), "").unwrap();

        let nested = vault.join("linux/vim");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("atajos.md"), "# Atajos\n").unwrap();

        // Debe quedar fuera del recuento.
        fs::create_dir_all(vault.join("tui")).unwrap();
        fs::write(vault.join("tui/README.md"), "# TUI\n").unwrap();
        fs::create_dir_all(vault.join(".git")).unwrap();
        fs::write(vault.join(".git/hidden.md"), "# Git\n").unwrap();

        let report = run(&paths, &Ok(loaded));
        assert_eq!(report.summary.fail, 0, "{:#?}", report.checks);
        assert!(report.summary.healthy);

        let markdown = find(&report, "vault.markdown");
        assert_eq!(markdown.status, Status::Ok);
        assert!(
            markdown.detail.contains("3 archivos"),
            "{}",
            markdown.detail
        );
        assert!(markdown.detail.contains("1 vacíos"), "{}", markdown.detail);
        assert!(
            markdown.detail.contains("2 categorías"),
            "{}",
            markdown.detail
        );

        assert_eq!(find(&report, "vault.exclusions").status, Status::Ok);
        assert_eq!(find(&report, "database.location").status, Status::Ok);
        assert_eq!(find(&report, "database.migrations").status, Status::Ok);
        assert_eq!(find(&report, "database.existing").status, Status::Skip);
        assert_eq!(find(&report, "obsidian").status, Status::Skip);
    }

    #[test]
    fn fails_when_database_is_inside_the_vault() {
        let temp = TempDir::new().unwrap();
        let (paths, mut loaded, vault) = fixture(&temp);
        loaded.config.database = vault.join("basic.db");

        let report = run(&paths, &Ok(loaded));
        let check = find(&report, "database.location");
        assert_eq!(check.status, Status::Fail);
        assert!(check.detail.contains("dentro del vault"));
        assert_eq!(report.exit_code(false), 1);
    }

    #[test]
    fn config_errors_are_visible_without_panicking() {
        let temp = TempDir::new().unwrap();
        let (paths, _, _) = fixture(&temp);
        let error = BasicError::InvalidConfig {
            path: paths.config_file.clone(),
            message: "vault inválido".to_string(),
        };

        let report = run(&paths, &Err(error));
        assert_eq!(find(&report, "config.source").status, Status::Fail);
        assert_eq!(find(&report, "config.values").status, Status::Skip);
        assert_eq!(report.exit_code(false), 1);
    }

    #[test]
    fn warns_when_obsidian_does_not_exclude_tui() {
        let temp = TempDir::new().unwrap();
        let (paths, loaded, vault) = fixture(&temp);
        let obsidian = vault.join(".obsidian");
        fs::create_dir_all(&obsidian).unwrap();
        fs::write(obsidian.join("app.json"), "{}").unwrap();

        let report = run(&paths, &Ok(loaded));
        let check = find(&report, "obsidian");
        assert_eq!(check.status, Status::Warn);
        assert!(check.hint.as_ref().unwrap().contains("tui/"));
    }

    #[test]
    fn accepts_obsidian_exclusion() {
        let temp = TempDir::new().unwrap();
        let (paths, loaded, vault) = fixture(&temp);
        let obsidian = vault.join(".obsidian");
        fs::create_dir_all(&obsidian).unwrap();
        fs::write(
            obsidian.join("app.json"),
            r#"{"userIgnoreFilters": ["tui/"]}"#,
        )
        .unwrap();

        let report = run(&paths, &Ok(loaded));
        assert_eq!(find(&report, "obsidian").status, Status::Ok);
    }

    #[test]
    fn strict_mode_turns_warnings_into_failure() {
        let temp = TempDir::new().unwrap();
        let (paths, loaded, vault) = fixture(&temp);
        fs::write(vault.join("nota.md"), "# Nota\n").unwrap();

        let report = run(&paths, &Ok(loaded));
        assert_eq!(report.summary.fail, 0);
        assert!(report.summary.warn > 0);
        assert_eq!(report.exit_code(false), 0);
        assert_eq!(report.exit_code(true), 1);
    }

    #[test]
    fn renders_a_readable_report() {
        let temp = TempDir::new().unwrap();
        let (paths, loaded, vault) = fixture(&temp);
        fs::write(vault.join("nota.md"), "# Nota\n").unwrap();

        let report = run(&paths, &Ok(loaded));

        let rendered = report.render(false);
        assert!(rendered.contains("BASIC"));
        assert!(rendered.contains("Resumen:"));
        assert!(rendered.contains("markdown"));
        assert!(rendered.contains("correcto con avisos"));

        let strict = report.render(true);
        assert!(strict.contains("fallo por avisos (--strict)"));
    }

    #[test]
    fn human_bytes_is_stable() {
        assert_eq!(human_bytes(0), "0 B");
        assert_eq!(human_bytes(1023), "1023 B");
        assert_eq!(human_bytes(1024), "1.0 KiB");
        assert_eq!(human_bytes(1024 * 1024), "1.0 MiB");
    }
}
