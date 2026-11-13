# BASIC TUI

Sistema personal de conocimiento y aprendizaje para la terminal.

> Estado: **F1 completada**. Incluye CLI, configuración XDG, logging, migraciones
> SQLite y `basic doctor`. La TUI, el indexado y la búsqueda llegan en fases
> posteriores. Markdown es la fuente de verdad; SQLite es índice y estado
> complementario.

## Requisitos

- Rust estable (edición 2024, MSRV 1.89).
- SQLite se compila embebido con `rusqlite` (`bundled`) e incluye FTS5; no hace
  falta instalarlo en el sistema.

## Instalación

```bash
cd tui
cargo build --release
cargo install --path .
```

El binario se llama `basic`.

## Comandos

```bash
basic doctor                  # diagnóstico legible
basic doctor --json           # informe estructurado
basic doctor --strict         # los avisos también hacen fallar el comando
basic paths                   # rutas XDG resueltas
basic paths --json
basic config init             # crea ~/.config/basic/config.toml
basic config init --force     # sobrescribe
basic config show             # configuración efectiva en TOML
basic config show --json
basic config path             # solo la ruta del archivo
```

Sin subcomando se muestra la ayuda. `basic --version` y `basic --help` funcionan
como en cualquier CLI de clap.

### Códigos de salida

| Código | Significado |
|---:|---|
| 0 | Correcto (puede haber avisos si no se usa `--strict`) |
| 1 | Fallo de diagnóstico o error de operación |
| 2 | Uso inválido del CLI |

`doctor` no modifica el vault ni abre la base real en escritura: valida el
esquema en una conexión SQLite en memoria y, si la base existe, la abre en modo
lectura. Solo escribe archivos efímeros de prueba en los directorios de datos y
estado para comprobar permisos.

## Rutas XDG

| Dato | Ruta |
|---|---|
| Configuración | `~/.config/basic/config.toml` |
| Base de datos | `~/.local/share/basic/basic.db` |
| Logs | `~/.local/state/basic/basic.log` |

La base de datos vive fuera del vault para no interferir con Obsidian Sync.

Variables de entorno útiles para pruebas o instalaciones no estándar:

| Variable | Efecto |
|---|---|
| `BASIC_CONFIG` | Ruta completa del archivo de configuración |
| `BASIC_CONFIG_DIR` | Directorio de configuración |
| `BASIC_DATA_DIR` | Directorio de datos |
| `BASIC_STATE_DIR` | Directorio de estado y logs |
| `BASIC_DB` | Ruta completa de la base de datos |
| `BASIC_VAULT` | Vault por defecto si no hay configuración |
| `RUST_LOG` | Nivel y filtros de logging (`debug`, `basic=trace`...) |

## Configuración

`basic config init` genera un archivo completo y comentado en la cabecera:

```toml
vault = "/home/usuario/Github/usuario/basic"
database = "/home/usuario/.local/share/basic/basic.db"
editor = "nvim"
theme = "gruvbox-dark"
link_style = "markdown"
frontmatter = true

[scan]
exclude = [".git", ".obsidian", ".trash", "tui", "target", "node_modules"]
follow_symlinks = false

[pomodoro]
focus_minutes = 25
short_break_minutes = 5
long_break_minutes = 15
long_break_every = 4
auto_start_next = false

[ui]
icons = "ascii"
mouse = false
week_starts_on = "monday"
```

Notas sobre la configuración:

- Si no indicas `vault` ni `BASIC_VAULT`, se usa la raíz de Obsidian más cercana
  (subiendo hasta encontrar `.obsidian`) o el directorio actual. Así
  `basic config init` funciona desde cualquier subdirectorio del vault.
- `vault` y `database` deben ser rutas absolutas. `~` se expande al inicio del
  usuario.
- Los campos desconocidos se rechazan para detectar erratas.
- `scan.exclude` contiene **prefijos relativos al vault**, no globs. Excluir
  `tui` excluye todo su subárbol. `.git` y `.obsidian` se ignoran siempre.
- `frontmatter` solo se aplicará a notas nuevas en fases posteriores; nunca se
  reescriben notas existentes.
- `theme`, `link_style`, Pomodoro y UI se usarán en F2 en adelante.

## Logging

Los logs van a `~/.local/state/basic/basic.log`, nunca a stdout, para no romper
`--json`. Al arrancar, si el archivo supera 10 MiB se rota a `basic.log.1`
(se conserva una sola generación). `RUST_LOG` tiene prioridad sobre el filtro por
defecto `basic=info,warn`.

## Base de datos

- SQLite con `journal_mode = WAL`, `foreign_keys = ON` y `busy_timeout` de 5 s.
- Migraciones embebidas en el binario y versionadas con `PRAGMA user_version`.
- FTS5 con tokenizador `unicode61 remove_diacritics 2`, de modo que `prestamo`
  encuentra `préstamo`.
- La migración inicial crea el catálogo, etiquetas, enlaces, estado de
  aprendizaje, Pomodoro, eventos y actividad diaria.
- Si se borra la base, el catálogo podrá reconstruirse escaneando el vault (F3).

## Obsidian

`tui/` está excluido en `.obsidian/app.json` mediante `userIgnoreFilters` y
también en `scan.exclude`. `basic doctor` avisa si la exclusión desaparece, por
ejemplo tras reinstalar o sincronizar el vault.

Para excluirlo manualmente: **Ajustes → Archivos y enlaces → Archivos
excluidos → `tui/`**.

## Calidad

```bash
cd tui
cargo fmt --all --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
```

CI ejecuta exactamente esas comprobaciones en `.github/workflows/ci.yml`.

La auditoría, la arquitectura y la hoja de ruta están en
[`../docs/design/F0-audit.md`](../docs/design/F0-audit.md).

## Alcance de F1

Incluido:

- Paquete Rust `basic` con módulos separados.
- Configuración XDG en TOML con validación.
- Logging a archivo con rotación.
- CLI (`doctor`, `paths`, `config`).
- Migración inicial `0001_init.sql` y migrador.
- Pruebas unitarias e integración.
- CI para fmt, clippy, tests y build release.

No incluido todavía:

- TUI y dashboard.
- Indexado del vault.
- Búsqueda y visor Markdown.
- Creación/edición de notas.
- Pomodoro, estadísticas y repaso.
