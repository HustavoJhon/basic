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
