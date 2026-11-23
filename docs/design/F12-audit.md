# F12 — Auditoría técnica y estabilización

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Estado:** auditoría completada; F2–F11 estabilizadas
- **Punto de partida:** `dev`, `main` y `origin/dev` en `1e9621d`

---

## 1. Estado real del repositorio

Verificado con `git status`, `git branch -vv`, `git remote -v` y `git log`:

- `origin` apunta a `git@github.com:HustavoJhon/basic.git`.
- `dev`, `main` y `origin/dev` estaban sincronizados en `1e9621d`.
- El árbol de trabajo solo tenía dos archivos sin trackear:
  - `docs/design/F2-F10-implementation.md` (documentación propia, incorporada en F12).
  - `Conversation – The forgotten backpack.md` (nota personal, **excluida de todos los commits**).
- No se encontraron cambios sin commit en código fuente, Cargo.toml, Cargo.lock,
  migraciones, README ni pruebas.

## 2. Fases F1–F11 verificadas en código

| Fase | Estado real | Evidencia |
|---|---|---|
| F0 | Completa | `docs/design/F0-audit.md` |
| F1 | Completa | XDG, logging, CLI base, `doctor`, `0001_init.sql`, CI |
| F2 | Completa | `ui.rs` con siete pantallas y tema Gruvbox |
| F3 | Completa | `infra/index.rs`, FTS5, enlaces, `missing_at` |
| F4 | Completa | `infra/catalog.rs`, FTS + `fuzzy-matcher` |
| F5 | Completa | `ui/markdown.rs` sobre `pulldown-cmark` 0.12 |
| F6 | Completa | `infra/notes.rs`, `basic new`, editor externo |
| F7 | Completa | `LearningMode` y estados en `learning_state` |
| F8 | Completa con observación | Pomodoro, persistencia y recuperación |
| F9 | Completa | `dashboard_stats`, `activity_days`, heatmap alineado al lunes |
| F10 | Completa | SM-2 determinista y `review_log` |
| F11 | Parcial | Tests y documentación sí; falta hardening final |

## 3. Hallazgos corregidos en F12

1. **Salida de depuración filtrada a stderr:** dos `eprintln!` temporalmente
   introducidos durante la depuración de la recuperación del Pomodoro seguían en
   `app.rs` y `infra/catalog.rs`. Se eliminaron. `main.rs` conserva los mensajes
   legítimos de error y aviso.
2. **Recuperación de Pomodoro:** el flujo de sesiones vencidas ya cierra la sesión
   y contabiliza el tiempo una sola vez; las pruebas `app::tests` lo cubren.
3. **Documentación sin trackear:** `F2-F10-implementation.md` se incorpora al
   repositorio en este bloque.
4. **Sin errores de integración:** `cargo check --all-targets` termina limpio.

## 4. Validación ejecutada

```text
cargo check --all-targets                  OK
cargo fmt --all --check                    OK
cargo clippy --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets          70 pasan (59 lib + 11 integración)
cargo build --locked --release             OK
```

También se verificó en una sesión anterior una PTY de 120×40 con el vault real
en un HOME aislado: navegación, búsqueda, apertura, estado, favorito, repaso y
Pomodoro funcionaron sin modificar notas.

## 5. Entorno para Code Lab

- .NET SDK detectado: `dotnet 10.0.400`. No se instala ni modifica nada del
  sistema; Code Lab solo invocará el SDK disponible.
- Candidata evaluada para el editor: `tui-textarea 0.7.0` (compatible con
  Ratatui, sin backend por defecto y con soporte Crossterm). Se decidirá en F14
  tras comprobar la versión de Ratatui que usa.
- Se mantiene la restricción de dependencias síncronas: nada de Tokio, sqlx,
  git2 ni reqwest.

## 6. Ajustes al roadmap

- F12 queda dedicada a estabilización y no introduce funcionalidad nueva.
- F13 creará el dominio de Code Lab y el esquema de migración, sin UI todavía.
- F14 añadirá el editor con dependencia de widget real si es compatible.
- F15 implementará el adaptador C# con projects temporales fuera del vault.
- Las fases F16–F22 se ejecutarán en bloques pequeños con validación y push
  después de cada una.

No se fusiona `dev` en `main`, no se instalan paquetes del sistema y no se
modifican credenciales, SSH, notas, PHP ni historial.
