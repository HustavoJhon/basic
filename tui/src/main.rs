use std::ffi::OsStr;
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, bail};
use chrono::Utc;
use clap::{CommandFactory, Parser};
use serde::Serialize;

use basic::cli::{
    Cli, CodeLabCommand, CodeLabLessonsArgs, CodeLabSeedArgs, Command, ConfigCommand,
    ConfigInitArgs, ConfigShowArgs, DoctorArgs, IndexArgs, NewNoteArgs, PathsArgs, ReviewArgs,
    SearchArgs, StatsArgs,
};
use basic::code_lab::app::CodeLabService;
use basic::config::{Config, LoadedConfig};
use basic::domain::{ActivityDay, DashboardStats, SearchScope};
use basic::infra::catalog;
use basic::infra::index::{self, IndexReport};
use basic::infra::notes;
use basic::infra::sqlite::Database;
use basic::ui;
use basic::{BasicError, doctor, logging, paths::Paths};

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> anyhow::Result<u8> {
    let cli = Cli::parse();
    let paths = Paths::resolve().context("no se pudieron resolver las rutas XDG")?;

    if let Err(error) = logging::init(&paths.log_file) {
        eprintln!(
            "aviso: no se pudo inicializar el registro en {}: {error}",
            paths.log_file.display()
        );
    }
    tracing::debug!(?paths, "rutas resueltas");

    match cli.command {
        Some(Command::Tui) => run_tui(&paths),
        Some(Command::Doctor(args)) => run_doctor(&paths, args),
        Some(Command::Paths(args)) => run_paths(&paths, args),
        Some(Command::Config(args)) => match args.command {
            ConfigCommand::Init(args) => run_config_init(&paths, args),
            ConfigCommand::Show(args) => run_config_show(&paths, args),
            ConfigCommand::Path => {
                println!("{}", paths.config_file.display());
                Ok(0)
            }
        },
        Some(Command::Index(args)) => run_index(&paths, args),
        Some(Command::Search(args)) => run_search(&paths, args),
        Some(Command::Stats(args)) => run_stats(&paths, args),
        Some(Command::New(args)) => run_new(&paths, args),
        Some(Command::Review(args)) => run_review(&paths, args),
        Some(Command::CodeLab(args)) => match args.command {
            CodeLabCommand::Seed(args) => run_code_lab_seed(&paths, args),
            CodeLabCommand::Lessons(args) => run_code_lab_lessons(&paths, args),
        },
        None => {
            // En una terminal interactiva el comando principal es la TUI; en
            // scripts y CI se imprime la ayuda para no bloquear.
            if std::io::stdout().is_terminal() && std::io::stdin().is_terminal() {
                run_tui(&paths)
            } else {
                Cli::command().print_help()?;
                println!();
                Ok(0)
            }
        }
    }
}

fn run_tui(paths: &Paths) -> anyhow::Result<u8> {
    if !std::io::stdout().is_terminal() || !std::io::stdin().is_terminal() {
        bail!("la TUI requiere una terminal interactiva; usa los subcomandos en scripts");
    }
    let loaded = load_valid(paths)?;
    if let Some(report) = ensure_index(&loaded.config)? {
        tracing::info!(summary = %report.summary(), "índice inicial");
    }
    let database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    ui::run(loaded.config, database).map(|()| 0)
}

/// Abre el catálogo y lo indexa si está vacío.
fn ensure_index(config: &Config) -> anyhow::Result<Option<IndexReport>> {
    let mut database = Database::open(&config.database)
        .with_context(|| format!("no se pudo abrir {}", config.database.display()))?;
    let count: i64 =
        database
            .connection()
            .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(None);
    }
    Ok(Some(index::index_vault(&mut database, config)?))
}

fn run_index(paths: &Paths, args: IndexArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    let mut database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    let report = index::index_vault(&mut database, &loaded.config)?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("{}", report.summary());
        for error in &report.errors {
            eprintln!("aviso: {error}");
        }
    }
    Ok(0)
}

fn run_search(paths: &Paths, args: SearchArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    ensure_index(&loaded.config)?;
    let database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    let query = args.query.join(" ");
    let scope = SearchScope::from(args.scope);
    let mut documents = catalog::search_documents(&database, &query, scope)?;
    documents.truncate(args.limit);

    if args.json {
        println!("{}", serde_json::to_string_pretty(&documents)?);
    } else if documents.is_empty() {
        println!("Sin resultados para «{query}».");
    } else {
        for document in &documents {
            println!("{}", document.title);
            println!("  {}", document.path);
            if !document.excerpt.is_empty() {
                println!("  {}", document.excerpt);
            }
        }
        println!("\n{} resultado(s).", documents.len());
    }
    Ok(0)
}

