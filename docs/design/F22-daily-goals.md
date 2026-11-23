# F22 — Objetivos diarios y exportación del progreso

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F21 (`49db257`)
- **Estado:** completado

---

## Objetivo

Cerrar Code Lab con una meta diaria configurable y una forma de sacar el
progreso del programa sin depender de servicios externos.

## Configuración

`Config` gana un bloque opcional `[code_lab]` con `daily_lessons` (1 por
defecto) y `daily_exercises` (2 por defecto). Como el resto de la configuración,
usa `serde(default, deny_unknown_fields)`: los archivos antiguos siguen siendo
válidos. `doctor` valida que los valores estén entre 0 y 20 / 0 y 50.

## Conteo diario

`daily_progress` consultaSQLite directamente:

- lecciones con `status = 'completed'` y `completed_at` dentro del día local,
- ejercicios distintos con un intento `passed` y `created_at` dentro del día.

El día se calcula con `chrono::Local`, igual que el resto de la aplicación. No
se introduce tabla ni migración nueva: la información ya estaba en v2 y v3.

## Presentación

La pantalla de estadísticas añade un panel `objetivos diarios · YYYY-MM-DD` con
ambos contadores y un color que indica si se alcanzaron. En terminales anchas se
apila bajo el panel de Code Lab; en estrechas va al final de la columna.

## Exportación

`basic code-lab export [--format markdown|json] [--output <ruta>]` construye un
`ProgressReport` con:

- fecha de generación,
- `CodeLabStats`,
- objetivos del día,
- enlaces a notas Markdown.

El formato Markdown es legible en cualquier editor; el JSON es el mismo modelo
serializado. `--output` crea directorios padre y se niega a escribir dentro del
vault para no convertir un informe en una nota. Sin `--output`, el informe va a
stdout y puede redirigirse.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  138 pasan (126 lib + 12 integración)
cargo build --locked --release                                     OK
```

Se probaron los valores por defecto y su validación, el conteo de intentos y
lecciones del día, el panel de la TUI y los tres caminos de la CLI (Markdown,
JSON y archivo, incluido el rechazo de rutas dentro del vault).

## Estado final de Code Lab

Con F22 el módulo cubre catálogo declarativo, lecciones con cuestionario,
ejercicios con runner real de C#, borradores, intentos, dificultad, repaso SM-2
de conceptos, estadísticas, puente con notas Markdown, navegación inversa y
exportación. No quedan fases pendientes en el roadmap conocido.
