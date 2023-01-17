# For

> En Python, el ciclo for se usa a menudo para iterar sobre objetos iterables como listas, tuplas o cadenas.

- Recorrido es el proceso de iteración a través de una serie. Si tenemos una sección de código que nos gustaría repetir un cierto número de veces, empleamos bucles for. El bucle for generalmente se usa en un objeto iterable, como una lista o la función de rango incorporada. La declaración for en Python atraviesa los elementos de una serie, ejecutando el bloque de código cada vez. La instrucción for se opone al ciclo “while”, que se emplea siempre que una condición requiere que se verifique en cada repetición o cuando una pieza de código se debe repetir indefinidamente.
```python
por  valor  en  secuencia:  
    {cuerpo del bucle}

# other example

# Código para encontrar la suma de los cuadrados de cada elemento de la lista usando for loop  
  
# creando la lista de números  
números = [ 3 ,  5 ,  23 ,  6 ,  5 ,  1 ,  2 ,  9 ,  8 ]  
  
# inicializando una variable que almacenará la suma  
suma_ =  0  
  
# usando for loop para iterar sobre la lista  
para  num  en  números:  
     
suma_ = suma_ + núm **  2   
  
print ( "La suma de los cuadrados es: " , sum_)
```

**¿Cúando usar “while” o “for”?**
- while: cuando no nonozcas la cantidad de elementos a recorrer o la cantidad de iteraciones a realizar.
- for: cuando conozas la cantidad de elementos a recorrer o el número de iteraciones a relaizar.


## Anidados

**For anidados**
> Es posible anidar los for, es decir, meter uno dentro de otro. Esto puede ser muy útil si queremos iterar algún objeto que en cada elemento, tiene a su vez otra clase iterable.

```python
# lista de listas
lista = [
  [56,34,1],
  [12,4,5],
  [9,4,3]
]

for i in lista:
  for j in i:
    print(j)
```

→ 56, 34, 1, 12, 4, 5, 9, 4, 3

Al bucle que se encuentra dentro del otro se le puede denominar bucle interior o bucle interno. El otro bucle sería el bucle exterior o bucle externo.

Es recomendable que los nombres de las variables de control de los bucles anidados no coincidan, para evitar ambigüedades.
