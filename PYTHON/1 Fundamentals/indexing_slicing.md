# Indexing y Slicing 

**¿Qué es la indexación?**

> Indexar es crear una referencia a un elemento de un iterable (tuplas, cadenas) por su posición particular dentro del iterable. Para comprobar cómo acceder a los elementos individuales de una lista, primero crearemos una lista. Veremos cómo funcionan las secuencias de objetos dentro de la lista en Python. La lista de objetos se escribe entre corchetes, como se muestra a continuación:

```python
#posisción de cada carácter de un texto en python 
text = 'Nosotros estamos aprendiendo Python '
print(text[0])#aca estamos accediendo al primer carácter 
print(text[1])#segindo carácter del texto
#debemo saber cuántos caracteres hay en nuestro texto, y como saber cuál es el ultimo carácter, de la siguiente manera.
size = len(text)
print('size => ', size)
print(text[size - 1])
#pero hay una forma mucho más fácil de acceder al último carácter del texto
print(text[-1])#con esta función Python nos contara la última posición del texto
print(text[-2])#seria el penúltimo carácter

# SLINCING
#con slicing podemos sacar ciertas partes del texto 
print(text[0:6])#aca estamos sacando desde el primer carácter hasta el carácter 4
print(text[10:16])# estamos sacando solo la palabra Python 
print(text[:15])#podemo obviar el primer parametro y decirle hasta que carácter quiero avanzar 
print(text[8:])#podemos indicar ir de cierto carácter hasta el final con esta función 
print(text[:])#nos imprime toto el texto
print(text[12:14:18])#tambiem puedo darle un inicio u un fin a nuestro texto(saltos de línea)
print(text[12:14:2])#stos saltos de línea tambien son validos 
print(text[::4])#al igual que este
```

## Slicing
