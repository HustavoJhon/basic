# F19 — Repaso, progreso y estadísticas de Code Lab

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F18 (`0bcc87a`)
- **Estado:** completado

---

## Objetivo

Unir Code Lab con el resto del sistema de aprendizaje: lo que queda pendiente
debe aparecer en Learning Mode, los conceptos deben tener su propia cola SM-2 y
el progreso debe poder consultarse tanto en la TUI como desde la CLI.

## Cambios

### Persistencia (sin migración nueva)

`seed_catalog` inserta los conceptos declarados en lecciones y ejercicios en
`code_lab_concept_review` con `INSERT OR IGNORE`. La tabla ya existía desde v2;
no se altera su esquema y una reseed nunca sobrescribe intervalos ni fallos del
usuario.

Nuevas consultas en `code_lab/infra.rs`:

- `code_lab_stats`: cuenta lecciones completadas, ejercicios resueltos, intentos
  totales y superados, tiempo acumulado y conceptos vencidos.
- `due_concepts`: conceptos sin fecha o con `due_at <= ahora`.
- `rate_concept`: aplica `schedule_review` (la misma curva SM-2 de las notas),
  incrementa `lapses` con `Again` y devuelve el próximo intervalo.
- `learning_items`: lecciones `in_progress`/`needs_review` y ejercicios
  `attempted`/`needs_review`, ordenados por nivel.

`CodeLabService` expone estas operaciones; la UI y la CLI no consultan SQLite
directamente.

### Learning Mode y lecciones

`LearnMode` gana un sexto modo, `[6] code lab`. En ese modo la lista no son
notas, sino los elementos pendientes de Code Lab. `Enter` abre un ejercicio en
el editor o una lección en una pantalla nueva con contenido Markdown y
cuestionario:

- `1`–`4` seleccionan la respuesta de la pregunta actual.
- `j`/`k` cambian de pregunta; `PageUp`/`PageDown` desplazan el contenido.
- `Enter` envía el cuestionario. Solo un acierto total marca `completed`;
  cualquier otro resultado marca `needs_review` y muestra la respuesta
  correcta.

Abrir una lección sigue sin completarla: `open_lesson` solo cambia
`not_started` a `in_progress`.

### Repaso de conceptos

`8` abre la cola de conceptos. El concepto actual se muestra con su idioma,
nivel, intervalo, facilidad y fallos; la lista inferior muestra los siguientes.
`1`–`4` califican y sacan el concepto de la cola hasta su próxima fecha. El
tiempo invertido aquí no se suma al Pomodoro ni a `seconds_spent`.

### Estadísticas

La pantalla `Stats` añade un panel de Code Lab con:

- lecciones completadas / totales,
- ejercicios resueltos / totales,
- intentos superados / totales,
- conceptos pendientes,
- tiempo acumulado de Code Lab.

En terminales anchas el panel va debajo del heatmap; en estrechas se apila
después del resumen. `basic code-lab stats [--json]` expone el mismo agregado.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  128 pasan (116 lib + 12 integración)
cargo build --locked --release                                     OK
```

No se necesitó ejecutar el SDK de .NET: esta fase no toca el runner. Las tres
pruebas ignoradas siguen disponibles para verificar borradores y soluciones con
un SDK real.

## Siguiente fase

F20 añadirá notas de conceptos y enlaces desde Code Lab al vault Markdown,
manteniendo Markdown como fuente de verdad y SQLite como índice.
