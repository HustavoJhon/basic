//! Adaptadores de ejecución de Code Lab.
//!
//! Un runner recibe código, casos de prueba y un espacio de trabajo; devuelve
//! trazas y resultados sin conocer la UI ni la base de datos. Añadir un
//! lenguaje significa implementar [`LanguageRunner`] y registrarlo en
//! [`runner_for`], sin tocar el editor ni el progreso.

mod csharp;
mod process;
mod workspace;

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::code_lab::domain::{AttemptStatus, ExecutionResult, TestSpec};
use crate::error::BasicError;

pub use csharp::CSharpRunner;
pub use workspace::Workspace;

/// Tiempo máximo por defecto para cada fase y caso de prueba.
pub const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Resultado de detectar el SDK de un lenguaje.
#[derive(Debug, Clone)]
pub struct Detection {
    pub version: Option<String>,
    pub execution: ExecutionResult,
}

impl Detection {
    pub fn available(&self) -> bool {
        self.version.is_some()
    }
}

/// Solicitud de ejecución independiente del lenguaje.
#[derive(Debug, Clone)]
pub struct RunRequest {
    pub exercise_id: i64,
    pub language: String,
    pub code: String,
    pub test_spec: TestSpec,
    pub timeout_secs: u64,
    /// Raíz del vault. Se usa para rechazar espacios de trabajo que caigan
    /// dentro de él.
    pub vault: PathBuf,
}

impl RunRequest {
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.timeout_secs.max(1))
    }
}

/// Resultado de un caso de prueba.
#[derive(Debug, Clone)]
pub struct CaseResult {
    pub name: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub duration_ms: u64,
}

/// Resultado agregado de una ejecución.
#[derive(Debug, Clone)]
pub struct RunOutcome {
    pub status: AttemptStatus,
    pub sdk_version: Option<String>,
    pub compile_ok: Option<bool>,
    pub tests_passed: i32,
    pub tests_total: i32,
    pub duration_ms: u64,
    pub executions: Vec<ExecutionResult>,
    pub cases: Vec<CaseResult>,
}

impl RunOutcome {
    pub fn passed(&self) -> bool {
        self.status == AttemptStatus::Passed
    }

    pub fn first_error(&self) -> Option<&str> {
        self.executions
            .iter()
            .find(|execution| !execution.succeeded())
            .and_then(|execution| {
                let output = execution.output();
                if output.trim().is_empty() {
                    None
                } else {
                    Some(output)
                }
            })
    }
}

/// Contrato común de todos los runners.
pub trait LanguageRunner: Send + Sync {
    fn language_code(&self) -> &'static str;
    fn detect(&self, vault: &Path) -> Result<Detection, BasicError>;
    fn run(&self, request: &RunRequest) -> Result<RunOutcome, BasicError>;
}

static CSHARP: OnceLock<CSharpRunner> = OnceLock::new();

/// Runner por defecto para el lenguaje indicado.
pub fn runner_for(language: &str) -> Option<&'static dyn LanguageRunner> {
    match language {
        "csharp" => Some(CSHARP.get_or_init(CSharpRunner::new)),
        _ => None,
    }
}

/// Compara salidas ignorando espacios finales y líneas vacías al final.
pub fn normalize_output(text: &str) -> String {
    let lines: Vec<&str> = text.lines().map(str::trim_end).collect();
    lines.join("\n").trim_end_matches('\n').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_only_trailing_whitespace() {
        assert_eq!(normalize_output("  3  \n"), "  3");
        assert_eq!(normalize_output("uno\ndos\n\n"), "uno\ndos");
        assert_eq!(normalize_output(""), "");
    }

    #[test]
    fn only_csharp_is_registered_for_now() {
        assert_eq!(
            runner_for("csharp").expect("runner de C#").language_code(),
            "csharp"
        );
        assert!(runner_for("rust").is_none());
    }

    #[test]
    fn request_timeout_has_a_safe_minimum() {
        let request = RunRequest {
            exercise_id: 1,
            language: "csharp".to_string(),
            code: String::new(),
            test_spec: TestSpec::default(),
            timeout_secs: 0,
            vault: PathBuf::from("/tmp/vault"),
        };
        assert_eq!(request.timeout().as_secs(), 1);
    }
}
