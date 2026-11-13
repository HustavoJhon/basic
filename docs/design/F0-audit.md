# F0 — Auditoría y diseño de BASIC TUI

- **Fecha:** 2026-11-13
- **Rama:** `dev`
- **Estado:** auditoría completada; arquitectura aprobada; F1 habilitada
- **Alcance:** solo lectura sobre el vault. No se modificaron notas, proyectos ni historial de Git.

---

## 1. Resumen ejecutivo

`basic` es un vault de Obsidian con **71 documentos Markdown (175 KB, 4.725 líneas)** y varios años de apuntes, cursos, ejercicios y un proyecto académico PHP. El corpus es pequeño y está en UTF-8 limpio, sin BOM, sin CRLF y sin enlaces simbólicos, por lo que es viable indexarlo completo al inicio sin penalización perceptible.

Los problemas principales no son de tamaño sino de **identidad y metadatos**:

- 42 archivos comparten el nombre `readme.md`.
- No existe frontmatter YAML ni wikilinks.
- Hay 5 documentos vacíos y 11 sin H1.
- Hay 45 rutas con espacios y 4 con caracteres no ASCII.
- El historial de Git pesa 342 MB por contenido eliminado, pero la aplicación no dependerá de Git.

La decisión arquitectónica central es mantener **Markdown como fuente de verdad** y usar **SQLite como índice y almacenamiento complementario** para aprendizaje, Pomodoro y actividad. La base de datos vive fuera del vault.

### Decisiones aprobadas

1. El proyecto Rust vive en `basic/tui/`.
2. `tui/` y `tui/target/` se excluyen del indexador y se excluirán en Obsidian.
3. SQLite vive en `~/.local/share/basic/basic.db`.
4. El estado de aprendizaje y las etiquetas se gestionan inicialmente en SQLite.
5. Los enlaces se generan en Markdown relativo.
6. El proyecto PHP académico se indexa como `project`, sin modificar su contenido.
7. La auditoría se guarda en `docs/design/F0-audit.md`.

### Mejoras incorporadas al diseño

- **`basic learn`** debe ofrecer cinco modos: continuar, explorar, pendientes, repasar y tema aleatorio.
- **`basic search`** debe buscar por título, contenido, ruta, categoría y código.
- **El heatmap** mide actividad de aprendizaje real (tiempo de estudio, Pomodoros, repasos y cambios), no simples aperturas.
- **SQLite nunca sustituye al Markdown**: si se borra la base de datos, el catálogo se reconstruye escaneando el vault.

---

## 2. Inventario real

| Métrica | Valor |
|---|---:|
| Archivos totales | 287 |
| Directorios | 141 |
| Profundidad máxima | 8 |
| Ruta más larga | 120 caracteres |
| Markdown | 71 archivos · 174.864 bytes |
| Líneas Markdown | 4.725 |
| JavaScript | 69 archivos · 292 KB |
| PHP | 35 archivos · 143 KB |
| PDFs | 2 · 3,5 MB |
| Imágenes | 49 · 8 MB |
| Commits | 423 desde 2022-06 |
| `.git` | 342 MB |

### Tipos de documento (clasificación exclusiva)

| Tipo | Cantidad |
|---|---:|
| Índices / README | 38 |
| Apuntes técnicos | 9 |
| Material de curso | 10 |
| Documentación de proyecto | 8 |
| Vacíos | 5 |
| Sin clasificar | 1 |
| **Total** | **71** |

### Categorías detectadas

```text
development (39)
├── javascript (23)
├── php (11)
├── rocketbot (4)
└── api (1)

resources (13)
├── platzi (10)
├── codigofacilito (1)
├── note (1)
└── youtube (1)

linux (9)
├── command (5)
├── security (3)
└── vim (1)

tools (5)
├── docker (2)
├── github (2)
└── git (1)

learning (2)
├── blockchain (1)
└── latex (1)

arquitecture (1)
(raíz) (2)
```

La categoría se deriva del primer segmento de la ruta relativa. Las categorías no se almacenan como verdad absoluta; si hace falta reagrupar, se usará una tabla de overrides.

