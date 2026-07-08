# Volviendo en el Tiempo en Git: reset y revert

**Resumen**

Para quienes se inician en el manejo de versiones con Git, comandos como git reset y git revert se vuelven herramientas indispensables, ya que permiten deshacer errores y ajustar el historial de cambios sin complicaciones. Aunque al avanzar en la experiencia puedan dejarse de lado, dominar su uso resulta clave para un control de versiones eficiente.

## ¿Cuál es la diferencia entre Git Reset y Git Revert?
- **Git Reset:** mueve el puntero de los commits a uno anterior, permitiendo “volver en el tiempo” y explorar el historial de cambios. Es útil para deshacer actualizaciones recientes o revisar lo que se hizo en cada commit.
- **Git Revert:** crea un nuevo commit que revierte los cambios de un commit específico, permitiendo conservar el historial original sin eliminaciones. Es ideal para regresar a un estado anterior sin afectar los commits de otros usuarios.
## ¿Cómo se utiliza Git Reset?
1. Ejecuta `git log` para identificar el historial de commits. El commit actual se marca con `HEAD` apuntando a `main`.
2. Si quieres eliminar cambios recientes:
    - Crea un archivo temporal (ejemplo: `error.txt`) y realiza un commit.
    - Verifica el historial con `git log` y localiza el hash del commit que deseas restablecer.
3. Para revertir a un estado anterior:
    - Usa git reset con parámetros:
        - `--soft:` solo elimina el archivo del área de staging.
        - `--mixed:` remueve los archivos de staging, manteniendo el historial de commits.
        - `--hard:` elimina los archivos y el historial hasta el commit seleccionado.
    - Este último parámetro debe ser una última opción debido a su impacto irreversible en el historial.
## ¿Cómo funciona Git Revert?
1. **Identificación del commit:** usa `git log` para encontrar el commit a revertir.
2. **Ejecuta** git revert seguido del hash del commit: crea un nuevo commit inverso, preservando el historial.
3. **Editar el mensaje de commit:** permite dejar claro el motivo de la reversión, ideal en equipos colaborativos para mantener claridad.
## ¿Cuándo es recomendable utilizar Git Reset o Git Revert?
Ambos comandos resultan útiles en diversas situaciones:

- **Corrección de errores:** si has subido un archivo incorrecto, git revert es rápido y seguro para deshacer el cambio sin afectar el historial.
- **Limpieza del historial:** en proyectos sólidos, puede que quieras simplificar el historial de commits; `git reset` ayuda a limpiar entradas innecesarias.
- **Manejo de conflictos:** en casos extremos de conflicto de archivos, `git reset` es útil, aunque puede ser mejor optar por resolver conflictos manualmente.
## ¿Cómo aseguras una correcta comunicación en el uso de estos comandos?
Utiliza estos comandos en sincronización con el equipo.
Evita el uso de `git reset --hard` sin coordinación para prevenir la pérdida de trabajo ajeno.
Documenta cada reversión con un mensaje claro para asegurar el seguimiento de cambios.

**Lecturas recomendadas**
- [Git - git-reset Documentation](https://git-scm.com/docs/git-reset)

- [Git - git-revert Documentation](https://git-scm.com/docs/git-revert)