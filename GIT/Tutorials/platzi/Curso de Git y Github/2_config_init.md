# Configuracion de Git y git init 

**Resumen**

Trabajar con Git en la terminal permite a los desarrolladores gestionar sus proyectos de manera eficiente. A continuación, revisamos cómo instalar, configurar y utilizar Git en Linux, Mac y WSL de Windows, junto con algunas recomendaciones prácticas para dominar los comandos iniciales de esta herramienta.

## ¿Cómo confirmar que Git está instalado en tu sistema?
Para verificar la instalación de Git:

1. Abre la terminal y escribe el comando git --version.
2. Si el comando devuelve un número de versión, Git está listo para usarse.
3. Si no aparece la versión, revisa los recursos adjuntos donde se explican las instalaciones para cada sistema operativo.

## ¿Cómo crear y preparar el primer proyecto con Git?
El primer paso para crear un proyecto en Git es:

1. Limpia la terminal para evitar confusión visual.
2. Crea una carpeta para el proyecto con `mkdir nombre_del_proyecto`.
3. Navega a la carpeta con cd nombre_del_proyecto.

## ¿Cómo inicializar un repositorio en Git?
Al estar dentro de la carpeta de tu proyecto, inicia el repositorio con:

- `git init`: Esto crea la rama inicial “master” por defecto.
Si prefieres la rama principal como “main”:

1. Cambia la configuración global escribiendo `git config --global init.defaultBranch main`.
2. Actualiza la rama en el proyecto actual con git branch -m main.

## ¿Cómo personalizar tu configuración de usuario en Git?
Configura el nombre de usuario y correo electrónico de Git, que identificará todas tus contribuciones:

1. Usa `git config --global user.name "Tu Nombre o Apodo"`.
2. Configura el correo electrónico con `git config --global user.email "tu.email@example.com"`.

**Tip:** Si necesitas corregir algún error en el comando, puedes usar la tecla de flecha hacia arriba para recuperar y editar el último comando escrito.

## ¿Cómo confirmar la configuración de Git?
Para revisar tu configuración, ejecuta:

`git config --list`: Aquí verás los datos de usuario y el nombre de la rama principal.
Esta configuración se aplicará a todos los repositorios que crees en adelante.

## ¿Qué hacer si olvidas un comando?
Git incluye un recurso rápido y útil para recordar la sintaxis de comandos:

1. Escribe git help en la terminal.
2. Navega la lista de comandos disponibles y consulta la documentación oficial de cada uno cuando sea necesario.

## Lecturas recomendadas

- [Git](https://git-scm.com/)

- [Git - git-init Documentation](https://git-scm.com/docs/git-init)

- [Git Cheat Sheet - GitHub Education](https://education.github.com/git-cheat-sheet-education.pdf)

- [Git - git-config Documentation](https://git-scm.com/docs/git-config)

- [Git - Configurando Git por primera vez](https://git-scm.com/book/es/v2/Inicio---Sobre-el-Control-de-Versiones-Configurando-Git-por-primera-vez)

- [Configurar Windows para WSL - Platzi](https://platzi.com/home/clases/6900-configuracion-windows/60922-configurar-windows-11-para-soportar-la-instalacion/)

- [GitHub · Build and ship software on a single, collaborative platform · GitHub](https://github.com/)


