# F14 — Editor TUI de Code Lab

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F13 (`1b36703`)
- **Estado:** completado

---

## Investigación de dependencias

Se evaluaron las alternativas antes de escribir un editor propio:

| Crate | Resultado | Motivo |
|---|---|---|
| `tui-textarea 0.7.0` | **Elegido** | Depende de `ratatui 0.29.0` y `crossterm 0.28`, exactamente las versiones ya usadas por BASIC. MIT, edición 2024 compatible con MSRV 1.89. |
| `tui-textarea-2 0.13/0.14` | Descartado | Compila contra `ratatui-core 0.1` y `ratatui-widgets 0.3` (generación Ratatui 0.30), no contra el `ratatui 0.29` actual. |
| Editor propio | Descartado | Duplicaría selección, undo, historial, scroll y manejo de UTF-8 sin beneficio funcional. |

No se introdujeron dependencias asíncronas ni servicios externos. La nueva
dependencia es síncrona y se compila embebida en el binario.

## Implementación

```
tui/src/code_lab/ui/
├── mod.rs
└── editor.rs   # CodeEditor sobre tui_textarea::TextArea
```

`CodeEditor` aporta:

- Código inicial y reemplazo posterior preservando saltos de línea finales.
- Tema Gruvbox Dark, numeración de líneas, estilo de línea activa y cursor.
- Estado `dirty` basado en comparación de contenido: mover el cursor no cuenta
  como modificación.
- `mark_saved()` para sincronizar el estado tras persistir.
- Atajos reservados: `Ctrl+S` (guardar), `Ctrl+Enter` (ejecutar) y `Esc`
  (volver). El editor los devuelve sin consumir para que la capa de aplicación
  decida qué hacer.
- Barra de estado que reduce su texto según el ancho: completa, media y
  compacta. En alturas menores a 4 filas dibuja solo el área de texto.
- Títulos y símbolos sin Nerd Font.

## Límites de esta fase

El editor todavía no está conectado a ninguna pantalla: F16 y F17 lo usarán
para lecciones y ejercicios. No guarda archivos, no ejecuta código y no conoce
el lenguaje más allá de la etiqueta mostrada. Esos flujos pertenecen a F15–F18.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  84 pasan (73 lib + 11 integración)
cargo build --locked --release                                     OK
```

Las pruebas incluyen entrada de texto, movimiento de cursor, atajos
reservados, sustitución de contenido y renderizado con `TestBackend` en áreas
de 30×7 y 4×2.

## Siguiente fase

F15 implementará el adaptador C# con detección de `dotnet`, proyectos
temporales fuera del vault, timeouts y trazas por fase en `code_lab_executions`.
