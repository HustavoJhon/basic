# Introduccion a la terminal y Linea de Comandos

**Que es la terminal?** 
> Es una interfaz grafica que simula una linea de comandos. Cuando hablamos de una linea de comandos nos referimos a una shell.

- **Terminal:** Ventana que nos muestra el prompt. Este aloja a la shell.

- **Linea de comandos (shell):** Un programa que toma comandos y los pasa al sistema operativo para hacer algo.

- **Algunos tipos de Shell**
  - Bourner Shell
  - Bash Shell
  - Z Shell
  - C Shell
  - Fish Shell
  - PowerShell

- **Un comando de manera sencilla es:** Un programa que se puede ejecutar desde la terminal. Este puede recibir parametros y opciones.

---

## Caminar en la terminal

> **Definicion** Un comando es una accion escrita, generalmente, a traves de una terminal y emulada por una shell para ser ejecutada por nuestro sistema operativo.

**$**_(prompt)_  **rm**_(command)_ **-rf**_(option)_ **directort**_(argument)_

- `ls` _Lista archivos_
  - **Option**
    - `-l` _**long** el peso de los archivos en bits_
    - `-lh` _Lista archibos para ver su peso de una manera mas legible en K y M_
    - `-a` _Lista archivos ocultos_
    - `-lS` _**size** lo ordena de acuerdo a su tamaño_ 
    - `-lSh` _nos muestra de acuerdo a su tamaño de manera legible en K y M_
    - `-lr` _nos muestra de reversa de Z a la A_
  - **TREE**
    - `tree -L 2` _nos muestra un arbor de directorios en 2 o n niveles_

- `clear` _Limpiar la terminal_

- `cd` _Movemos entre directorios_ 
  - `pushd` _mueve al nuevo directorio y guarda el actual en la pila._
  - `popd` _saca un directorio de la pila y te lleva alla_

- `pwd` _nos muestra el directorio actual_

- `mkdir` _Crear un directorio_ 
  - `rmdir` _Borrar un directorio_ 

- `touch` _crear archivos_

- `cp` _Copiar un archivo_ 

- `rm` _**REMOVE eliminar archivos o capetas_ 
  - `-i` _**Interactive** te pregunta si estas seguro de eliminar el archivo_
  - `-r` _**Recursive** elimina todo lo que este dentro de una carpeta
  - `-f` _**Force** fuerza a borrar todo_

- `mv` _**MOVE** mueve o renombra archivos_ 

- `file` _nos muestra que tipo de archivo es_

## Explorar el contenido de nuestros archivo

- `head` _muestra las primeras 10 lineas de nuestro archivo_
  * example: (head file.md -n 15) > nos muestra las primeras 15 lineas

- `tail` _muestra las ultimas 10 lineas de nuestro archivo_
  * example: (tail file.md -n 20) > nos muestra las 20 ultima lineas_ 

- `less` _muestra todo el contenido de nuestro archivo dentro de la consola_
    * **search** con (/...) podemos buscar palabras 

- `xdg-open` _abre un programa para inspecccionar ese archivo_

- `nautilus` _abre en la interfaz de ventanas la carpeta que seleccines_

## Que es un comando?

> Un comando es un mensaje enviado al ordenador que provoca una respuesta en este sistema y se comporta como una orden, pues informa al dispositivo informático que debe ejecutar una acción según la indicación que pueda enviarse. Cada sistema operativo incorpora un determinado número de comandos básicos, que permiten ejecutar las tareas más simples con órdenes directas. A continuación conocerás todo lo relacionado con sistemas operativos basados en UNIX y sus comandos basicos en la terminal.

1. Un programa ejecutable.
2. Un comando de utilidad de la shell.
3. Una funcion de shell.
4. Un alias. 

- `type` _Muestra que tipo de comando es._
- `help` _Muestra todo lo que podemos hacer con el comando seleccionado._
- `whatis` _Ofrese una descripcion muy cota del comando seleccionado._
- `man` _De manual, nos permite conocer muchas mas informacion de un comando._ 
- `info` _Similar al anterior, pero un poco resumido y con otro formato._

## Wildcards 
> Las wildcards o comodines son una serie de caracteres especiales que nos permiten encontrar patrones o realizar búsquedas más avanzadas. Es aplicable para archivos y directorios. Las wildcards te sirven para realizar seccionamiento de archivos o directorios, ademas de ls los wildcards tambien pueden usarse con cualquier comando que realice la manipulación de archivos como mv, cp y rm.

1. **Buscar todo (*)**
  - El asterisco te ayuda a buscar toda la información dentro de una carpeta, pero puedes limitar su uso. Si por ejemplo quieres buscar los archivos que tengan una extensión “.png”, escribes:

**Example**
`ls -l *.png`

  - También lo puedes poner al final, si quisieras buscar, todos los archivos que comiencen por unos caracteres específicos, entonces escribes esos caracteres y luego el asterisco. Por ejemplo, si quisieras buscar todos los archivos que comiencen por “fotosDe”, habría que escribir:

**Example** 
`ls -l fotoDe*` 

2. **Buscar por cantidad de caracteres (?)** 

Si dentro de tus archivos tuvieras una especie de código para guardar tus fotos, algo así como “foto1.png”, “foto2.png”, “foto3.png”, etc. En este caso, sabemos que primero tenemos el string “foto”, luego un solo número y por último la extensión “.png”.

Si quisieras buscar esas fotos escribirías:
**Example**
`ls -l foto?.png`

- Aquí estás indicando:
  - Busca todo lo que comience por la cadena de caracteres “foto”
  - Que inmediatamente después tenga un solo caracter
  - Y que al final tenga la cadena de caracteres “.png”
_Pero si sabes que no tiene un solo caracter, sino que tiene varios, entonces escribes tantos signos de interrogación como caracteres existan. Por ejemplo, si quieres buscar las fotos que tengan esta estructura “foto11.jpg”, escribes:_
