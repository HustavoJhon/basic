# FUNCIONES, ALCANCE Y ABSTRACCION

**Habilidades importantes de los ingenieros del Software**

1. Abstracción: Significa que no necesitas entender la forma como opera algo internamente para poderlo utilizar.
2. Decomposición: Permite dividir el código en componentes que colaboran con un fin común (se puede pensar como mini programas dentro de un programa mayor).

```py
def <nombre>(<parámetros>):
    <cuerpo>
    return <expresion>

#Ejemplo:
def suma (a, b):
    total = a + b
    return total

suma(2, 3)
```
**Argumentos de tipo Keyword y valores por defecto**
```py
def nombre_completo(nombre, apellido, inverso=False):
    if inverso:
        return f" {apellido} {nombre}"
    else:
        return f" {nombre} {apellido}"

nombre_completo("Harrinson", "Quintero")
nombre_completo("Harrinson", "Quintero", inverso=True)
nombre_completo(apellido="Harrinson", nombre="Quintero")
```

## Scope o Alcance

in Python , the **LEGB rule** is usde to decide the order in whitch the namespace are to be searched for scope resolution.
The scopes are listed below in terms of hierarchy(highest to lowest/narrowest to broadest):
- Local(L): Defined inside function/class
- Enclosed(E): Defined inside enclosing functions(Nested funtion concept)
- Global(G): defined at the uppermodt level
- Built-in(B): Reserverd names in Python builtin modules

## Especificaciones del codigo

**Documentacion:**

1. La primera linea debe ser un resumen, importante que quepa en una sola linea y este separada del resto de docstrings por un espacion en blanco.

2. El resto de la cadena de documentación debe describir el comportamiento de la función.

3. Se recomienda dejar una línea en blanco antes de las triples comillas que cierran la cadena de documentación.

> Fuente: http://edupython.blogspot.com/2013/12/documentando-programas-en-python.html

## Recursividad

Algoritmica:

Una foma de usar el principio “divide and conquer”, definirla así es una forma de crear soluciones a partir de versiones más pequeñas del mismo problema.
Existen soluciones base que permiten iterarse para encontrar un solución.

Programática:

Una técnica mediante la cual una función se llama así misma. Una función adentro de otra función, existen varios ejemplos de esto.
Los factoriales, la serie de fibonnaci, los fractales.
Podemos definir la recursión como una iteración, cualquier función recursiva podemos representarla como un loop.

1. Siempre escribimos nuestro caso base.
nota: en python se llega a un límite de recursividad. ¿Cual es el límite?
El límite de recursividad existe con el fin de que no exista un “Stack overflow”, un flujo excesivo de recursiones causa este famosso problema.
Sin embargo aunque puede ser peligroso el StackFrames de python puede ser grande.
> Python no es un lenguaje funcional y las recursiones de colas no son particularmente una técnica eficiente, reescribir el algotimo iterativamente, de ser posible es generalmente una mejor idea

La idea anterior quedará a debate.
Lo que está ocurriendo con la recursión es:
Estamos poniendo stack encima cada ve que ejecutamos un factorial.
Al tener caso base se regresan todos lo valores en una sola lína.

El factorial lo que hace es que me ejecuta la función n, n veces.

A continuación dejo el estudio comentariado y referenciado de recursividad y su límite.
```py
def me():
    """
■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■ INICIO
        Este código teiene como propósito Encontrar el límite de recursión teórico de mi máquina y compararlo con el real.
        1. Defino una funcion recursiva
        2. Le paso dos  argumentos n y resultado_suma  haciendo referencia a un número n y mi suma.
        3. Imprimo el resultado de suma iterado después de pasarlo por un ciclo for

■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■  FINAL
            El resultado que me va a devolver al final es:
=>
    [Previous line repeated 993 more times]==> Cuando mi límite de recursión teórico es de 1000
        ...
    RecursionError: maximum recursion depth exceeded while calling a Python object
■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■ RECURSION LIMIT
    Para cambiar el límite de recursión en python se usa la función: sys.setrecursionlimit(limit)
        Esta función ajusta mi limite máximo de profundidad del stack del interprete de pyhton, este límite previene que la recursión "
        infinita" crashee el "C stack" de Pyhton
■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■ CONSIDERACIONES
    El límite es directamente proporcional a la máquina. Es plataforma dependiente y es mejor usarlo cuando el contexto lo amerite.
        Si el nuevo límite de profundidad en la recursión es muy bajo el error RecursionError aparece.

https://docs.python.org/3/library/sys.html#sys.setrecursionlimit
■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■ ¿CUAL ES MI LÍMITE DE RECURSIÓN?
=>      Para saber el límite de recursión se usa: sys.getrecursionlimit()
        Me devuelve el valor actual del limite de recursión para el stack de python.

    """
    pass

#Con este import traigo palabra reservada sys. del sistema
import sys

print(help(me))
print(f'Mi límite de recursión teórico es: {sys.getrecursionlimit()}')

if __name__ == '__main__':

    def recursive_function  (n , resultado_sum):
        if n < 1:
            print(f'Vuelve siempre cero para que se ejecute un true infinito {n}. Soy el resultado de una suma y tengo el valor de: {resultado_sum} ')
            return sum
        else:
            # Soy la suma inicialmente de la forma =>  n = n+1
            # Y mi resultado lo voy a iterar, repetir como una suma =>  (n + 1) + resultado_suma
            recursive_function(n - 1, resultado_sum + n)

#El valor de c me dará indicación de que empieza
    c = 0
# Le digo que me itere hasta 998.
for i in range(998):
    c += 1
    valor_iterado = recursive_function(c, 0)
```