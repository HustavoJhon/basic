# F20 — Notas de Code Lab en el vault Markdown

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F19 (`22eb60e`)
- **Estado:** completado

---

## Objetivo

Permitir que lecciones, ejercicios y conceptos de Code Lab tengan una nota
propia en el vault Markdown, sin convertir SQLite en fuente de verdad ni
reescribir notas del usuario.

## Esquema v5 (aditivo)

`migrations/0005_code_lab_notes.sql` crea `code_lab_note_links` con clave
primaria `(language, item_type, item_id)`, la ruta relativa de la nota, un
`document_id` opcional y `created_at`. El documento se referencia con
`ON DELETE SET NULL`: si el índice se limpia, el enlace sobrevive. La reversión
está documentada en `0005_code_lab_notes.down.sql` y no se aplica
automáticamente.

No se modifica ninguna tabla anterior. `LATEST_SCHEMA_VERSION` pasa a 5 y la
lista de tablas obligatorias incluye la nueva.

## Flujo en la TUI

- `n` en una lección o en el repaso de conceptos, `Ctrl+N` en un ejercicio
  (la tecla `n` pertenece al editor).
- Si ya hay enlace, se abre la nota indexada; si el archivo existe pero no está
  indexado, se informa de la ruta.
- Si no hay enlace, se muestra un prompt con una ruta sugerida:
  - `code-lab/csharp/<slug-de-la-leccion>.md`
  - `code-lab/csharp/ejercicio-<slug>.md`
  - `code-lab/csharp/concepto-<id>.md`
- El cuerpo se genera con el título, el tipo, el lenguaje, los conceptos y el
  contenido disponible. Las lecciones incluyen su Markdown; los ejercicios,
  enunciado, pistas y código inicial; los conceptos, nivel y fallos.
- Al confirmar, `create_note_with_body` crea el archivo **solo si no existe**,
  se reindexa el vault, se guarda el enlace y se abre la nota en el lector.

Las pantallas muestran la ruta enlazada cuando existe. Los atajos y el helper
se documentan en `README.md`.

## Capas

- `code_lab/domain.rs`: `NoteItemType` y `NoteLink`.
- `code_lab/infra.rs`: lectura, upsert y listado de enlaces; consulta de un
  concepto por id.
- `code_lab/app.rs`: `CodeLabService::{note_link, save_note_link, note_links,
  concept}`.
- `infra/notes.rs`: `normalize_note_path`, `create_note_with_body` y
  `render_note_body`; la creación genérica mantiene su comportamiento.
- `infra/catalog.rs`: `document_by_path`.
- `app.rs`: borrador de nota y manejo del prompt.
- `ui.rs`: atajos contextuales y visualización de la ruta.
- CLI: `basic code-lab notes [--json]`.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  131 pasan (119 lib + 12 integración)
cargo build --locked --release                                     OK
```

Las pruebas cubren el ciclo completo del enlace en SQLite, la creación de la
nota desde el prompt, los atajos de las tres pantallas y el nuevo subcomando de
la CLI. El SDK de .NET no interviene en esta fase.

## Siguiente fase

F21 añadirá plantillas de concepto por lenguaje y navegación desde una nota al
ejercicio o lección de origen, manteniendo el enlace como metadato opcional.
