# MODULO 2

1. Install [Python](www.python.org)

## ELementos basicos de Python

- **¿Qué es un lenguaje de programación?**

    Es un lenguaje formal que, mediante una serie de instrucciones, le permite a un programador escribir un conjunto de órdenes, acciones consecutivas, datos y algoritmos para, de esa forma, crear programas que controlen el comportamiento físico y lógico de una máquina.

- **¿Qué tipos de lenguaje de programación existen?**
    
    El lenguaje de programación es la base para construir todas las aplicaciones digitales que se utilizan en el día a día y se clasifican en dos tipos principales: lenguaje de bajo nivel y de alto nivel.

1. Lenguaje de programación de bajo nivel
Son lenguajes totalmente orientados a la máquina.
Este lenguaje sirve de interfaz y crea un vínculo inseparable entre el hardware y el software.

2. Lenguaje de programación de alto nivel
Tienen como objetivo facilitar el trabajo del programador, ya que utilizan unas instrucciones más fáciles de entender.

3. Lenguaje específico de dominio
En desarrollo de software e ingeniería de dominio, un lenguaje específico de dominio, o “lenguaje específico del dominio”, (en inglés domain-specific language, DSL) es un lenguaje de programación o especificación dedicado a resolver un problema en particular, representar un problema específico y proveer una técnica para solucionar una situación particular. El concepto no es nuevo pero se ha vuelto más popular debido al aumento del uso de modelaje específico del dominio.1​

4. Lenguaje de propósito general
Se llama lenguaje de propósito general al lenguaje de programación cuyos procedimientos, instrucciones y estructuras de datos están diseñados para resolver todo tipo de problemas.

5. Lenguaje de interpretados y compilados
La principal diferencia entre un lenguaje compilado y uno interpretado es que el lenguaje compilado requiere un paso adicional antes de ser ejecutado, la compilación, que convierte el código que escribes a lenguaje de máquina. Un lenguaje interpretado, por otro lado, es convertido a lenguaje de máquina a medida que es ejecutado.

Ejemplos de lenguajes compilados incluyen C, C++, Java, Go y Rust, entre muchos otros. Ejemplos de lenguajes interpretados incluyen Ruby, Python y JavaScript, entre muchos otros. A todos estos lenguajes se les conoce como lenguajes de alto nivel.

```python
1 + 1 # 1
'Platzi' / 5 # Error
'Platzi' * 3 #PlatziPlatziPlatzi
#PrintStatement
print('Hola mundo')
'Mr' + '. Internatuta'
2 + 2
```

**¿Que es un objeto?**

Concepto, abstracción o cosa con límites bien definidos y con significado para el problema que se está manejando

Escalares vs No escalares
- Tipos

int.
float.
bool.
str.
```py
#Definiendo variables
my_int  = 1
my_float = 1.0
my_bool = True
my_none = None
my_str = 'Hola'

#Imprimiendo el tipo
type(my_int)
type(my_float)
type(my_bool)
type(my_none)
type(my_str)
```

¿Que pasa si ejecutas esto?

```py
1 + 1 
2 - 5
2.0 * 3
6 // 2
6 // 4
6 / 4
7 % 2
2 ** 2
Resultado

>>> 1 + 1
2
>>> 2 - 5
-3
>>> 2.0 * 3
6.0
>>> 6 // 2
3
>>> 6 // 4
1
>>> 6 / 4
1.5
>>> 7 % 2
1
```

## Asignación de variables

Las variables son simplemente nombres asociados a espacios en memoria.
Se lee de derecha a izquierda. Al contrario que la lectura normal.

```py
a = 2

//A es igual a dos. Es la sentencia en español.No se lee así en programación, en términos de programación se lee:

//"Estamos asignando el valor 2 a la variable a"
//"La variable a tiene el valor 2 asignado(=)en memoria"

x=4

//"La variable x apunta al objeto 4 adentro (asignado,=) en memoria"

z=(a*x)/2

//"Z es el resultado de una expresión asignado a z"
```

Como principio se buscan dar nombres exactos a las variables para que sea legible a cualquier persona que lo lea,
las variables tienen que tener nombres significativas para los humanos. Como si buscásemos acercar las definiciones para entender entre nosotros las instrucciones que le damos a la máquina.
---
- Siempre y cuando las variable son se llamen como las palabras reservadas y que respeten la forma en que cada lenguaje de programación posee sus estándares
- Todo sea por el término: “Convention over configuration”

La documentación de todo lenguaje guarda sus palabras reservadas.