fn run_stats(paths: &Paths, args: StatsArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    ensure_index(&loaded.config)?;
    let database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    let stats = catalog::dashboard_stats(&database)?;
    let activity = catalog::activity_days(&database, args.days)?;

    if args.json {
        #[derive(Serialize)]
        struct Output<'a> {
            stats: &'a DashboardStats,
            activity: &'a [ActivityDay],
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&Output {
                stats: &stats,
                activity: &activity,
            })?
        );
    } else {
        println!("Documentos       {}", stats.total);
        println!("Pendientes       {}", stats.pending);
        println!("En curso         {}", stats.in_progress);
        println!("Para repasar     {}", stats.due);
        println!("Sin contenido    {}", stats.stubs);
        println!(
            "Estudio hoy      {}",
            basic::domain::format_duration(stats.study_seconds_today)
        );
        println!("Pomodoros hoy    {}", stats.pomodoros_today);
        println!("Racha            {} días", stats.streak_days);
        if !stats.categories.is_empty() {
            println!("\nCategorías:");
            for (category, count) in &stats.categories {
                println!("  {category:<24} {count:>4}");
            }
        }
    }
    Ok(0)
}

fn run_new(paths: &Paths, args: NewNoteArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    let created = notes::create_note(&loaded.config, &args.path, args.title.as_deref())?;
    let report = if args.no_index {
        None
    } else {
        let mut database = Database::open(&loaded.config.database)
            .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
        Some(index::index_vault(&mut database, &loaded.config)?)
    };

    if args.json {
        #[derive(Serialize)]
        struct Output<'a> {
            path: &'a str,
            relative: &'a str,
            index: Option<&'a IndexReport>,
        }
        let relative = created.path.to_string_lossy();
        println!(
            "{}",
            serde_json::to_string_pretty(&Output {
                path: &relative,
                relative: &created.relative,
                index: report.as_ref(),
            })?
        );
    } else {
        println!("Nota creada: {}", created.path.display());
        if let Some(report) = report {
            println!("{}", report.summary());
        }
    }
    Ok(0)
}

fn run_review(paths: &Paths, args: ReviewArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    ensure_index(&loaded.config)?;
    let database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    let documents = catalog::due_documents(&database)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&documents)?);
    } else if documents.is_empty() {
        println!("No hay repasos pendientes.");
    } else {
        for document in &documents {
            let due = document
                .review_due_at
                .map(catalog::format_date)
                .unwrap_or_else(|| "-".to_string());
            println!("{}  ·  {}  ·  vence {}", document.title, document.path, due);
        }
        println!("\n{} repaso(s) pendiente(s).", documents.len());
    }
    Ok(0)
}

fn run_code_lab_seed(paths: &Paths, args: CodeLabSeedArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    let mut database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    let report = CodeLabService::new(&mut database).seed()?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("Idiomas          {}", report.languages);
        println!("Rutas            {}", report.paths);
        println!("Lecciones        {}", report.lessons);
        println!("Preguntas        {}", report.questions);
        println!("Ejercicios       {}", report.exercises);
    }
    Ok(0)
}

fn run_code_lab_lessons(paths: &Paths, args: CodeLabLessonsArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;
    let mut database = Database::open(&loaded.config.database)
        .with_context(|| format!("no se pudo abrir {}", loaded.config.database.display()))?;
    let mut service = CodeLabService::new(&mut database);
    service.seed()?;
    let lessons = service.lessons()?;

    if args.json {
        #[derive(Serialize)]
        struct Output<'a> {
            id: i64,
            slug: &'a str,
            title: &'a str,
            summary: &'a str,
            concepts: &'a [String],
            order: i32,
            minutes: i32,
        }
        let output: Vec<Output<'_>> = lessons
            .iter()
            .map(|lesson| Output {
                id: lesson.id,
                slug: &lesson.slug,
                title: &lesson.title,
                summary: &lesson.summary,
                concepts: &lesson.concepts,
                order: lesson.order_index,
                minutes: lesson.estimated_minutes,
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        for lesson in &lessons {
            println!(
                "{:02}. {} · {} min · {}",
                lesson.order_index, lesson.title, lesson.estimated_minutes, lesson.slug
            );
            println!("     {}", lesson.summary);
        }
        println!("\n{} lección(es) disponibles.", lessons.len());
    }
    Ok(0)
}

fn run_doctor(paths: &Paths, args: DoctorArgs) -> anyhow::Result<u8> {
    let loaded = Config::load(paths);
    match &loaded {
        Ok(loaded) => tracing::info!(
            source = %loaded.source.describe(),
            vault = %loaded.config.vault.display(),
            database = %loaded.config.database.display(),
            "configuración cargada"
        ),
        Err(error) => tracing::error!(%error, "no se pudo cargar la configuración"),
    }

    let report = doctor::run(paths, &loaded);
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print!("{}", report.render(args.strict));
    }

    Ok(report.exit_code(args.strict) as u8)
}

