# F17 — Ejercicios fáciles en la TUI

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F16 (`a21b556`)
- **Estado:** completado

---

## Objetivo

Cerrar el ciclo de práctica dentro de BASIC: elegir un ejercicio fácil de C#,
editar código, guardar el borrador, ejecutar los casos de prueba y conservar el
progreso, sin salir de la terminal ni depender de servicios externos.

## Pantallas

- **[7] code lab**: lista de ejercicios `Easy` con nivel, tema y estado.
- **ejercicio**: columna izquierda con enunciado, ejemplos visibles, recuento de
  casos ocultos y resultado; columna derecha con el editor Gruvbox reutilizable.

En terminales de 70–99 columnas el ejercicio se apila en dos filas. Los atajos
reservados (`Ctrl+S`, `Ctrl+Enter`, `Ctrl+V`, `Esc`) los decide la aplicación;
el resto de teclas pertenecen al editor.

## Flujo de ejecución

1. `Ctrl+S` guarda el borrador en `code_lab_drafts`.
2. `Ctrl+Enter` marca `pending_run`. El bucle dibuja el estado «Ejecutando…»
   antes de bloquear la terminal con `dotnet`.
3. El runner construye un workspace temporal, detecta el SDK, restaura, compila
   y ejecuta cada caso con `stdin` propio.
4. `AttemptSummary` convierte el resultado del runner en datos de persistencia.
   En una sola transacción se guardan el intento, su código, las trazas por
   fase y el progreso derivado.
5. La UI muestra solo los casos visibles; los ocultos aparecen como
   `ocultos: n/m superados`. Su salida esperada nunca llega al buffer de la
   terminal.

## Esquema v4

`migrations/0004_code_lab_practice.sql` añade:

- `code_lab_drafts`: último código editado por ejercicio.
- `code_lab_attempt_code`: código exacto usado en cada intento.

No altera tablas anteriores. La reversión está documentada en `.down.sql`.

## Reglas de progreso

- Abrir un ejercicio actualiza `last_opened_at` pero no incrementa `attempts`.
- Un intento fallido marca `attempted` y guarda el mejor número de casos
  superados y su porcentaje.
- Solo un intento con todos los casos correctos marca `solved` y fija
  `solved_at`.
- `seconds_spent` no se toca al ejecutar: Pomodoro y Code Lab siguen teniendo
  contadores independientes.
- La solución de referencia se lee con una consulta explícita (`Ctrl+V`) y no
  forma parte del modelo `Exercise` que recibe la pantalla.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  117 pasan (105 lib + 12 integración)
cargo build --locked --release                                     OK
```

Prueba ignorada ejecutada manualmente con el SDK real:

```text
cargo test --locked --lib app::tests::real_sdk_solves_the_first_easy_exercise -- --ignored
```

Resultado: .NET `10.0.400` detectado, solución de referencia compilada y 4/4
casos superados, con intento y borrador persistidos.

## Siguiente fase

F18 añadirá los ejercicios `Medium` y `Hard`, con filtros por dificultad y
revisión de conceptos, reutilizando este mismo espacio de trabajo.
