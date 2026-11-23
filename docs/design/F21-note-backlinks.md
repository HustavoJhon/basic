# F21 — Volver al origen desde una nota y plantillas por lenguaje

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F20 (`8f47de6`)
- **Estado:** completado

---

## Objetivo

Cerrar el círculo entre Code Lab y el vault: una nota creada desde una lección,
un ejercicio o un concepto debe poder devolver al elemento de origen sin buscar
a mano. Además, la plantilla deja de estar pegada a C# y pasa a ser declarativa.

## Marcador de origen

Cada nota nueva incluye una primera línea de comentario Markdown:

```html
<!-- basic-code-lab language="csharp" type="lesson" id="3" -->
```

`NoteBacklink::parse` lee ese marcador sin depender de SQLite ni del resto del
contenido. El marcador no se dibuja: el lector filtra la línea antes de pasar el
cuerpo a `markdown::render`. La cabecera añade `c: origen` cuando el documento
tiene un origen reconocido.

`c` en el lector abre:

- la lección, con su cuestionario (abrir no la completa),
- el ejercicio, con su borrador y editor,
- el concepto, situando la cola de repaso en ese elemento.

Si la nota no tiene marcador, `c` solo informa en la barra de estado. El enlace
de F20 en SQLite sigue siendo la fuente para «abrir la nota»; el marcador es la
fuente para «volver al elemento».

## Plantillas por lenguaje

`code_lab/domain.rs` define `NoteTemplate` con `language`, `code_fence`,
`concepts_heading` y `practice_heading`. Hay una plantilla declarada para C# y
una genérica como reserva. La generación del cuerpo en `App` usa esos campos:
las notas de C# escriben bloques `csharp` y secciones «Conceptos de C#» y
«Practica en C#». Añadir un lenguaje nuevo no requiere tocar la TUI.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  135 pasan (123 lib + 12 integración)
cargo build --locked --release                                     OK
```

Se añadieron pruebas del marcador, de las plantillas, del retorno desde el
lector y de que el comentario no aparece en pantalla.

## Siguiente fase

F22 añadirá objetivos diarios de práctica y la exportación del progreso en un
formato legible, sin servicios externos.