---

## 3. Calidad del corpus y riesgos para el indexador

| Hallazgo | Impacto | Mitigación |
|---|---|---|
| 42 `readme.md` | Colisión de identificadores | ID = ruta relativa normalizada; breadcrumb visible |
| 5 documentos vacíos | Sin título ni contenido | `is_stub = true`; no fallar |
| 11 documentos sin H1 | Sin título natural | Fallback: primer heading → nombre de archivo → carpeta |
| 1 título H1 duplicado | Resultados ambiguos | Mostrar siempre ruta/categoría |
| 45 rutas con espacios | Enlaces y shell | Codificar `%20` o usar `<ruta con espacios>` |
| 4 rutas no ASCII | Ancho visual | UTF-8 + `unicode-width` |
| 105 rutas con mayúsculas | Filesystems case-insensitive | Preservar case; comprobar colisiones al crear |
| 0 frontmatter / 0 wikilinks | Sin metadatos ni grafo | Derivar metadatos; frontmatter solo en notas nuevas |
| 239 de 435 bloques sin lenguaje | Resaltado pobre | Fallback a texto plano; no adivinar agresivamente |
| 166 etiquetas HTML inline | Renderer ingenuo las muestra | Strip/escape en el AST |
| 34 imágenes | TUI no renderiza imágenes | Placeholder + ruta |
| 8 enlaces relativos rotos | Resolución fallida | Guardar `resolved = false` |
| 12 documentos con tablas | Sin soporte nativo en Ratatui | Renderer propio |
| Línea de 1.041 caracteres | Desbordes | Wrapping por ancho visual |
| 18 archivos sin newline final | Ediciones concatenadas | Normalizar al escribir |
| Proyecto PHP académico | No debe modificarse | Escáner de solo lectura |

**Fortalezas del corpus:** sin BOM, sin CRLF, sin archivos no-UTF8, sin symlinks, sin caracteres problemáticos en rutas (`# % ? [ ] & | < >`).

---

## 4. Arquitectura Rust

### 4.1 Paquete único con módulos estrictos

```text
tui/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── migrations/
│   └── 0001_init.sql
├── templates/
│   ├── note.md
│   ├── course.md
│   ├── exercise.md
│   └── project.md
├── src/
│   ├── main.rs
│   ├── cli/
│   ├── config.rs
│   ├── logging.rs
│   ├── domain/
│   ├── app/
│   ├── infra/
│   │   ├── sqlite.rs
│   │   ├── vault.rs
│   │   ├── markdown.rs
│   │   └── clock.rs
│   └── ui/
└── tests/
```

### 4.2 Dirección de dependencias

```text
cli ─────┐
         ├──> app ──> domain
ui ──────┘             ▲
                       │ traits (ports)
infra ─────────────────┘
```

- `domain` no conoce Ratatui, SQLite ni el sistema de archivos.
- `app` implementa casos de uso y depende de traits.
- `infra` implementa persistencia, escaneo, Markdown y reloj.
- `cli` y `ui` son adaptadores.
- `main.rs` es el único composition root.

### 4.3 Dependencias

| Crate | Uso | Justificación |
|---|---|---|
| `ratatui` + `crossterm` | TUI y eventos | Estándar maduro, sin runtime async |
| `clap` | CLI | Subcomandos, help y validación |
| `serde` + `toml` | Configuración | Config humana y versionable |
| `serde_json` | Export/import y `--json` | Interoperabilidad |
| `rusqlite` (`bundled`) | Persistencia | Síncrono, FTS5, sin Tokio |
| `pulldown-cmark` | Markdown | CommonMark, eventos claros |
| `syntect` | Resaltado | Temas `.tmTheme`; carga perezosa |
| `fuzzy-matcher` | Búsqueda difusa | Suficiente para títulos y rutas |
| `chrono` | Fechas | UTC + hora local |
| `unicode-width` | Anchos | Acentos, emojis y CJK |
| `walkdir` | Escaneo | Recorrido robusto |
| `directories` | XDG | Config/data/state correctos |
| `thiserror` | Errores de dominio | Tipados |
| `anyhow` | Borde del binario | Contexto en `main` |
| `tracing` + `tracing-subscriber` | Logs | Archivo, sin romper stdout |

