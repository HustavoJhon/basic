//! Adaptador para C# usando el SDK de .NET instalado por el usuario.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::code_lab::domain::{AttemptStatus, ExecutionPhase, ExecutionResult, TestSpec};
use crate::error::BasicError;

use super::process::{self, CommandSpec};
use super::workspace::Workspace;
use super::{CaseResult, Detection, LanguageRunner, RunOutcome, RunRequest, normalize_output};

const LANGUAGE: &str = "csharp";
const PROJECT_NAME: &str = "BasicRunner";
const DETECT_TIMEOUT_SECS: u64 = 5;

/// Runner de C#. No instala nada y no eleva privilegios.
#[derive(Debug, Clone)]
pub struct CSharpRunner {
    sdk: PathBuf,
}

impl Default for CSharpRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl CSharpRunner {
    pub fn new() -> Self {
        Self {
            sdk: PathBuf::from("dotnet"),
        }
    }

    /// Permite inyectar un SDK falso en pruebas sin cambiar el comportamiento
    /// del binario.
    pub fn with_sdk(sdk: impl Into<PathBuf>) -> Self {
        Self { sdk: sdk.into() }
    }

    fn detect_in(&self, workspace: &Workspace, timeout: Duration) -> Result<Detection, BasicError> {
        let execution = self.execute(
            workspace,
            ExecutionPhase::Detect,
            "detect",
            &[process::arg("--version")],
            None,
            timeout,
        )?;

        let version = if execution.succeeded() {
            let candidate = execution.stdout.trim().lines().next().unwrap_or("").trim();
            if is_plausible_version(candidate) {
                Some(candidate.to_string())
            } else {
                None
            }
        } else {
            None
        };

        Ok(Detection { version, execution })
    }

    fn execute(
        &self,
        workspace: &Workspace,
        phase: ExecutionPhase,
        label: &str,
        args: &[OsString],
        stdin: Option<&str>,
        timeout: Duration,
    ) -> Result<ExecutionResult, BasicError> {
        let environment = dotnet_environment(workspace);
        process::run(&CommandSpec {
            program: &self.sdk,
            args: args.to_vec(),
            cwd: &workspace.project_dir,
            stdin,
            timeout,
            phase,
            label,
            logs: &workspace.logs,
            home: &workspace.home,
            tmp: &workspace.tmp,
            env: &environment,
        })
    }
}

impl LanguageRunner for CSharpRunner {
    fn language_code(&self) -> &'static str {
        LANGUAGE
    }

    fn detect(&self, vault: &Path) -> Result<Detection, BasicError> {
        let workspace = Workspace::create(vault)?;
        self.detect_in(&workspace, Duration::from_secs(DETECT_TIMEOUT_SECS))
    }

    fn run(&self, request: &RunRequest) -> Result<RunOutcome, BasicError> {
        let started = std::time::Instant::now();
        let workspace = Workspace::create(&request.vault)?;
        let timeout = request.timeout();

        let detection = self.detect_in(&workspace, timeout)?;
        let mut executions = vec![detection.execution.clone()];
        let sdk_version = detection.version.clone();

        let Some(version) = sdk_version.as_deref() else {
            return Ok(failed_outcome(
                sdk_version,
                None,
                &request.test_spec,
                executions,
                started.elapsed().as_millis(),
            ));
        };
        let Some(framework) = target_framework(version) else {
            return Err(BasicError::InvalidConfig {
                path: PathBuf::from("dotnet"),
                message: format!("versión de .NET no soportada: {version}"),
            });
        };

        let csproj = project_file(&framework);
        workspace.write("project/BasicRunner.csproj", &csproj)?;
        workspace.write("project/Program.cs", &request.code)?;

        let project_path = workspace.project_dir.join(format!("{PROJECT_NAME}.csproj"));
        let restore = self.execute(
            &workspace,
            ExecutionPhase::Restore,
            "restore",
            &[
                process::arg("restore"),
                process::arg(&project_path),
                process::arg("--nologo"),
                process::arg("--verbosity"),
                process::arg("quiet"),
                process::arg("--source"),
                process::arg(&workspace.nuget_source),
            ],
            None,
            timeout,
        )?;
        executions.push(restore.clone());

        let build = if restore.succeeded() {
            self.execute(
                &workspace,
                ExecutionPhase::Build,
                "build",
                &[
                    process::arg("build"),
                    process::arg(&project_path),
                    process::arg("-c"),
                    process::arg("Release"),
                    process::arg("--no-restore"),
                    process::arg("--nologo"),
                    process::arg("--verbosity"),
                    process::arg("quiet"),
                ],
                None,
                timeout,
            )?
        } else {
            skipped(ExecutionPhase::Build, "restore falló")
        };
        executions.push(build.clone());

        if !build.succeeded() {
            return Ok(failed_outcome(
                sdk_version,
                Some(false),
                &request.test_spec,
                executions,
                started.elapsed().as_millis(),
            ));
        }

        let dll = workspace
            .project_dir
            .join("bin")
            .join("Release")
            .join(&framework)
            .join(format!("{PROJECT_NAME}.dll"));
        let mut cases = Vec::new();
        let mut passed = 0;

        for (index, case) in request.test_spec.cases.iter().enumerate() {
            let label = format!("case-{index:02}");
            let execution = self.execute(
                &workspace,
                ExecutionPhase::Run,
                &label,
                &[process::arg(&dll)],
                Some(&case.input),
                timeout,
            )?;
            executions.push(execution.clone());
            let actual = normalize_output(&execution.stdout);
            let expected = normalize_output(&case.expected_output);
            let case_passed = execution.succeeded() && actual == expected;
            if case_passed {
                passed += 1;
            }
            cases.push(CaseResult {
                name: case.name.clone(),
                passed: case_passed,
                visible: case.visible,
                expected,
                actual,
                duration_ms: execution.duration_ms,
            });
        }

        if cases.is_empty() {
            executions.push(skipped(
                ExecutionPhase::Test,
                "la especificación no define casos de prueba",
            ));
        }

        let tests_total = request.test_spec.cases.len() as i32;
        let status = if !cases.is_empty() && passed == tests_total {
            AttemptStatus::Passed
        } else {
            AttemptStatus::Failed
        };

        Ok(RunOutcome {
            status,
            sdk_version,
            compile_ok: Some(true),
            tests_passed: passed,
            tests_total,
            duration_ms: elapsed_ms(started.elapsed().as_millis()),
            executions,
            cases,
        })
    }
}

