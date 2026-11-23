# F16 — Catálogo de lecciones y progreso

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F15 (`a5f9a65`)
- **Estado:** completado

---

## Objetivo

Añadir contenido teórico de C#, cuestionarios y progreso persistente, sin
mezclarlo con las notas Markdown del vault ni con la TUI todavía.

## Capas añadidas

```
tui/src/code_lab/
├── app.rs       # CodeLabService: casos de uso
├── catalog/     # datos declarativos de C#
├── infra.rs     # SQL, seed y progreso
└── ...
```

El catálogo define una ruta (`csharp-fundamentos`), cinco lecciones y seis
ejercicios. Los ejercicios ya quedan sembrados para F17/F18, pero todavía no
tienen pantalla propia.

### Lecciones

1. Tu primer programa
2. Variables y tipos
3. Control de flujo
4. Métodos y colecciones
5. Entradas y pruebas automáticas

Cada lección incluye contenido Markdown, conceptos y dos preguntas de opción
múltiple con explicación. El contenido de apuntes personales sigue viviendo en
el vault; aquí solo está el currículo de Code Lab.

## Esquema v3

`migrations/0003_code_lab_quizzes.sql` añade `code_lab_quiz_questions` con
clave foránea a `code_lab_lessons`. No modifica v1 ni v2 y conserva los datos
existentes. La reversión está documentada en el archivo `.down.sql`.

## Reglas de progreso

- `open_lesson` actualiza `last_opened_at` y puede poner la lección en
  `in_progress`, pero **nunca** en `completed`.
- `submit_quiz` solo marca `completed` si se aciertan todas las preguntas.
- Un fallo parcial guarda el número de aciertos y marca `needs_review` con
  vencimiento a un día.
- `seconds_spent` se acumula por separado. Pomodoro y Code Lab no comparten
  contadores, evitando duplicar tiempo.
- El seed es idempotente: se puede ejecutar en cada arranque sin duplicar filas
  ni borrar progreso.

## CLI incorporada

```bash
basic code-lab seed           # informe legible
basic code-lab seed --json
basic code-lab lessons        # lista ordenada
basic code-lab lessons --json
```

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  107 pasan (95 lib + 12 integración)
cargo build --locked --release                                     OK
```

Las pruebas cubren seed idempotente, apertura sin finalización, cuestionario
incompleto versus perfecto, acumulación de tiempo, lectura de ejercicios sin
solución y el flujo CLI con HOME/XDG aislados.

## Siguiente fase

F17 añadirá la pantalla de ejercicios fáciles con el editor, ejecución y
resultado de casos visibles/ocultos.
