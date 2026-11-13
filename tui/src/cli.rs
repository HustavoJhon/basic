use clap::{Args, Parser, Subcommand};

/// BASIC — conocimiento y aprendizaje desde la terminal.
#[derive(Debug, Parser)]
#[command(
    name = "basic",
    version,
    about = "Sistema personal de conocimiento y aprendizaje para la terminal",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Diagnostica configuración, vault, base de datos y entorno
    Doctor(DoctorArgs),
    /// Muestra las rutas XDG resueltas
    Paths(PathsArgs),
    /// Gestiona la configuración TOML
    Config(ConfigArgs),
}

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Emite el informe como JSON
    #[arg(long)]
    pub json: bool,
    /// Trata los avisos como fallos (útil en CI)
    #[arg(long)]
    pub strict: bool,
}

#[derive(Debug, Args)]
pub struct PathsArgs {
    /// Emite las rutas como JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Crea el archivo de configuración
    Init(ConfigInitArgs),
    /// Muestra la configuración efectiva
    Show(ConfigShowArgs),
    /// Imprime la ruta del archivo de configuración
    Path,
}

#[derive(Debug, Args)]
pub struct ConfigInitArgs {
    /// Sobrescribe un archivo existente
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct ConfigShowArgs {
    /// Emite la configuración como JSON
    #[arg(long)]
    pub json: bool,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn doctor_accepts_flags() {
        let cli = Cli::try_parse_from(["basic", "doctor", "--json", "--strict"]).unwrap();
        match cli.command {
            Some(Command::Doctor(args)) => {
                assert!(args.json);
                assert!(args.strict);
            }
            other => panic!("subcomando inesperado: {other:?}"),
        }
    }

    #[test]
    fn config_requires_a_subcommand() {
        assert!(Cli::try_parse_from(["basic", "config"]).is_err());
    }
}