**Rechazos:** `sqlx`, `tokio`, `git2`, `reqwest`, ORM, watcher de archivos, renderizado de imágenes.

### 4.4 Flujo de indexado

1. Cargar catálogo desde SQLite.
2. Lanzar hilo de escaneo del vault.
3. Parsear Markdown y enviar lotes por canal.
4. Escribir en SQLite en una transacción.
5. Actualizar la UI con progreso.
6. La búsqueda usa FTS5 para contenido y fuzzy para título/ruta.

**Identidad:** ruta relativa con `/`. Si un archivo desaparece se marca `missing_at`; no se borra el estado de aprendizaje. La reconciliación por hash para renombrados queda para F10.

### 4.5 Ubicaciones

| Dato | Ruta |
|---|---|
| Código | `basic/tui/` |
| Config | `~/.config/basic/config.toml` |
| SQLite | `~/.local/share/basic/basic.db` |
| Logs | `~/.local/state/basic/basic.log` |

La base de datos nunca vive dentro del vault para evitar conflictos con Obsidian Sync.

### 4.6 Concurrencia

- Hilo principal: UI y escrituras SQLite rápidas.
- Hilo indexador: recorrido y parseo; no toca SQLite.
- Canal `std::sync::mpsc`.
- `AppState::update(Action) -> Vec<Effect>`.
- Pomodoro con reloj de pared y tick de UI cada 250 ms.
- Sin async ni Tokio.

---

## 5. Modelo de datos

### Principios

1. Markdown es la fuente de verdad del conocimiento.
2. SQLite es caché, índice y estado complementario.
3. El catálogo se reconstruye desde el vault.
4. El estado de aprendizaje, Pomodoro e historial se exportan con `basic export`.
5. Las notas existentes no se modifican automáticamente.

### Tablas principales

- `documents`: ruta, título, categoría, tipo, tamaño, mtime, hash, extracto, `is_stub`, `missing_at`.
- `documents_fts`: FTS5 con título, cuerpo y ruta; tokenizador `unicode61 remove_diacritics 2`.
- `tags`, `document_tags`.
- `links`: enlaces Markdown, wiki, imagen y externos; `resolved`.
- `learning_state`: estado, fechas, `review_due_at`, intervalo, ease, contador.
- `document_visits`, `favorites`, `review_log`.
- `pomodoro_sessions`, `pomodoro_runtime` (snapshot de recuperación).
- `events`, `daily_activity`, `daily_category_activity`.
- `meta` y `PRAGMA user_version` para migraciones.

El esquema completo se materializa en `tui/migrations/0001_init.sql` en F1.

---

## 6. Diseño de la TUI

### Paleta Gruvbox Dark

| Rol | Hex |
|---|---|
| Fondo | `#282828` |
| Panel | `#3c3836` |
| Selección | `#504945` |
| Bordes | `#665c54` |
| Texto | `#ebdbb2` |
| Secundario | `#928374` |
| Acento | `#fe8019` |
| Éxito | `#b8bb26` |
| Advertencia | `#fabd2f` |
| Error | `#fb4934` |
| Enlaces | `#83a598` |

### Layout

| Tamaño | Comportamiento |
|---|---|
| ≥ 120×32 | Sidebar + contenido + panel derecho |
| 90–119 | Sidebar + contenido; panel derecho oculto |
| 70–89 | Una columna con tabs |
| < 70×20 | Aviso de terminal pequeña |

### Pantallas

- **Dashboard:** continuar, actividad, hoy, acciones rápidas.
- **Explore:** árbol, búsqueda, filtros, preview.
- **Learn:** continuar, explorar, pendientes, repasar, aleatorio.
- **Review:** cola del día.
- **Pomodoro:** timer, fase, ciclo, documento vinculado.
- **Stats:** heatmap de 365 días, rachas, distribución.

