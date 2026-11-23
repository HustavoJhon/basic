# F15 — Runner de C#

- **Fecha:** 2026-11-22
- **Rama:** `dev`
- **Depende de:** F14 (`fd3c6a7`)
- **Estado:** completado

---

## Objetivo

Ejecutar ejercicios de C# usando el SDK que ya tenga instalado el usuario, sin
instalar paquetes del sistema, sin servicios externos y sin acoplar la
ejecución al editor ni a la base de datos.

## Arquitectura

```
tui/src/code_lab/runners/
├── mod.rs        # trait LanguageRunner, tipos de solicitud/resultado
├── csharp.rs     # adaptador de .NET
├── process.rs    # ejecución síncrona con timeout y salida acotada
└── workspace.rs  # proyectos temporales fuera del vault
```

`LanguageRunner` expone `detect(vault)` y `run(request)`. Añadir Rust, Python o
JavaScript consistirá en implementar este trait y registrarlo en `runner_for`;
la UI y el progreso no cambiarán.

## Flujo de ejecución

1. **Workspace:** `tempfile` crea `/tmp/basic-code-lab/run-XXXX`. Si ese
   directorio cae dentro del vault configurado, la ejecución se rechaza.
2. **Detección:** `dotnet --version` con timeout propio. Se acepta la versión
   plausible y se deriva `net{major}.0` (por ejemplo, `10.0.400` → `net10.0`).
3. **Generación:** se escriben `BasicRunner.csproj` y `Program.cs` con el código
   del usuario. El proyecto no tiene `PackageReference`.
4. **Restore:** `dotnet restore --source <directorio vacío>` con caché NuGet
   dentro del workspace; no se consulta NuGet.org.
5. **Build:** `dotnet build -c Release --no-restore`.
6. **Casos:** se ejecuta el DLL compilado con `dotnet <dll>`; el `stdin` de cada
   caso se escribe en un archivo del workspace. Se compara la salida ignorando
   espacios finales y líneas vacías al final.
7. **Resultado:** estado del intento, mejor/último resultado, número de casos,
   duración y una traza por fase lista para `code_lab_executions`.

## Seguridad

- Los comandos se construyen con `std::process::Command` y listas de
  argumentos; **no se usa shell ni interpolación de comandos**.
- No se invoca `sudo` ni ningún mecanismo de elevación.
- El directorio de trabajo, `HOME`, `TMPDIR` y las cachés de .NET/NuGet se
  redirigen al workspace temporal.
- La salida se guarda en archivos, no en tuberías, evitando bloqueos por
  buffers llenos. Está limitada a 32 KiB por flujo.
- Cada fase y cada caso tienen timeout con `try_wait()` + `sleep()`; al
  expirar se mata el proceso y se registra el motivo.
- El workspace se elimina automáticamente al terminar.
- **Límite conocido:** no hay sandbox a nivel de sistema operativo. El código
  del usuario se ejecuta con los permisos de la persona; el aislamiento es de
  rutas, entorno y duración, no de capabilities. No se ejecutará código de
  ejercicios que no hayan sido revisados previamente en el catálogo.

## Validación

```text
cargo fmt --all --check                                             OK
cargo clippy --locked --all-targets --all-features -- -D warnings   OK
cargo test --locked --all-targets                                  98 pasan (87 lib + 11 integración)
cargo build --locked --release                                     OK
```

Además se ejecutó la prueba ignorada con el SDK real:

```text
cargo test --locked --lib \
  code_lab::runners::csharp::tests::real_sdk_runs_a_csharp_program \
  -- --ignored
```

Resultado: .NET `10.0.400` detectado, build correcto y 2/2 casos superados sin
acceso a la red. La prueba queda `#[ignore]` para no exigir .NET en CI.

## Siguiente fase

F16 creará el catálogo de lecciones de C# y la capa de aplicación que lee y
escribe progreso, dejando la UI para las fases posteriores.
