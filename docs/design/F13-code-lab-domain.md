# F13 — Dominio y esquema de Code Lab

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F12 (`8e314a1`)
- **Estado:** completado

---

## Objetivo

Preparar la base de Code Lab sin introducir UI, ejecución de código ni
dependencias nuevas. Esta fase solo define el dominio y un esquema SQLite
adicional que convive con F1–F10.

## Arquitectura

```
tui/src/code_lab/
├── mod.rs        # punto de entrada del módulo
└── domain.rs     # modelos y reglas puras, sin I/O
```

La capa de dominio no importa `rusqlite`, `ratatui`, `std::process` ni ningún
crate de red. Las fases posteriores añadirán `catalog`, `app`, `runners`, `ui` y
el catálogo de ejercicios encima de estos tipos.

## Migración v2

`migrations/0002_code_lab.sql` añade únicamente tablas nuevas:

| Tabla | Responsabilidad |
|---|---|
| `code_lab_languages` | lenguajes, comando SDK y versión detectada |
| `code_lab_paths` | rutas de aprendizaje por lenguaje |
| `code_lab_lessons` | lecciones teóricas en Markdown |
| `code_lab_exercises` | enunciados, pistas, starter y solución de referencia |
| `code_lab_lesson_progress` | estado, cuestionario y tiempo por lección |
| `code_lab_exercise_progress` | intentos, mejor resultado y tiempo por ejercicio |
| `code_lab_attempts` | intentos con duración, estado y resultado de compilación |
| `code_lab_executions` | trazas de detect/restore/build/run/test |
| `code_lab_concept_review` | repaso espaciado por concepto |
| `code_lab_favorites` | favoritos de lecciones y ejercicios |
| `code_lab_session_links` | vínculo opcional entre Pomodoro y Code Lab |

Propiedades:

- La migración se aplica en transacción y avanza `user_version` a `2`.
- No altera ni borra tablas de v1; una base v1 con datos se actualiza intacta.
- `0002_code_lab.down.sql` documenta la reversión, pero no se ejecuta sola:
  exige copia de seguridad y ajuste manual de `user_version`.
- La solución de referencia existe en la base, pero el modelo `Exercise`
  devuelto a la UI no la contiene. Solo `ExerciseSolution` la expone cuando se
  pide explícitamente.

## Reglas de dominio

- Abrir una lección cambia `not_started`/`needs_review` a `in_progress`; nunca
  la marca como completada.
- Completar una lección exige acertar todas las preguntas del cuestionario.
- Un ejercicio pasa a `solved` solo si pasan todos los casos de prueba.
- Si un ejercicio resuelto falla después, pasa a `needs_review` sin perder el
  hecho de que fue resuelto.
- Los casos ocultos no aparecen en `Exercise::visible_cases()`.
- El progreso guarda el mejor resultado, no solo el último intento.
- El tiempo se acumula en contadores propios de Code Lab para no duplicar el
  tiempo ya registrado por Pomodoro.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  77 pasan (66 lib + 11 integración)
cargo build --locked --release                                     OK
```

Incluye una prueba que crea una base v1 real, aplica la migración v2 y
comprueba que los datos anteriores siguen disponibles.

## Siguiente fase

F14 añadirá el editor TUI. Se evaluará `tui-textarea 0.7.0` frente a Ratatui
0.29 antes de decidir la dependencia; el editor no debe conocer los runners.
