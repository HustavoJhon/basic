# F18 — Dificultad media y difícil

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F17 (`d753395`)
- **Estado:** completado

---

## Objetivo

Completar el catálogo de práctica disponible en la TUI: los seis ejercicios de C#
(fáciles, medios y difíciles) se pueden filtrar, abrir y ejecutar con el mismo
editor y el mismo runner, sin cambiar el modelo de progreso.

## Cambios

- `CodeLabState::difficulty` recuerda el filtro activo; `1`/`2`/`3` cambian entre
  fácil, medio y difícil.
- `refresh_code_lab` carga el progreso de **todos** los ejercicios para que el
  cabezal pueda mostrar `resueltos/total` del filtro actual.
- La cabecera de Code Lab muestra las tres opciones con el nivel activo en
  acento Gruvbox. El pie y la ayuda documentan los atajos.
- No se añadió esquema nuevo: la tabla `code_lab_exercises` ya contiene
  `difficulty`, `level` y `test_spec_json` desde v2.

## Catálogo verificado

| Dificultad | Ejercicios |
|---|---|
| Fácil | suma-dos-numeros, saludo-nombre, numero-par |
| Medio | maximo-tres, invertir-cadena |
| Difícil | frecuencias-palabras |

La prueba ignorada `real_sdk_solves_every_catalogued_exercise` ejecuta las seis
soluciones de referencia con .NET real. Detectó dos errores en los datos del
catálogo, ya corregidos:

- `invertir-cadena`: la salida esperada del caso con espacios era incorrecta.
- `frecuencias-palabras`: el ejemplo decía `one` en lugar de `uno`.

El runner y la UI no cambiaron; solo los casos declarativos.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  119 pasan (107 lib + 12 integración)
cargo build --locked --release                                     OK
```

Pruebas ignoradas ejecutadas manualmente:

```text
cargo test --locked --lib app::tests::real_sdk_solves_the_first_easy_exercise -- --ignored
cargo test --locked --lib app::tests::real_sdk_solves_every_catalogued_exercise -- --ignored
```

Ambas pasan con .NET `10.0.400`.

## Siguiente fase

F19 conectará el progreso de lecciones y ejercicios con Learning Mode, repaso de
conceptos y una vista de estadísticas de Code Lab.