fn dotnet_environment(workspace: &Workspace) -> Vec<(OsString, OsString)> {
    let nuget = workspace.home.join(".nuget");
    vec![
        (
            process::arg("DOTNET_CLI_TELEMETRY_OPTOUT"),
            process::arg("1"),
        ),
        (process::arg("DOTNET_NOLOGO"), process::arg("1")),
        (
            process::arg("DOTNET_SKIP_FIRST_TIME_EXPERIENCE"),
            process::arg("1"),
        ),
        (
            process::arg("DOTNET_CLI_HOME"),
            process::arg(&workspace.home),
        ),
        (
            process::arg("NUGET_PACKAGES"),
            process::arg(nuget.join("packages")),
        ),
        (
            process::arg("NUGET_HTTP_CACHE_PATH"),
            process::arg(nuget.join("http-cache")),
        ),
        (
            process::arg("NUGET_PLUGINS_CACHE_PATH"),
            process::arg(nuget.join("plugins-cache")),
        ),
    ]
}

fn project_file(framework: &str) -> String {
    format!(
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    \
         <OutputType>Exe</OutputType>\n    \
         <TargetFramework>{framework}</TargetFramework>\n    \
         <ImplicitUsings>enable</ImplicitUsings>\n    \
         <Nullable>disable</Nullable>\n    \
         <AssemblyName>{PROJECT_NAME}</AssemblyName>\n    \
         <RootNamespace>{PROJECT_NAME}</RootNamespace>\n    \
         <InvariantGlobalization>true</InvariantGlobalization>\n    \
         <EnableDefaultCompileItems>false</EnableDefaultCompileItems>\n  </PropertyGroup>\n  \
         <ItemGroup>\n    <Compile Include=\"Program.cs\" />\n  </ItemGroup>\n</Project>\n"
    )
}

fn target_framework(version: &str) -> Option<String> {
    let major = version.trim().split('.').next()?.parse::<u32>().ok()?;
    if major < 6 {
        return None;
    }
    Some(format!("net{major}.0"))
}

fn is_plausible_version(version: &str) -> bool {
    !version.is_empty()
        && version.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '+')
        })
}

fn skipped(phase: ExecutionPhase, reason: &str) -> ExecutionResult {
    ExecutionResult {
        phase,
        command: String::new(),
        exit_code: None,
        stdout: String::new(),
        stderr: format!("Fase omitida: {reason}"),
        duration_ms: 0,
    }
}

fn failed_outcome(
    sdk_version: Option<String>,
    compile_ok: Option<bool>,
    spec: &TestSpec,
    executions: Vec<ExecutionResult>,
    elapsed: u128,
) -> RunOutcome {
    RunOutcome {
        status: AttemptStatus::Failed,
        sdk_version,
        compile_ok,
        tests_passed: 0,
        tests_total: spec.cases.len() as i32,
        duration_ms: elapsed_ms(elapsed),
        executions,
        cases: Vec::new(),
    }
}

