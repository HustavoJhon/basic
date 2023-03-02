# Uso de GitHub 

GitHub es una plataforma que nos permite guardar repositorios de Git que podemos usar como servidores remotos y ejecutar algunos comandos de forma visual e interactiva (sin necesidad de la consola de comandos).

Luego de crear nuestra cuenta, podemos crear o importar repositorios, crear organizaciones y proyectos de trabajo, descubrir repositorios de otras personas, contribuir a esos proyectos, dar estrellas y muchas otras cosas.

El `README.md` es el archivo que veremos por defecto al entrar a un repositorio. Es una muy buena práctica configurarlo para describir el proyecto, los requerimientos y las instrucciones que debemos seguir para contribuir correctamente.

Para clonar un repositorio desde GitHub (o cualquier otro servidor remoto) debemos copiar la URL (por ahora, usando `HTTPS`) y ejecutar el comando `git clone` + la URL que acabamos de copiar. Esto descargará la versión de nuestro proyecto que se encuentra en GitHub.

Sin embargo, esto solo funciona para las personas que quieren empezar a contribuir en el proyecto.

### Cómo conectar un repositorio de GitHub a nuestro documento local
Si queremos conectar el repositorio de GitHub con nuestro repositorio local, que creamos usando el comando git init, debemos ejecutar las siguientes instrucciones:

1. Guardar la URL del repositorio de GitHub con el nombre de origin
```git
git remote add origin URL
```
2. Verificar que la URL se haya guardado correctamente:
```git
git remote
git remote -v
```
3. Traer la versión del repositorio remoto y hacer merge para crear un commit con los archivos de ambas partes. Podemos usar `git fetch` y `git merge` o solo `git pull` con el flag `--allow-unrelated-histories`:
```git
git pull origin master --allow-unrelated-histories
```
4. Por último, ahora sí podemos hacer `git push` para guardar los cambios de nuestro repositorio local en GitHub:
```git
git push origin master
```
### Cómo autenticarte en GitHub 2022
Antes de empezar debemos renombrar la rama ‘máster’ a ‘main’, este es el nuevo estándar en GitHub, para esto:

1. Primero nos posicionamos en la rama a la que queremos cambiarle el nombre.
2. Ejecutamos el siguiente comando: git branch -M main
**Pasos para crear un token de acceso personal.**

**Desde el 2022 GitHub** ya no deja hacer el push con la contraseña del propio GitHub, para esto tenemos que crear un token, y este token es la contraseña que vamos a colocar cuando nos pida clave

Seguir la secuencia: Ingresamos a nuestra cuenta de GitHub.

1. Buscamos `Settings`
2. Click en `Developer settings`
3. Click en `Personal access tokens`
4. Click en `Generate new token` aquí se puede colocar un nombre, la fecha de expiración.
5. Tildar en repo y luego click en el botón verde `Generate token`

---

## Cómo funcionan las llaves públicas y privadas

Las **llaves públicas y privadas**, conocidas también como cifrado asimétrico de un solo camino, sirven para mandar mensajes privados entre varios nodos con la lógica de que firmas tu mensaje con una llave pública vinculada con una llave privada que puede leer el mensaje.

Las llaves públicas y privadas nos ayudan a cifrar y descifrar nuestros archivos de forma que los podamos compartir sin correr el riesgo de que sean interceptados por personas con malas intenciones.

### Cómo funciona un mensaje cifrado con llaves públicas y privadas
1. Ambas personas deben crear su llave pública y privada.
2. Ambas personas pueden compartir su llave pública a las otras partes (recuerda que esta llave es pública, no hay problema si la “interceptan”).
3. La persona que quiere compartir un mensaje puede usar la llave pública de la otra persona para cifrar los archivos y asegurarse que solo puedan ser descifrados con la llave privada de la persona con la que queremos compartir el mensaje.
4. El mensaje está cifrado y puede ser enviado a la otra persona sin problemas en caso de que los archivos sean interceptados.
5. La persona a la que enviamos el mensaje cifrado puede emplear su llave privada para descifrar el mensaje y ver los archivos.
> Nota: puedes compartir tu llave pública, pero nunca tu llave privada.

## Configura tus llaves SSH en local

En este ejemplo, aprenderemos cómo configurar nuestras llaves SSH en local.

### Cómo generar tus llaves SSH
1. Generar tus llaves SSH**
Recuerda que es muy buena idea proteger tu llave privada con una contraseña.
```git
ssh-keygen -t rsa -b 4096 -C "tu@email.com"
```
2. Terminar de configurar nuestro sistema.
En Windows y Linux:

Encender el “servidor” de llaves SSH de tu computadora:
```git
eval $(ssh-agent -s)
```
Añadir tu llave SSH a este “servidor”:
```git
ssh-add ruta-donde-guardaste-tu-llave-privada
```

## Conexión a GitHub con SSH
La creación de las SSH es necesario solo una vez por cada computadora. Aquí conocerás cómo conectar a GitHub usando SSH.

