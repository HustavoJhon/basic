//! Pruebas de extremo a extremo del binario.
//!
//! Cada prueba usa un HOME temporal y variables XDG propias, así que nunca
//! toca la configuración ni la base de datos real del usuario.

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

const ENV_OVERRIDES: &[&str] = &[
    "BASIC_CONFIG",
    "BASIC_CONFIG_DIR",
    "BASIC_DATA_DIR",
    "BASIC_STATE_DIR",
    "BASIC_DB",
    "BASIC_VAULT",
    "RUST_LOG",
    "EDITOR",
];

fn basic(home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_basic"));
    for key in ENV_OVERRIDES {
        command.env_remove(OsStr::new(key));
    }
    command
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_DATA_HOME", home.join(".local/share"))
        .env("XDG_STATE_HOME", home.join(".local/state"))
        .env("BASIC_VAULT", home.join("vault"));
    command
}

fn run(mut command: Command) -> Output {
    command.output().expect("el binario basic debe ejecutarse")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn json(output: &Output) -> Value {
    let text = stdout(output);
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("stdout no es JSON ({error}):\n{text}"))
}

fn check<'a>(report: &'a Value, id: &str) -> &'a Value {
    report["checks"]
        .as_array()
        .expect("checks debe ser un array")
        .iter()
        .find(|check| check["id"] == id)
        .unwrap_or_else(|| panic!("no existe la comprobación {id}: {report:#}"))
}

fn make_vault(home: &Path) {
    let vault = home.join("vault");
    fs::create_dir_all(&vault).unwrap();
    fs::write(vault.join("nota.md"), "# Nota\n\nContenido de prueba.\n").unwrap();
}

#[test]
fn doctor_json_describes_the_vault_without_creating_the_database() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    let output = run({
        let mut command = basic(temp.path());
        command.args(["doctor", "--json"]);
        command
    });

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report = json(&output);

    assert_eq!(report["summary"]["fail"], 0);
    assert_eq!(report["summary"]["healthy"], true);
    assert_eq!(check(&report, "vault.markdown")["status"], "ok");
    assert!(
        check(&report, "vault.markdown")["detail"]
            .as_str()
            .unwrap()
            .contains("1 archivos")
    );
    assert_eq!(check(&report, "database.location")["status"], "ok");
    assert_eq!(check(&report, "database.migrations")["status"], "ok");
    assert_eq!(check(&report, "database.existing")["status"], "warn");

    // El diagnóstico no debe crear la base real, solo validar el esquema en memoria.
    assert!(!temp.path().join(".local/share/basic/basic.db").exists());

    // El logging sí debe dejar su archivo en el directorio de estado.
    assert!(temp.path().join(".local/state/basic/basic.log").exists());
}

#[test]
fn config_init_show_and_force() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    let init = run({
        let mut command = basic(temp.path());
        command.args(["config", "init"]);
        command
    });
    assert!(init.status.success(), "stderr: {}", stderr(&init));

    let config_file = temp.path().join(".config/basic/config.toml");
    assert!(config_file.exists());
    let contents = fs::read_to_string(&config_file).unwrap();
    assert!(contents.starts_with("# BASIC — configuración"));
    assert!(contents.contains("vault ="));

    let show = run({
        let mut command = basic(temp.path());
        command.args(["config", "show", "--json"]);
        command
    });
    assert!(show.status.success(), "stderr: {}", stderr(&show));
    let value = json(&show);
    assert_eq!(
        value["config"]["vault"].as_str().unwrap(),
        temp.path().join("vault").to_str().unwrap()
    );
    assert_eq!(
        value["config"]["database"].as_str().unwrap(),
        temp.path()
            .join(".local/share/basic/basic.db")
            .to_str()
            .unwrap()
    );
    assert_eq!(value["config"]["pomodoro"]["focus_minutes"], 25);

    let second = run({
        let mut command = basic(temp.path());
        command.args(["config", "init"]);
        command
    });
    assert!(!second.status.success());
    assert!(stderr(&second).contains("ya existe"));

    let forced = run({
        let mut command = basic(temp.path());
        command.args(["config", "init", "--force"]);
        command
    });
    assert!(forced.status.success(), "stderr: {}", stderr(&forced));
}

#[test]
fn config_init_discovers_the_obsidian_root_from_a_subdirectory() {
    let temp = TempDir::new().unwrap();
    let vault = temp.path().join("vault");
    fs::create_dir_all(vault.join(".obsidian")).unwrap();
    let nested = vault.join("development/php");
    fs::create_dir_all(&nested).unwrap();
    fs::write(vault.join("nota.md"), "# Nota\n").unwrap();

    let init = run({
        let mut command = basic(temp.path());
        command.env_remove("BASIC_VAULT");
        command.current_dir(&nested);
        command.args(["config", "init"]);
        command
    });
    assert!(init.status.success(), "stderr: {}", stderr(&init));

    let show = run({
        let mut command = basic(temp.path());
        command.env_remove("BASIC_VAULT");
        command.current_dir(&nested);
        command.args(["config", "show", "--json"]);
        command
    });
    assert!(show.status.success(), "stderr: {}", stderr(&show));
    assert_eq!(
        json(&show)["config"]["vault"].as_str().unwrap(),
        vault.to_str().unwrap()
    );
}

#[test]
fn invalid_config_makes_doctor_fail() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    let config_dir = temp.path().join(".config/basic");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.toml"), "vault = [").unwrap();

    let output = run({
        let mut command = basic(temp.path());
        command.args(["doctor", "--json"]);
        command
    });

    assert!(!output.status.success());
    let report = json(&output);
    assert_eq!(check(&report, "config.source")["status"], "fail");
    assert_eq!(check(&report, "config.values")["status"], "skip");
    assert_eq!(report["summary"]["fail"], 1);
}

#[test]
fn strict_mode_fails_on_warnings() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    let normal = run({
        let mut command = basic(temp.path());
        command.args(["doctor", "--json"]);
        command
    });
    assert!(normal.status.success(), "stderr: {}", stderr(&normal));

    let strict = run({
        let mut command = basic(temp.path());
        command.args(["doctor", "--json", "--strict"]);
        command
    });
    assert!(!strict.status.success());
    let report = json(&strict);
    assert_eq!(report["summary"]["fail"], 0);
    assert!(report["summary"]["warn"].as_u64().unwrap() > 0);
}

#[test]
fn paths_command_respects_environment_overrides() {
    let temp = TempDir::new().unwrap();
    let database = temp.path().join("custom/basic.db");

    let output = run({
        let mut command = basic(temp.path());
        command.env("BASIC_DB", &database);
        command.args(["paths", "--json"]);
        command
    });
    assert!(output.status.success(), "stderr: {}", stderr(&output));

    let value = json(&output);
    assert_eq!(value["database"], database.to_str().unwrap());
    assert_eq!(
        value["config_file"],
        temp.path()
            .join(".config/basic/config.toml")
            .to_str()
            .unwrap()
    );
    assert_eq!(
        value["log_file"],
        temp.path()
            .join(".local/state/basic/basic.log")
            .to_str()
            .unwrap()
    );
}

#[test]
fn no_subcommand_prints_help() {
    let temp = TempDir::new().unwrap();
    let output = run(basic(temp.path()));
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("Usage:"));
    assert!(text.contains("doctor"));
    assert!(text.contains("config"));
}
