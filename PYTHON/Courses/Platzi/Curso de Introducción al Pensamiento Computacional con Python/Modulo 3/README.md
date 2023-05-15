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

