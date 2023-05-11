# Programas numericos

## Representacion de flotantes

## Enumeracion exhaustiva

También llamado “adivina y verifica” donde simplemente generamos todas las posibilidades. Técnicamente este no es un algoritmo eficiente, sin embargo, dependiendo del universo de posibilidades puede ser que sea el mas adecuado, ya que las computadoras actuales son muy rapidas y por lo tanto la eficiencia de nuestro programa no es relevante, por lo tanto siempre ten en mente este tipo de algoritmo como uno de los primeros en implementar.

Vamos a crear un ejemplo de enumeración exhaustiva buscando la raíz cuadrada exacta de un numero.

## Aproximacion de soluciones

Es similar a la enumarción exhaustiva, pero no necesita una respuesta exacta, por lo tanto podemos aproximar soluciones con un margen de error que llamaremos epsilon.

Como siempre en programación debemos hacer un trade-off, no podemos ser precisos y rápidos a la ves, por lo tanto cuando nuestro epsilon es muy pequeño esto significa que debemos realizar mas iteraciones para llegar a la aproximación, lo cual significa sacrificar tiempo. Y por otro lado si queremos que nuestro tiempo de ejecución sea lo mas corto posible debemos sacrificar la precisión aumentando el valor de epsilon.
```py
objetivo = int(input('Escoge un numero: '))

epsilon = 0.01      # Definimos un margen de error.
paso = epsilon**2   # Los pasos para buscar la raiz sera igual a epsilon^2
respuesta = 0       # Inicializamos una respuesta 0


while abs(respuesta**2 - objetivo) >= epsilon and respuesta <= objetivo:
    respuesta += paso

if abs(respuesta**2 - objetivo) >= epsilon:
    print(f'No se encontró la raiz cuadrada de {objetivo}')
else:
    print(f'La raiz cuadrada de {objetivo} es {respuesta}')
```
Puedes intentar ir moviendo la magnitud de epsilon para obtener una mejor precisión o mejorar el tiempo de ejecución.

## Busqueda Binaria

- Cuando la respuesta se encuentra en un conjunto ordenado, podemos utilizar busqueda binaria
- Es altamente eficiente, pues corta el espacio de busqueda en dos por cada iteracion

<img src="./IMG2.png">