### `basic learn`

Debe ofrecer cinco entradas:

1. **Continuar:** últimos documentos en `in_progress` o `needs_review`.
2. **Explorar:** catálogo por categoría.
3. **Pendientes:** `pending` y documentos `is_stub`.
4. **Repasar:** cola con `review_due_at <= hoy`.
5. **Aleatorio:** documento de una categoría elegida, excluyendo índices y vacíos por defecto.

### `basic search`

Alcances independientes y combinables:

- **Título:** fuzzy sobre H1 y nombre de archivo.
- **Contenido:** FTS5 sobre texto extraído.
- **Ruta:** fuzzy sobre ruta relativa.
- **Categoría:** filtro exacto por segmento.
- **Código:** FTS5 sobre bloques de código indexados aparte o sobre el cuerpo con lenguaje detectado.

La UI muestra el alcance activo y el ranking. La CLI admite `--scope title|content|path|category|code|all`.

### Heatmap de actividad real

Se registra actividad cuando existe una acción de aprendizaje, no una simple apertura:

- Pomodoro de concentración completado.
- Repaso completado.
- Documento creado o modificado.
- Estado de aprendizaje cambiado.
- Tiempo de lectura/estudio acumulado de forma explícita.

Abrir un documento solo actualiza `last_opened_at` y el historial; no pinta el heatmap. La intensidad se calcula con minutos de estudio: `0`, `1–14`, `15–29`, `30–59`, `60+`.

### Markdown en la TUI

- `pulldown-cmark` → AST → `ratatui::text::Line`.
- `syntect` para bloques de código, con tema Gruvbox y caché.
- Tablas: alineación visual; vista clave-valor en anchos pequeños.
- Enlaces: referencias `[n]`; internos navegables; externos con `xdg-open`.
- Imágenes: placeholder.
- HTML: strip.
- Frontmatter: oculto en el preview; visible en metadatos.

---

## 7. Obsidian

| Riesgo | Mitigación |
|---|---|
| Obsidian indexa `tui/` | Excluir `tui/` en “Excluded files”; el escáner también lo ignora |
| `target/` gigante | `.gitignore` + exclusión explícita |
| SQLite dentro del vault | BD en XDG |
| Frontmatter automático | Solo notas nuevas; nunca notas existentes |
| 42 `readme.md` | Enlaces Markdown relativos; wikilinks calificados si se usan |
| Sync activo | Escritura atómica + comprobación de mtime + backup |
| Case-insensitive | Comprobar colisiones al crear |
| Plantillas de Obsidian | Plantillas de la app en `tui/templates/` |
| Tema Omarchy | La TUI usa Gruvbox propio |

---

## 8. Roadmap

| Fase | Alcance |
|---|---|
| F0 | Auditoría y diseño |
| F1 | Paquete Rust, config XDG, logging, CLI, `doctor`, migraciones, CI |
| F2 | TUI y dashboard Gruvbox |
| F3 | Escáner e indexador |
| F4 | Explorer y búsqueda |
| F5 | Visor Markdown |
| F6 | Creación y edición |
| F7 | Learning Mode |
| F8 | Pomodoro |
| F9 | Estadísticas y heatmap |
| F10 | Repaso inteligente |
| F11 | Pruebas, documentación y optimización |

Cada fase incluye pruebas, documentación y commits pequeños en `dev`. No se hace merge a `main` ni despliegue sin autorización.

---

## 9. Alcance de F1

Incluye:

- Paquete `tui/` con edición 2024.
- Configuración XDG en TOML.
- Logging a archivo.
- CLI con `basic doctor`.
- Migración inicial `0001_init.sql`.
- Pruebas unitarias e integración.
- CI para fmt, clippy, tests y build release.

No incluye:

- Indexado de notas.
- TUI.
- Búsqueda.
- Creación/edición.
- Modificación del proyecto PHP.
- Reorganización de documentación.
- Limpieza del historial Git.