fn elapsed_ms(elapsed: u128) -> u64 {
    u64::try_from(elapsed).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use crate::code_lab::domain::TestCase;

    use super::*;

    #[test]
    fn maps_sdk_major_to_target_framework() {
        assert_eq!(target_framework("10.0.400").as_deref(), Some("net10.0"));
        assert_eq!(target_framework("8.0.100").as_deref(), Some("net8.0"));
        assert_eq!(target_framework("5.0.400"), None);
        assert_eq!(target_framework("previsto"), None);
    }

    #[test]
    fn project_file_uses_detected_framework_only() {
        let project = project_file("net10.0");
        assert!(project.contains("<TargetFramework>net10.0</TargetFramework>"));
        assert!(project.contains("<Compile Include=\"Program.cs\" />"));
        assert!(!project.contains("PackageReference"));
    }

    #[test]
    fn validates_sdk_version_before_writing_files() {
        assert!(is_plausible_version("10.0.400"));
        assert!(is_plausible_version("10.0.100-preview.7"));
        assert!(!is_plausible_version("10.0.400\nmalicioso"));
        assert!(!is_plausible_version(""));
    }

    #[test]
    fn distinguishes_compile_result_and_missing_cases() {
        let outcome = failed_outcome(None, Some(false), &TestSpec::default(), Vec::new(), 42);
        assert_eq!(outcome.status, AttemptStatus::Failed);
        assert_eq!(outcome.compile_ok, Some(false));
        assert_eq!(outcome.duration_ms, 42);
        assert!(!outcome.passed());
    }

    #[cfg(unix)]
    #[test]
    fn fake_sdk_runs_the_full_flow_without_real_dotnet() {
        use std::os::unix::fs::PermissionsExt;

        let tools = TempDir::new().unwrap();
        let vault = TempDir::new().unwrap();
        let sdk = tools.path().join("dotnet");
        std::fs::write(
            &sdk,
            "#!/bin/sh\ncase \"$1\" in\n  --version) echo 10.0.400 ;;\n  restore|build) exit 0 ;;\n  *.dll) cat >/dev/null; echo 3; exit 0 ;;\n  *) exit 2 ;;\nesac\n",
        )
        .unwrap();
        std::fs::set_permissions(&sdk, std::fs::Permissions::from_mode(0o755)).unwrap();

        let request = RunRequest {
            exercise_id: 1,
            language: "csharp".to_string(),
            code: "Console.In.ReadToEnd();".to_string(),
            test_spec: TestSpec {
                cases: vec![TestCase {
                    name: "suma".to_string(),
                    input: "1 2".to_string(),
                    expected_output: "3\n".to_string(),
                    visible: true,
                }],
                ..TestSpec::default()
            },
            timeout_secs: 5,
            vault: vault.path().to_path_buf(),
        };
        let outcome = CSharpRunner::with_sdk(&sdk).run(&request).unwrap();
        assert_eq!(outcome.sdk_version.as_deref(), Some("10.0.400"));
        assert_eq!(outcome.compile_ok, Some(true));
        assert!(outcome.passed(), "{:?}", outcome.first_error());
        assert_eq!(outcome.tests_passed, 1);
        assert_eq!(outcome.cases[0].actual, "3");
        assert!(outcome.cases[0].visible);
    }

    #[test]
    #[ignore = "requiere el SDK de .NET instalado"]
    fn real_sdk_runs_a_csharp_program() {
        let vault = TempDir::new().unwrap();
        let code = "\
var partes = Console.In.ReadToEnd().Trim().Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);
int a = int.Parse(partes[0]);
int b = int.Parse(partes[1]);
Console.WriteLine(a + b);
";
        let request = RunRequest {
            exercise_id: 1,
            language: "csharp".to_string(),
            code: code.to_string(),
            test_spec: TestSpec {
                cases: vec![
                    TestCase {
                        name: "visible".to_string(),
                        input: "1 2\n".to_string(),
                        expected_output: "3\n".to_string(),
                        visible: true,
                    },
                    TestCase {
                        name: "oculto".to_string(),
                        input: "10 25\n".to_string(),
                        expected_output: "35\n".to_string(),
                        visible: false,
                    },
                ],
                ..TestSpec::default()
            },
            timeout_secs: 30,
            vault: vault.path().to_path_buf(),
        };
        let outcome = CSharpRunner::new().run(&request).unwrap();
        assert!(
            outcome.sdk_version.is_some(),
            "no se detectó el SDK de .NET"
        );
        assert!(
            outcome.passed(),
            "sdk={:?} compile={:?} error={:?}",
            outcome.sdk_version,
            outcome.compile_ok,
            outcome.first_error()
        );
        assert_eq!(outcome.tests_passed, 2);
    }
}
