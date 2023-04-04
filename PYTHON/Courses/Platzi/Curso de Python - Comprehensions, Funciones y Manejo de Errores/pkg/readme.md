# PAQUETES

Los paquetes son las carpetas que contienen varios módulos y cada uno con una función distinta.

Para ello debe tener siempre un archivo de nombre `__init__.py` (por lo general esta vacio, ya que así es compatible con programas python versiones anteriores a la 3), con esto le estamos indicado a python que esto se trata de un paquete y no de una carpeta.

> Para acceder a los módulos de los paquetes podemos utilizar estas opciones:
    import nombrecarpeta.nombremodulo
    from nombrecarpeta import nombremodulo
    from nombrecarpeta.nombremodulo import def

Estos son algunos ejemplos:

Antes de realizar el ejemplo debemos tener nuestra carpeta definida con la siguiente estructura:

En nuestra carpeta pkg, creamos dos archivos: `__init__.py` y `aritmetica.py`
`__init__.py` Se utiliza cuando debemos utilizar paquetes en versiones anteriores a python 3 o puede contener información la cual se ejecutara siempre, para este caso le colocamos el titulo Bienvenido a la clase de Aritmetica.

`aritmetica.py` va a contener nuestras operaciones aritmeticas que son, suma, resta, multiplicacion y division:

```python

def sumar(a, b):
    return a + b

def restar(a, b):
    return a - b

def mult(a, b):
    return a * b

def div(a, b):
    return a / b

```

Ahora en nuestro archivo principal `main.py` ejecutaremos los paquetes de la siguiente manera:

`import nombrecarpeta.nombremodulo`

```python
import pkg.aritmetica
print(pkg.aritmetica.sumar(5,7))
print(pkg.aritmetica.restar(5,7))
```

Produccion:

```python
Bienvenido a la clase de Aritmética
12
-2
from nombrecarpeta import nombremodulo

from pkg import aritmetica 
print(aritmetica.sumar(5,7))
print(aritmetica.restar(5,7))
```

Produccion:

```python
Bienvenido a la clase de Aritmética
12
-2
```

`from nombrecarpeta.nombremodulo import def`

```python
from pkg.aritmetica import sumar
print(sumar(5,7))
from pkg.aritmetica import restar
print(restar(5,7))
```

Produccion:

```python
Bienvenido a la clase de Aritmética
12
-2
```