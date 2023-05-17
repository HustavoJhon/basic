# Uso de Strings y ciclos

## Strings en Python

Los **strings** o **cadenas de textos** tienen un comportamiento distinto a otros tipos como los booleanos, enteros, floats. Las cadenas son secuencias de caracteres, todas se pueden acceder a través de un índice.

Podemos saber la longitud de un string, cuántos caracteres se encuentran en esa secuencia. Lo podemos saber con la built-in function global llamada `len`.

Algo importante a tener en cuenta cuando hablamos de strings es que estos son inmutables, esto significa que cada vez que modificamos uno estamos generando un nuevo objeto en memoria.

El índice de la primera letra es 0, en la programación se empieza a contar desde 0

## Operaciones con String en Python

```python
word = "Hello world"
dir(word)
```

**DOC**

```python
def my_function():
    """Este es un textot de ayuda de la funcion"""
    pass

help(my_function)
```