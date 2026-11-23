//! Ejecución síncrona de procesos con salida a archivos y timeout.

use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::code_lab::domain::{ExecutionPhase, ExecutionResult};
use crate::error::BasicError;

use super::workspace::write_file;

const MAX_OUTPUT_CHARS: usize = 32 * 1024;
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// Parámetros de una sola fase de ejecución.
#[derive(Debug)]
pub(crate) struct CommandSpec<'a> {
    pub program: &'a Path,
    pub args: Vec<OsString>,
    pub cwd: &'a Path,
    pub stdin: Option<&'a str>,
    pub timeout: Duration,
    pub phase: ExecutionPhase,
    pub label: &'a str,
    pub logs: &'a Path,
    pub home: &'a Path,
    pub tmp: &'a Path,
    pub env: &'a [(OsString, OsString)],
}

/// Ejecuta un proceso sin shell, sin tuberías que puedan bloquearse y con un
/// límite de tiempo. La salida se guarda en archivos del workspace.
pub(crate) fn run(spec: &CommandSpec<'_>) -> Result<ExecutionResult, BasicError> {
    let stdin_path = spec.logs.join(format!("{}.stdin", spec.label));
    let stdout_path = spec.logs.join(format!("{}.stdout", spec.label));
    let stderr_path = spec.logs.join(format!("{}.stderr", spec.label));

    match spec.stdin {
        Some(input) => write_file(&stdin_path, input)?,
        None => write_file(&stdin_path, "")?,
    }

    let stdin = File::open(&stdin_path).map_err(|source| BasicError::ReadFile {
        path: stdin_path.clone(),
        source,
    })?;
    let stdout = File::create(&stdout_path).map_err(|source| BasicError::WriteFile {
        path: stdout_path.clone(),
        source,
    })?;
    let stderr = File::create(&stderr_path).map_err(|source| BasicError::WriteFile {
        path: stderr_path.clone(),
        source,
    })?;

    let mut command = Command::new(spec.program);
    command
        .current_dir(spec.cwd)
        .args(&spec.args)
        .stdin(Stdio::from(stdin))
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .env("HOME", spec.home)
        .env("TMPDIR", spec.tmp)
        .env("XDG_CACHE_HOME", spec.home.join(".cache"));
    for (key, value) in spec.env {
        command.env(key, value);
    }

    let command_text = display(spec.program, &spec.args);
    let mut child = command.spawn().map_err(|source| BasicError::ProcessSpawn {
        command: command_text.clone(),
        source,
    })?;

    let started = Instant::now();
    let status = loop {
        match child.try_wait()? {
            Some(status) => break Some(status),
            None if started.elapsed() >= spec.timeout => break None,
            None => thread::sleep(POLL_INTERVAL),
        }
    };
    let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

    if status.is_none() {
        let _ = child.kill();
        let _ = child.wait();
        let stderr_text = read_bounded(&stderr_path)?;
        let stdout_text = read_bounded(&stdout_path)?;
        let timeout_note = format!(
            "Tiempo de espera agotado tras {}s.\n{stderr_text}",
            spec.timeout.as_secs()
        );
        return Ok(ExecutionResult {
            phase: spec.phase,
            command: command_text,
            exit_code: None,
            stdout: stdout_text,
            stderr: timeout_note,
            duration_ms,
        });
    }

    let status = status.expect("estado ya comprobado");
    Ok(ExecutionResult {
        phase: spec.phase,
        command: command_text,
        exit_code: status.code(),
        stdout: read_bounded(&stdout_path)?,
        stderr: read_bounded(&stderr_path)?,
        duration_ms,
    })
}

fn display(program: &Path, args: &[OsString]) -> String {
    let mut parts = vec![program.to_string_lossy().into_owned()];
    parts.extend(args.iter().map(|arg| arg.to_string_lossy().into_owned()));
    parts.join(" ")
}

pub(crate) fn read_bounded(path: &Path) -> Result<String, BasicError> {
    let bytes = fs::read(path).map_err(|source| BasicError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let text = String::from_utf8_lossy(&bytes);
    let mut chars = text.chars();
    let mut output: String = chars.by_ref().take(MAX_OUTPUT_CHARS).collect();
    if chars.next().is_some() {
        output.push_str("\n[salida truncada]");
    }
    Ok(output)
}

pub(crate) fn arg(value: impl AsRef<OsStr>) -> OsString {
    value.as_ref().to_os_string()
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn truncates_large_output_without_breaking_utf8() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("out.txt");
        let text = "á".repeat(MAX_OUTPUT_CHARS + 10);
        fs::write(&path, text).unwrap();
        let output = read_bounded(&path).unwrap();
        assert!(output.ends_with("[salida truncada]"));
        assert!(output.chars().count() <= MAX_OUTPUT_CHARS + 20);
    }

    #[test]
    fn builds_a_readable_command_without_a_shell() {
        let args = vec![arg("--version"), arg("/tmp/proyecto con espacio")];
        assert_eq!(
            display(Path::new("dotnet"), &args),
            "dotnet --version /tmp/proyecto con espacio"
        );
    }

    #[test]
    fn writes_stdin_before_spawning() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("in.txt");
        write_file(&path, "1 2\n").unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), "1 2\n");
    }
}
