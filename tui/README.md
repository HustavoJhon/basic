# BASIC TUI

Sistema personal de conocimiento y aprendizaje para la terminal.

> Estado: **F2–F16 implementadas**. Incluye TUI Gruvbox, indexado incremental,
> explorador y búsqueda, visor Markdown, creación de notas, Learning Mode,
> Pomodoro, estadísticas con heatmap y repaso tipo SM-2. Code Lab añade el
> esquema v3, el dominio del lenguaje, el editor TUI reutilizable, el runner
> síncrono de C# y el catálogo de lecciones con cuestionarios. Markdown es la
> fuente de verdad; SQLite es índice y estado complementario. Las notas
> existentes nunca se reescriben automáticamente.

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
basic tui                     # abre la interfaz de terminal
basic index                   # escanea el vault y actualiza el índice
basic index --json

basic search vim              # búsqueda difusa y de contenido
basic search --scope content prestamo
basic search --json --limit 10 ownership

basic stats                   # resumen y actividad
basic stats --json --days 30

basic new linux/atajos-vim    # crea una nota nueva (nunca sobrescribe)
basic new notas/idea --title "Idea" --no-index

basic review                  # repasos pendientes
basic review --json

basic code-lab seed           # puebla lecciones y ejercicios
basic code-lab seed --json
basic code-lab lessons        # lista las lecciones de C#
basic code-lab lessons --json