Podemos reasignar las variable, lo que simplemente estamos cambiando es la dirección, hacia a donde apunta memoria. El nombre de la variable no cambia, cambia es el apuntador en memoria.

(ver imagen de reasignación, mirar si lo puedo hacer con goolge dirve.)
El garbage collector es el que permite liberar el espacio de memoria para que se puedan hacer reasignaciones.

- **Garbage collector:** En ciencias de la computación el garbage collector un mecanismo automático de gestión de memoria, trata de recoger la memoria que ocupaban lso objetos en memoria que ya no son usados en el programa.
Alivia la administración manual de memoria. En C sucedía explícitamente. Por lo cual es un porceso que sucede implícitamente. Tener en cuenta que hay otras estrategias de administración de memoria y que cada una afecta el rendimiento.
Para mayor información leer:

Garbage Collector

CONCLUSIÓN:

- Las variables hacen los programas comprensibles

- Son simplemente nombres que apuntas a memoria

- Los operadores de igualdad ( = ) reciben el nombre de “Asignación”

## Cadenas y entradas

**Cadenas**

Las cadenas son secuencias de caracteres.

```py
'123'         #Esta es una cadena
```
Los operadores que utilizamos tienen otros significados. Cuando utilizamos el operador multiplicar (*) lo que haremos es multiplicar la cadena por el numero de veces que deseamos, y con el operador suma (+) concatenaremos varias cadenas, sin embargo Python nos permite concatenar de una forma mas legible.
```py
'123' * 3               # Con el operador *
'123123123'             # Obtenemos este resultado.

'123' + '456'           # Y el operador +
'123456'                # Concatenara las cadenas.

('Hip ' * 3) + 'hurra'  # Tambien podemos combinar operadores
'Hip Hip Hip hurra'

f'{"Hip " * 3}hurra'    # En Python podemos usar la expresion f para concatenar
'Hip Hip Hip hurra'
```
A las cadenas les podemos asignar diversas funciones:

**len:** nos indica la longitud de la cadena.
**indexing:** con esto podemos acceder a cada uno de los elementos de esta cadena a través de indices.
**slicing:** podemos dividir las cadenas subcadenas.
```py
my_str = 'Hello, world!'    # Creamos una cadena

len(my_str)                 # Consultamos por su longitud
13

my_str[0]                   # Con slicing consultamos por el 1er caracter.
'H'

my_str[1]                   # Consultamos por el 2do caracter.
'e'

my_str[2]                   # Consultamos por el 3er caracter.
'l'

my_str[2:]                  # Traemos desde el 3er caracter hasta el final.
'llo, world!'

# Es importante indicar que los finales no son inclusivos.

my_str[:3]                  # Tremos desde el principio hasta el 3ro.
'Hel'

my_str[2:5]                 # Traemos desde el 3er caracter hasta el 5to.
'llo'

my_str[::2]                 # Traemos desde el principio hasta el final saltando de 2 en 2.
'Hlo ol!'
```
- Los objetos de tipo str pueden representarse con comillas dobles (") o comillas simples (’)
- El operador suma (+) tiene diferente significado según el tipo de dato. Con cadenas significa concatenación.
- El operador multiplicación (*) es el operador de repetición con cadenas.
- Las cadenas son inmutables. Esto significa que una vez que creamos una cadena en memoria esta ya no puede cambiar, podemos reasignar la variable que la referencia a otro valor, pero la cadena en memoria no cambiara.

**Entradas**

Las entradas son una forma recibir información para que las computadoras logren realizas cómputos.

- Python tiene la función input para recibir datos del usuario del programa
- Input siempre regresa cadenas, por lo que si queremos utilizar otro tipo, tenemos que hacer type casting. El type casting es transformar el tipo de dato en otro, con esto podemos transformar el tipo y guardarlo en memoria asignandolo a una variable.
```py
nombre = input('Cual es tu nombre: ')   # Utilizamos input para ingresar un nombre
Cual es tu nombre: Karl

print(nombre) # Vemos que contiene nuestra variable nombre
Karl

print(f'Tu nombre es {nombre}')   # Imprimimos una cadena concatenando una oracion con nuestra variable.
Tu nombre es Karl

numero = input('Escribe un numero: ')   # Utilizamos input para ingresar un numero
Escribe un numero: 45

numero    # Vemos que contiene nuestra variable numero
'45'

type(numero)    # Vemos el tipo de dato que es numero
<class 'str'>   # Y vemos que es un str

numero = int(input('Escribe un numero: '))  # Pero si definimos previamente el input como int
Escribe un numero: 45

type(numero)    # Nuestra variable numero sera de tipo int
<class 'int'>
```