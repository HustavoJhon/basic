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

## Programas remificados

Para que nuestros programas realicen trabajos interesantes estos deben ser capaces de tomar decisiones, test o pruebas, es desde este concepto donde salen las ramificaciones. Dentro de los test que podemos realizar son los operadores de comparación y estos nos devolveras si la comparación es verdadera (True) o falsa (False).

- Igual (==): Lo utilizaremos para comparar 2 objetos.
- Distinto (!=): Verificamos que los objetos sean distintos.
- Mayor que (>): Igual que en algebra, comparamos si el primer termino es mayor que el segundo.
- Menor que (<): Verificamos que el primer termino sea menor que el segundo.
- Mayor igual que (>=): Verificamos que el primer termino sea mayor igual al segundo.
- Menor igual que (<=): Verificamos que el primer termino sea menor igual al segundo.
Ademas de los operadores de comparación también tenemos los operadores lógicos, estos son 3 (and, or, not).

Una vez que podemos entender bien los operadores de comparación y lógicos podemos generar nuestros programas ramificados. Una forma típica de ocupar los operadores es con el método if.
```py
if condition:   # Evaluamos en primera instancia una condición.
    expresion
elif:           # En caso de que no se cumpla la condición anterior evaluamos nuevamente con otra.
    expresion
else:           # En caso de que no se cumpla ninguna condición.
    expresion

# En el ejemplo anterior pueden es obligatorio el 'if', sin embargo 'elif'
# y 'else' son opcionales. Pueden existir cuantos 'elif' queramos, pero solo
# puede haber 1 'if' y 1 'else'.

if 4 > 5:
    ...
elif 4 < 5:
    print('4 es menor que 5')
else:
    ...
```
Para poner en práctica esto crearemos un archivo programas_ramificados.py y dentro de el escribiremos:
```py
num_1 = int(input('Escoge un entero: '))    # Preguntamos por un primer numero.
num_2 = int(input('Escoge otro entero: '))  # Luego preguntamos por un segundo numero.

if num_1 > num_2:       # Si el primer numero es mayor que el segundo.
    print('El primer numero es mayor que el segundo.')  # Imprimimos esta expresión.
elif num_1 < num_2:     # En caso de que el segundo sea mayor.
    print('El segundo numero es mayor que el primero.') # Imprimiremos esta expresión.
else:   # En caso de que no cumpla ninguna condición.
    print('Los 2 numeros son iguales.')
```
Para ejecutar nuestro programa iremos a la terminal y escribiremos
```py
python3 la/dirección/relativa/de/tu/archivo/programas_ramificados.py
```
y en consolo nos preguntara nuestros numeros y nos dara un resultado
```py
Escoge un entero: 8
Escoge otro entero: 4
#El primer numero es mayor que el segundo.
Escoge un entero: 7
Escoge otro entero: 10
#El segundo numero es mayor que el primero.
Escoge un entero: 4
Escoge otro entero: 4
#Los 2 numeros son iguales.
```


## Iteraciones

En esta clase vimos que los iteradores son estructuras de control que me permiten llevar un flujo siempre que una condición se cumpla. Si esta condición se cumple eternamente, y no le decimos al programa que se detenga se generará un infinit loop.

Adjunto el código de esta clase donde vemos un ciclo While anidado dentro de otro.
```py
contador_externo=0
contador_interno=0

while contador_externo <5:
    while contador_interno <6:
        print(contador_externo,contador_interno)
        contador_interno+=1
        # contador_interno += 1 --> Si eliminamos esto, habrá un bucle infinito de ceros porque el contador nunca avanza,
        # y es el contador interno el que define el externo.

        if contador_interno >=3:
            break # Esto es un break statement qu eme dice que cuando mi contador interno sea 3, pare.
            # Solo debe parar el contador interno, pues el externo tiene un contador por si mismo y seguirá la ejecución

    contador_externo+=1
    # contador_externo += 1 --> Si elimino esto, al ser el contador externo siempre 0, le estoy diciendo que vaya hasta infinito, y el código interno
    # se seguirá ejecutando de 0 a 5 indefinidamente
    contador_interno=0
    # contador_interno=0  --->Eliminar este implica que contador_interno solo tendrá la oportunidad de ejecutarse de 0 a 5.

# El código en general tiene dos mecanismos uno interior que hace ciclos cortos, y uno exterior que envuelve los ciclos.
# Son estructuras de control anidadas.
```