Luego de crear nuestras llaves SSH podemos entregarle la llave pública a GitHub para comunicarnos de forma segura y sin necesidad de escribir nuestro usuario y contraseña todo el tiempo.

Para esto debes entrar a la [Configuración de Llaves SSH en GitHub](https://github.com/settings/keys), crear una nueva llave con el nombre que le quieras dar y el contenido de la llave pública de tu computadora.

Ahora podemos actualizar la URL que guardamos en nuestro repositorio remoto, solo que, en vez de guardar la URL con HTTPS, vamos a usar la URL con SSH:
```git
ssh
git remote set-url origin url-ssh-del-repositorio-en-github
```
- Windows (Git Bash):
```git
clip < ~/.ssh/id_rsa.pub
```
- Linux (Ubuntu):
```git
cat ~/.ssh/id_rsa.pub
```

## Tags y versiones en Git y Github

> Los tags o etiquetas nos permiten asignar versiones a los commits con cambios más importantes o significativos de nuestro proyecto.

### Comandos para trabajar con etiquetas:
- Crear un nuevo tag y asignarlo a un commit: `git tag -a nombre-del-tag id-del-commit`.
- Borrar un tag en el repositorio local: `git tag -d nombre-del-tag`.
- Listar los tags de nuestro repositorio local: `git tag` o `git show-ref --tags`.
- Publicar un tag en el repositorio remoto: `git push origin --tags`.
- Borrar un tag del repositorio remoto: `git tag -d nombre-del-tag` y `git push origin :refs/tags/nombre-del-tag`.

Para generar un comando complejo con varios comandos de una forma optimizada, utilizamos conjuntos de sentencias conocidas como alias.

### Cómo aregar un alias solo para git
- Para un proyecto:
```git
git config alias.arbolito "log --all --graph --decorate --oneline"
```
- Global:
```git
git config --global alias.arbolito "log --all --graph --decorate --oneline"
```
- Para correrlo:
```git
git arbolito
```
**plus**
```git
git config --global alias.superlog "log --graph --abbrev-commit --decorate --date=relative --format=format:'%C(bold blue)%h%C(reset) - %C(bold green)(%ar)%C(reset) %C(white)%s%C(reset) %C(dim white)- %an%C(reset)%C(bold yellow)%d%C(reset)' --all"
```

## Manejo de ramas en Github

Si no te funciona el comando gitk es posible no lo tengas instalado por defecto.
Para instalar gitk debemos ejecutar los siguientes comandos:
sudo apt-get update
sudo apt-get install gitk

### Repasa: ¿Qué es Git?

Las ramas nos permiten hacer cambios a nuestros archivos sin modificar la versión principal (master). Puedes trabajar con ramas que nunca envías a GitHub, así como pueden haber ramas importantes en GitHub que nunca usas en el repositorio local. Lo crucial es que aprendas a manejarlas para trabajar profesionalmente.

Si, estando en otra rama, modificamos los archivos y hacemos commit, tanto el historial(git log) como los archivos serán afectados. La ventaja que tiene usar ramas es que las modificaciones solo afectarán a esa rama en particular. Si luego de “guardar” los archivos(usando commit) nos movemos a otra rama (git checkout otraRama) veremos como las modificaciones de la rama pasada no aparecen en la otraRama.

### Comandos para manejo de ramas en GitHub
- Crear una rama:
```git
git branch branchName
```
- Movernos a otra rama:
```git
git checkout branchName
```
- Crear una rama en el repositorio local:
```git
git branch nombre-de-la-rama o git checkout -b nombre-de-la-rama.
```
- Publicar una rama local al repositorio remoto:
```git
git push origin nombre-de-la-rama.
```
Recuerda que podemos ver gráficamente nuestro entorno y flujo de trabajo local con Git utilizando el comando `gitk`. Gitk fue el primer visor gráfico que se desarrolló para ver de manera gráfica el historial de un repositorio de Git.

## Configurar multiples colaboradores en un repositorio de Github

 Por defecto, cualquier persona puede clonar o descargar tu proyecto desde GitHub, pero no pueden crear commits, ni ramas. Esto quiere decir que pueden copiar tu proyecto pero no colaborar con él. Existen varias formas de solucionar esto para poder aceptar contribuciones. Una de ellas es añadir a cada persona de nuestro equipo como colaborador de nuestro repositorio.

###  Cómo agregar colaboradores en Github
- Solo debemos entrar a la configuración de colaboradores de nuestro proyecto. Se encuentra en:
`Repositorio > Settings > Collaborators`
Ahí, debemos añadir el email o username de los nuevos colaboradores.

Si, como colaborador, agregaste erróneamente el mensaje del commit, lo puedes cambiar de la siguiente manera:

- Hacer un commit con el nuevo mensaje que queremos, esto nos abre el editor de texto de la terminal:
```git
git commit —amend
```
- Corregimos el mensaje
- Traer el repositorio remoto
```git
git pull origin master
```
- Ejecutar el cambio
```git
git push --set-upstream origin master
```