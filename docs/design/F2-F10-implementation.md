# F2–F10 — Implementación de la TUI y el aprendizaje

- **Fecha:** 2026-11-13
- **Rama:** `dev`
- **Estado:** implementado y verificado en un entorno aislado
- **Alcance:** no se modificaron notas del vault, el proyecto PHP ni el historial de Git

---

## 1. Resumen

Sobre la base de F1 se añadieron las fases F2–F10 en una arquitectura por capas:

| Capa | Responsabilidad |
|---|---|
| `domain` | Modelos, estados, SM-2, actividad y formato |
| `app` | Estado de pantallas, acciones, Pomodoro y prompts |
| `infra` | Markdown, indexado, catálogo, notas y SQLite |
| `ui` | Ratatui, Crossterm, tema Gruvbox y render Markdown |

`main.rs` es el composition root: resuelve XDG, carga configuración, abre la base
y despacha CLI o TUI. `app` e `infra` no dependen de Ratatui, así que la lógica se
prueba sin terminal.

Markdown sigue siendo la fuente de verdad. SQLite (`~/.local/share/basic/basic.db`)
es un índice reconstruible y un almacén de estado complementario. Ninguna nota
existente se reescribe.

## 2. Fases

### F2 — TUI y dashboard

- Ratatui 0.29 + Crossterm 0.28, sin Tokio.
- Pantallas: inicio, explorar, lectura, aprender, repaso, pomodoro y estadísticas.
- Tema Gruvbox Dark con colores RGB.
- Layout adaptable; aviso por debajo de 70×20 y vista de una columna por debajo
  de 100 columnas.
- Restauración de terminal incluso si el bucle falla.

### F3 — Escáner e indexador

- Recorrido con `walkdir`, sin seguir symlinks por defecto.
- Exclusión por prefijos: `.git`, `.obsidian`, `.trash`, `tui`, `target`,
  `node_modules`, configurables en `scan.exclude`.
- Hash SHA-256 para detectar cambios; actualización incremental.
- Detección de archivos ausentes mediante `missing_at`.
- FTS5 con `unicode61 remove_diacritics 2`.
- Etiquetas de frontmatter e inline, enlaces Markdown, wiki e imágenes.
- Resolución de enlaces relativos y wiki; los rotos quedan con `resolved = 0`.
- Título de respaldo desde el nombre del archivo o la carpeta para `README.md`.

### F4 — Explorador y búsqueda

- Búsqueda FTS por contenido y código.
- Búsqueda difusa por título, ruta y categoría con `fuzzy-matcher`.
- Resultados combinados sin duplicados y ordenados por relevancia.
- Vista previa con estado, categoría, minutos y extracto.

### F5 — Visor Markdown

- Renderizador propio sobre `pulldown-cmark` 0.12.
- Encabezados, párrafos, listas, citas, código, tablas, reglas, tareas, enlaces,
  énfasis e imágenes como marcadores.
- HTML inline se ignora; el ancho se delega al wrapping de Ratatui.

### F6 — Creación y edición

- `basic new` y `n` en la TUI.
- Solo crea archivos inexistentes; rechaza rutas absolutas y `..`.
- Frontmatter opcional (`title`, `created`, `tags`) y encabezado H1.
- Edición con el editor externo configurado; al volver se recarga la nota.
- No hay edición embebida ni sobrescritura automática.

### F7 — Learning Mode

Modos: continuar, explorar, pendientes, repasar y aleatorio. El estado se guarda
en `learning_state` (`pending`, `in_progress`, `learned`, `needs_review`).
Abrir una nota pendiente la pasa a `in_progress`; `s` cicla el estado.

### F8 — Pomodoro

- Fases de concentración, descanso corto y descanso largo.
- Ciclos configurables y `auto_start_next`.
- Sesiones y snapshots en `pomodoro_sessions` y `pomodoro_runtime`.
- Solo las concentraciones completadas suman tiempo de estudio y pomodoros.
- Al arrancar, una sesión interrumpida se reanuda en pausa; si ya habría
  terminado, se cierra y se registra una sola vez.

### F9 — Estadísticas y heatmap

- Totales, pendientes, en curso, repasos vencidos, documentos vacíos.
- Estudio del día, pomodoros y racha.
- Actividad diaria y por categoría.
- Heatmap de 365 días alineado al lunes, con escala de cinco niveles.

### F10 — Repaso

SM-2 simplificado y determinista:

| Calificación | Intervalo | Factor de facilidad |
|---|---|---|
| Otra vez | 1 día | −0.20 |
| Difícil | ×1.2 (mín. 1) | −0.15 |
| Bien | 1 → 3 → ×ease | igual |
| Fácil | 3 → 5 → ×ease×1.3 | +0.15 |

El factor se limita a `[1.3, 3.0]`. Cada repaso queda en `review_log`, actualiza
`review_due_at` y suma actividad del día.

## 3. CLI

Se mantienen `doctor`, `paths` y `config`. Se añaden:

```text
basic tui
basic index [--json]
basic search [--scope all|title|content|path|category|code] [--limit N] [--json] <términos>
basic stats [--days N] [--json]
basic new <ruta> [--title T] [--no-index] [--json]
basic review [--json]
```

Sin subcomando, en una TTY se abre la TUI; con stdout o stdin redirigidos se
imprime la ayuda para no bloquear scripts y CI.

`index`, `search`, `stats`, `review` y la TUI indexan automáticamente si la base
está vacía. Esto no toca Markdown: solo lee el vault y escribe SQLite fuera de él.

## 4. Pruebas y verificación

- Pruebas unitarias de dominio, Markdown, indexado, catálogo, notas y Pomodoro.
- Pruebas de integración del binario con HOME/XDG temporales.
- Flujo verificado con el vault real en `/tmp/opencode/basic-smoke`:
  - 72 documentos indexados, 0 errores.
  - Búsqueda, estadísticas y repaso consultados sin modificar notas.
  - TUI probada en una PTY de 120×40: navegación, búsqueda, apertura, cambio de
    estado, favorito, repaso y Pomodoro.
- `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test` y
  `cargo build --release` en verde.

## 5. Limitaciones conocidas

- Sin resaltado de sintaxis por lenguaje; el código usa un estilo uniforme.
- Las imágenes se muestran como marcadores, no se renderizan.
- La edición es externa; no hay editor embebido.
- Solo existe el tema Gruvbox Dark.
- El heatmap usa la fecha local del sistema, sin zona horaria configurable.
- El índice es de un único vault por base de datos.