fn run_paths(paths: &Paths, args: PathsArgs) -> anyhow::Result<u8> {
    if args.json {
        println!("{}", serde_json::to_string_pretty(paths)?);
    } else {
        println!("home_dir    = {}", paths.home_dir.display());
        println!("config_dir  = {}", paths.config_dir.display());
        println!("config_file = {}", paths.config_file.display());
        println!("data_dir    = {}", paths.data_dir.display());
        println!("state_dir   = {}", paths.state_dir.display());
        println!("database    = {}", paths.database.display());
        println!("log_file    = {}", paths.log_file.display());
    }
    Ok(0)
}

fn run_config_init(paths: &Paths, args: ConfigInitArgs) -> anyhow::Result<u8> {
    let exists = paths.config_file.exists();
    if exists && !args.force {
        bail!(
            "{} ya existe; usa `basic config init --force` para sobrescribirlo",
            paths.config_file.display()
        );
    }

    // Sobrescribir sin copia de seguridad puede perder ajustes del usuario.
    if exists {
        let backup = backup_config(&paths.config_file)?;
        println!("Copia de seguridad: {}", backup.display());
    }

    // `--force` reescribe la plantilla. Si el archivo anterior es válido se
    // conservan sus valores para no cambiar el vault ni el resto de ajustes
    // por el simple hecho de ejecutar el comando desde otro directorio.
    let (config, preserved) = if exists {
        match Config::load(paths) {
            Ok(loaded) if loaded.validate().is_empty() => (loaded.config, true),
            Ok(_) => {
                eprintln!(
                    "aviso: la configuración existente no es válida; se usarán valores por defecto"
                );
                (Config::default_for(paths), false)
            }
            Err(error) => {
                eprintln!(
                    "aviso: no se pudo leer la configuración existente ({error}); se usarán valores por defecto"
                );
                (Config::default_for(paths), false)
            }
        }
    } else {
        (Config::default_for(paths), false)
    };

    let issues = config.validate();
    if !issues.is_empty() {
        bail!(
            "no se puede escribir una configuración inválida: {}",
            issues.join("; ")
        );
    }

    config
        .save(&paths.config_file)
        .with_context(|| format!("no se pudo crear {}", paths.config_file.display()))?;
    println!(
        "Configuración {} en {}\nVault: {}",
        if preserved { "actualizada" } else { "creada" },
        paths.config_file.display(),
        config.vault.display()
    );
    if preserved {
        println!("Se conservaron los valores existentes.");
    }
    Ok(0)
}

/// Copia el archivo de configuración a un nombre con marca de tiempo.
fn backup_config(path: &Path) -> anyhow::Result<PathBuf> {
    let name = path
        .file_name()
        .unwrap_or_else(|| OsStr::new("config.toml"))
        .to_string_lossy();
    let stamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let mut backup = path.with_file_name(format!("{name}.bak-{stamp}"));
    let mut suffix = 1;
    while backup.exists() {
        backup = path.with_file_name(format!("{name}.bak-{stamp}-{suffix}"));
        suffix += 1;
    }

    fs::copy(path, &backup).with_context(|| {
        format!(
            "no se pudo crear la copia de seguridad {}",
            backup.display()
        )
    })?;
    Ok(backup)
}

fn run_config_show(paths: &Paths, args: ConfigShowArgs) -> anyhow::Result<u8> {
    let loaded = load_valid(paths)?;

    if args.json {
        #[derive(Serialize)]
        struct Output<'a> {
            source: &'a str,
            config: &'a Config,
        }

        let source = loaded.source.describe();
        println!(
            "{}",
            serde_json::to_string_pretty(&Output {
                source: &source,
                config: &loaded.config,
            })?
        );
    } else {
        println!("# fuente: {}", loaded.source.describe());
        print!("{}", loaded.config.to_toml()?);
    }

    Ok(0)
}

fn load_valid(paths: &Paths) -> anyhow::Result<LoadedConfig> {
    let loaded = Config::load(paths)?;
    let issues = loaded.validate();
    if !issues.is_empty() {
        return Err(BasicError::InvalidConfig {
            path: paths.config_file.clone(),
            message: issues.join("; "),
        }
        .into());
    }
    Ok(loaded)
}
