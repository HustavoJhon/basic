# Comandos Basicos de Git: add, commit y log

**Resumen**
Aprender a utilizar Git desde los primeros pasos puede parecer desafiante, pero es esencial para registrar cambios y manejar versiones de cualquier proyecto. Siguiendo un flujo de trabajo sencillo y utilizando los comandos adecuados, puedes dominar el control de versiones y llevar un seguimiento preciso de tus archivos.

## ¿Cómo inicia el control de versiones con Git?
El primer paso es iniciar un repositorio con el comando `git init`, que crea una carpeta oculta llamada `.git` en el directorio de trabajo. Esta carpeta actúa como una bitácora, almacenando cada cambio y movimiento de los archivos que se manejan en el proyecto.

## ¿Cómo se crean y agregan archivos a Git?
Para crear un archivo desde la terminal, utiliza un editor como `nano`. Una vez creado, puedes verificar su existencia y estado con `git status`, que te mostrará el archivo como no registrado. Para incluirlo en el área de staging, donde estará listo para el commit, usa `git add` nombre_del_archivo.txt. Esta área de staging es un “limbo” donde decides qué archivos entrarán en el control de versiones.

- **Ejemplo de comandos:**
    - `nano testing.txt` para crear el archivo.
    - `git add testing.txt` para agregarlo al área de staging.

## ¿Qué es el área de staging y cómo funciona?
El área de staging permite revisar los cambios antes de que se registren oficialmente en el repositorio. Los archivos en staging aún no forman parte del historial de versiones; están en espera de que se realice un commit o de ser devueltos a su estado original con `git rm --cached nombre_del_archivo.txt`.

## ¿Cómo realizar el commit de los archivos en Git?
Una vez en staging, se ejecuta `git commit -m "mensaje descriptivo"` para registrar los cambios en el repositorio. El mensaje en el commit es crucial porque indica la acción realizada, como “nuevo archivo de testing”. Este mensaje permite identificar los cambios de forma clara y ordenada en el historial del proyecto.

- **Ejemplo de commit:**
    - `git commit -m "nuevo archivo de testing"`

## ¿Cómo gestionar múltiples archivos en Git?
Para trabajar con varios archivos a la vez, utiliza `git add .` que agrega todos los archivos sin registrar en el área de staging. Puedes decidir entre realizar commits individuales o múltiples en función de la cantidad de archivos y los cambios realizados en cada uno.

## ¿Cómo visualizar el historial de cambios en Git?
El comando `git log` muestra el historial de commits, proporcionando una vista completa de cada cambio realizado en el proyecto. Esta bitácora permite ver el estado de cada archivo y la información de cada commit.

## ¿Qué sucede al modificar un archivo en Git?
Cuando un archivo se edita, Git lo detecta como “modificado”. El flujo de trabajo para registrar este cambio es el mismo que para un archivo nuevo: `git add` para llevarlo a staging y `git commit` para guardar la modificación. Esto asegura que Git mantenga un registro detallado de cada cambio, actualización o eliminación en el proyecto.

## ¿Cómo maneja Git diferentes tipos de archivos?
Git trata cualquier archivo de igual manera, sin importar su extensión o tipo, ya sea de texto, código o imagen. Con `git add` y `git commit`, cualquier cambio en estos archivos se registra, facilitando el control de versiones sin importar el tipo de contenido.

**Lecturas recomendadas**

- [Git - git-add Documentation](https://git-scm.com/docs/git-add)

- [Git - git-commit Documentation](https://git-scm.com/docs/git-commit)

- [Git - git-log Documentation](https://git-scm.com/docs/git-log)

- [Póngase en marcha - Documentación de GitHub](https://docs.github.com/es/get-started/start-your-journey)
