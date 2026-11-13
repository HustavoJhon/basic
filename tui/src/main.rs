use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, bail};
use chrono::Utc;
use clap::{CommandFactory, Parser};
use serde::Serialize;

use basic::cli::{
    Cli, Command, ConfigCommand, ConfigInitArgs, ConfigShowArgs, DoctorArgs, PathsArgs,
};
use basic::config::{Config, LoadedConfig};
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
        None => {
            Cli::command().print_help()?;
            println!();
            Ok(0)
        }
    }
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
