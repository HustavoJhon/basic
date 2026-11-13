use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::domain::SearchScope;

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
    /// Abre la interfaz de terminal (por defecto en una TTY)
    Tui,
    /// Diagnostica configuración, vault, base de datos y entorno
    Doctor(DoctorArgs),
    /// Muestra las rutas XDG resueltas
    Paths(PathsArgs),
    /// Gestiona la configuración TOML
    Config(ConfigArgs),
    /// Escanea el vault y actualiza el índice SQLite
    Index(IndexArgs),
    /// Busca documentos por título, contenido, ruta, categoría o código
    Search(SearchArgs),
    /// Muestra estadísticas del catálogo y la actividad
    Stats(StatsArgs),
    /// Crea una nota nueva sin sobrescribir archivos existentes
    New(NewNoteArgs),
    /// Lista los documentos pendientes de repaso
    Review(ReviewArgs),
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

#[derive(Debug, Args)]
pub struct IndexArgs {
    /// Emite el informe como JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Términos de búsqueda (si se omiten, lista los documentos)
    #[arg(trailing_var_arg = true)]
    pub query: Vec<String>,
    /// Campo donde buscar
    #[arg(long, value_enum, default_value_t = ScopeArg::All)]
    pub scope: ScopeArg,
    /// Número máximo de resultados
    #[arg(long, default_value_t = 30)]
    pub limit: usize,
    /// Emite los resultados como JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ScopeArg {
    All,
    Title,
    Content,
    Path,
    Category,
    Code,
}

impl From<ScopeArg> for SearchScope {
    fn from(value: ScopeArg) -> Self {
        match value {
            ScopeArg::All => Self::All,
            ScopeArg::Title => Self::Title,
            ScopeArg::Content => Self::Content,
            ScopeArg::Path => Self::Path,
            ScopeArg::Category => Self::Category,
            ScopeArg::Code => Self::Code,
        }
    }
}

#[derive(Debug, Args)]
pub struct StatsArgs {
    /// Emite las estadísticas como JSON
    #[arg(long)]
    pub json: bool,
    /// Días de actividad que se incluyen en el heatmap
    #[arg(long, default_value_t = 365)]
    pub days: u32,
}

#[derive(Debug, Args)]
pub struct NewNoteArgs {
    /// Ruta relativa al vault, con o sin extensión `.md`
    pub path: String,
    /// Título de la nota (por defecto, derivado del nombre)
    #[arg(long)]
    pub title: Option<String>,
    /// No indexar el vault después de crearla
    #[arg(long)]
    pub no_index: bool,
    /// Emite el resultado como JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct ReviewArgs {
    /// Emite los documentos como JSON
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

    #[test]
    fn search_parses_scope_and_query() {
        let cli = Cli::try_parse_from([
            "basic",
            "search",
            "--scope",
            "content",
            "préstamo",
            "de",
            "memoria",
        ])
        .unwrap();
        match cli.command {
            Some(Command::Search(args)) => {
                assert_eq!(args.query, ["préstamo", "de", "memoria"]);
                assert!(matches!(args.scope, ScopeArg::Content));
            }
            other => panic!("subcomando inesperado: {other:?}"),
        }
    }

    #[test]
    fn new_accepts_a_title() {
        let cli = Cli::try_parse_from(["basic", "new", "linux/vim", "--title", "Atajos"]).unwrap();
        match cli.command {
            Some(Command::New(args)) => {
                assert_eq!(args.path, "linux/vim");
                assert_eq!(args.title.as_deref(), Some("Atajos"));
            }
            other => panic!("subcomando inesperado: {other:?}"),
        }
    }
}