basic doctor                  # diagnóstico legible
basic doctor --json           # informe estructurado
basic doctor --strict         # los avisos también hacen fallar el comando
basic paths                   # rutas XDG resueltas
basic paths --json
basic config init             # crea ~/.config/basic/config.toml
basic config init --force     # reescribe y deja una copia .bak-*
basic config show             # configuración efectiva en TOML
basic config show --json
basic config path             # solo la ruta del archivo
```

En una terminal interactiva, ejecutar `basic` sin subcomando abre la TUI. En
scripts o CI imprime la ayuda para no bloquear. `basic --version` y
`basic --help` funcionan como en cualquier CLI de clap.

`search` acepta `--scope all|title|content|path|category|code`. La búsqueda de
contenido usa FTS5 (insensible a acentos); la de títulos, rutas y categorías usa
coincidencia difusa y ordena por relevancia.

## TUI

| Tecla | Acción |
|---|---|
| `1`–`6` | inicio, explorar, aprender, repaso, pomodoro, estadísticas |
| `Tab` | siguiente pantalla |
| `j`/`k` | mover selección o desplazar el lector |
| `Enter` | abrir el documento seleccionado |
| `/` | buscar |
| `n` | crear una nota nueva |
| `i` | reindexar el vault |
| `s` | cambiar estado de aprendizaje (en el lector) |
| `f` | marcar o quitar favorito |
| `e` | abrir la nota en `$EDITOR` |
| `p` | ir a Pomodoro e iniciarlo |
| `Espacio` | iniciar o pausar Pomodoro |
| `x` / `>` | reiniciar / saltar fase |
| `1`–`4` | calificar un repaso (otra vez, difícil, bien, fácil) |
| `1`–`5` | elegir modo en Learning Mode |
| `?` | ayuda contextual |
| `q` / `Esc` | salir / volver |

La TUI usa la paleta Gruvbox Dark, se adapta a partir de 70×20 y recurre a una
vista de una columna en terminales estrechas. El lector renderiza encabezados,
listas, tablas, código, citas, enlaces e imágenes como marcadores; las imágenes
no se dibujan en la terminal.

### Crear y editar

`basic new` y `n` en la TUI crean un archivo con frontmatter opcional y se niegan
a escribir si la ruta ya existe. La ruta debe ser relativa al vault y no puede
salir de él (`..`, rutas absolutas). Para editar se abre el editor externo
configurado (`editor`); al volver, la nota se recarga y se reindexa la siguiente
vez que se solicite.

### Códigos de salida

| Código | Significado |
|---:|---|
| 0 | Correcto (puede haber avisos si no se usa `--strict`) |
| 1 | Fallo de diagnóstico o error de operación |
| 2 | Uso inválido del CLI |

`doctor` no modifica el vault ni abre la base real en escritura: valida el
esquema en una conexión SQLite en memoria y, si la base existe, la abre en modo
lectura. Para comprobar permisos puede crear los directorios de datos y estado
si no existen y escribe en ellos archivos efímeros de prueba.

Con `--strict`, el informe indica explícitamente `Estado: fallo por avisos
(--strict)` cuando el código de salida es 1 por ese motivo.

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
- `basic config init --force` guarda una copia `config.toml.bak-<fecha>` antes de
  escribir. Si la configuración anterior es válida conserva sus valores, así que
  ejecutarlo desde otro directorio no cambia el vault configurado. Si no se
  puede leer o validar, se generan los valores por defecto.
- `scan.exclude` contiene **prefijos relativos al vault**, no globs. Excluir
  `tui` excluye todo su subárbol. `.git` y `.obsidian` se ignoran siempre.
- `frontmatter` solo se aplica a notas nuevas; las existentes nunca se
  reescriben automáticamente. `link_style` queda reservado para generación de
  enlaces y `theme` para temas alternativos (hoy Gruvbox Dark).
- Los tiempos de Pomodoro y `auto_start_next` se aplican en la TUI. Si la
  aplicación se cierra a mitad de una sesión, `pomodoro_runtime` permite saber
  que quedó una sesión abierta.

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
- El índice es incremental por hash SHA-256 y tamaño/mtime; los archivos
  borrados se marcan como ausentes en lugar de eliminarse del historial.
- Si se borra la base, el catálogo se reconstruye escaneando el vault. `basic
  index` y la primera ejecución de la TUI lo hacen automáticamente.
- El indexador ignora `.git`, `.obsidian`, `.trash`, `tui`, `target` y
  `node_modules` (prefijos configurables en `scan.exclude`).
- `doctor` marca como omitida la comprobación de la base cuando aún no existe;
  así `--strict` no falla por algo que se crea al primer uso.
- El repaso usa un SM-2 simplificado y determinista: `otra vez` reinicia el
  intervalo, `difícil` crece poco, `bien` y `fácil` alargan el intervalo.
- La actividad diaria alimenta el heatmap y la racha. Solo las sesiones de
  Pomodoro de concentración completadas suman tiempo de estudio.

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

## Fases implementadas

| Fase | Alcance |
|---|---|
| F0 | Auditoría y arquitectura |
| F1 | Paquete Rust, XDG, logging, CLI, `doctor`, migraciones, CI |
| F2 | TUI Ratatui con tema Gruvbox y dashboard |
| F3 | Escáner e indexador incremental con FTS5 |
| F4 | Explorador y búsqueda difusa/de contenido |
| F5 | Visor Markdown con tablas, listas, código y citas |
| F6 | Creación segura de notas y edición externa |
| F7 | Learning Mode por modos |
| F8 | Pomodoro con fases, ciclos y persistencia |
| F9 | Estadísticas, actividad diaria y heatmap |
| F10 | Repaso con SM-2 simplificado |

Pendiente para una fase posterior: resaltado de sintaxis por lenguaje,
renderizado real de imágenes, edición embebida y temas alternativos.

## Arquitectura

```text
src/
├── main.rs          # composition root y despacho CLI
├── cli.rs           # comandos clap
├── domain.rs        # modelos, SM-2, actividad y formato
├── app.rs           # estado, acciones y Pomodoro (sin Ratatui)
├── ui.rs            # pantallas Ratatui, eventos y tema Gruvbox
├── ui/markdown.rs   # Markdown a líneas de Ratatui
└── infra/
    ├── markdown.rs  # metadatos, etiquetas, enlaces y excerpt
    ├── index.rs     # escaneo, upsert, FTS, tags y enlaces
    ├── catalog.rs   # consultas, búsqueda, estadísticas, repaso y Pomodoro
    ├── notes.rs     # creación de notas con frontmatter
    ├── sqlite.rs    # apertura, migraciones y conexión
    └── fs.rs        # escritura atómica
```

Markdown es la fuente de verdad. `app` e `infra` no dependen de Ratatui; `ui`
solo presenta y traduce eventos. Esto mantiene la lógica de repaso, Pomodoro e
indexado comprobable sin terminal.
