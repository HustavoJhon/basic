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
    assert_eq!(check(&report, "database.existing")["status"], "skip");

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

    // Un valor personalizado debe sobrevivir a `--force`.
    let custom = contents.replace("focus_minutes = 25", "focus_minutes = 45");
    fs::write(&config_file, custom).unwrap();

    let forced = run({
        let mut command = basic(temp.path());
        command.args(["config", "init", "--force"]);
        command
    });
    assert!(forced.status.success(), "stderr: {}", stderr(&forced));
    let forced_output = stdout(&forced);
    assert!(forced_output.contains("Copia de seguridad"));
    assert!(forced_output.contains("actualizada"));
    assert!(forced_output.contains("Se conservaron los valores existentes."));

    let backups: Vec<_> = fs::read_dir(config_file.parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_name().to_string_lossy().contains(".bak-"))
        .collect();
    assert_eq!(backups.len(), 1, "debe quedar una copia de seguridad");
    assert!(
        fs::read_to_string(backups[0].path())
            .unwrap()
            .contains("focus_minutes = 45")
    );

    let after = run({
        let mut command = basic(temp.path());
        command.args(["config", "show", "--json"]);
        command
    });
    assert!(after.status.success(), "stderr: {}", stderr(&after));
    assert_eq!(json(&after)["config"]["pomodoro"]["focus_minutes"], 45);
}

#[test]
fn strict_passes_once_the_config_exists() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    let init = run({
        let mut command = basic(temp.path());
        command.args(["config", "init"]);
        command
    });
    assert!(init.status.success(), "stderr: {}", stderr(&init));

    // `sh` está disponible tanto en Linux como en macOS; `nvim` puede no
    // estar instalado en el entorno de CI.
    let config_file = temp.path().join(".config/basic/config.toml");
    let contents = fs::read_to_string(&config_file)
        .unwrap()
        .replace("editor = \"nvim\"", "editor = \"sh\"");
    fs::write(&config_file, contents).unwrap();

    let strict = run({
        let mut command = basic(temp.path());
        command.args(["doctor", "--strict"]);
        command
    });
    assert!(
        strict.status.success(),
        "stdout: {}\nstderr: {}",
        stdout(&strict),
        stderr(&strict)
    );
    assert!(stdout(&strict).contains("Estado: correcto."));
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
    assert!(text.contains("index"));
    assert!(text.contains("search"));
}

#[test]
fn index_search_stats_and_review_workflow() {
    let temp = TempDir::new().unwrap();
    let vault = temp.path().join("vault");
    fs::create_dir_all(vault.join("linux/vim")).unwrap();
    fs::write(vault.join("nota.md"), "# Nota\n\nContenido de prueba.\n").unwrap();
    fs::write(
        vault.join("linux/vim/atajos.md"),
        "# Atajos de Vim\n\n`dd` borra una línea y `yy` la copia. #vim\n",
    )
    .unwrap();

    let index = run({
        let mut command = basic(temp.path());
        command.args(["index", "--json"]);
        command
    });
    assert!(index.status.success(), "stderr: {}", stderr(&index));
    let report = json(&index);
    assert_eq!(report["added"], 2, "{report:#}");
    assert_eq!(report["errors"].as_array().unwrap().len(), 0);

    let search = run({
        let mut command = basic(temp.path());
        command.args(["search", "--json", "--scope", "content", "copia"]);
        command
    });
    assert!(search.status.success(), "stderr: {}", stderr(&search));
    let results = json(&search);
    assert_eq!(results.as_array().unwrap().len(), 1, "{results:#}");
    assert_eq!(results[0]["title"], "Atajos de Vim");

    let stats = run({
        let mut command = basic(temp.path());
        command.args(["stats", "--json", "--days", "7"]);
        command
    });
    assert!(stats.status.success(), "stderr: {}", stderr(&stats));
    let stats = json(&stats);
    assert_eq!(stats["stats"]["total"], 2);
    assert_eq!(stats["activity"].as_array().unwrap().len(), 7);

    let review = run({
        let mut command = basic(temp.path());
        command.args(["review", "--json"]);
        command
    });
    assert!(review.status.success(), "stderr: {}", stderr(&review));
    assert_eq!(json(&review).as_array().unwrap().len(), 0);
}

#[test]
fn new_creates_a_note_and_never_overwrites() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    let created = run({
        let mut command = basic(temp.path());
        command.args(["new", "linux/atajos-vim", "--title", "Atajos de Vim"]);
        command
    });
    assert!(created.status.success(), "stderr: {}", stderr(&created));
    let path = temp.path().join("vault/linux/atajos-vim.md");
    let original = fs::read_to_string(&path).unwrap();
    assert!(original.contains("# Atajos de Vim"));
    assert!(original.starts_with("---\ntitle: \"Atajos de Vim\""));

    let conflict = run({
        let mut command = basic(temp.path());
        command.args(["new", "linux/atajos-vim"]);
        command
    });
    assert!(!conflict.status.success());
    assert!(stderr(&conflict).contains("ya existe"));
    assert_eq!(fs::read_to_string(&path).unwrap(), original);
}

#[test]
fn code_lab_seed_and_lessons_workflow() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());
    let init = run({
        let mut command = basic(temp.path());
        command.args(["config", "init"]);
        command
    });
    assert!(init.status.success(), "stderr: {}", stderr(&init));

    let seed = run({
        let mut command = basic(temp.path());
        command.args(["code-lab", "seed", "--json"]);
        command
    });
    assert!(seed.status.success(), "stderr: {}", stderr(&seed));
    let report = json(&seed);
    assert_eq!(report["languages"], 1);
    assert_eq!(report["lessons"], 5);
    assert_eq!(report["questions"], 10);
    assert_eq!(report["exercises"], 6);

    let lessons = run({
        let mut command = basic(temp.path());
        command.args(["code-lab", "lessons", "--json"]);
        command
    });
    assert!(lessons.status.success(), "stderr: {}", stderr(&lessons));
    let list = json(&lessons);
    assert_eq!(list.as_array().unwrap().len(), 5);
    assert_eq!(list[0]["slug"], "primer-programa");
    assert_eq!(list[4]["slug"], "entradas-y-pruebas");
}

#[test]
fn new_rejects_paths_outside_the_vault() {
    let temp = TempDir::new().unwrap();
    make_vault(temp.path());

    for path in ["../fuera", "/absoluta"] {
        let output = run({
            let mut command = basic(temp.path());
            command.args(["new", path]);
            command
        });
        assert!(!output.status.success(), "«{path}» no debería crearse");
    }
}
