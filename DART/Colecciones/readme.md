# Colecciones

**Que es una coleccion**

Es una estructura, que representa un grupo de valores mediante una unica variable.

Por lo general solo contiene un unico tipo de dato y cada valor puede ser locaizado por un indice.

**Beneficios de usar una coleccion**

- Esfuerzo

    Reduce los esfuerzos de programacion.

- Manipulacion

    Permite manipular las estructuras.

- Reutilizacion
    
    Fomenta la reutilizacion de software.

## Coleccion de tipo Lista

**Que es una lista**

- Definicion

    Es un tipo de dato que consiste en una secuencia ordenada de valores de un tipo especifico y de tamanio variable.

- Creacion

    `List <int> = [1, 2, 3]`

    `List <String> = []`

**Propiedades de List**

- `fist`: Devuelve el primer elemento de la lista.

- `isEmpty`: Devuelve true si la coleccion no tiene elementos.

- `isNotEmpty`: Devuelve true si la coleccion tiene al menos un elemento.

- `length`: Devuelve el tamanio de la lista.

- `last`: Devuelve el ultimo elemento de la lista.

- `reversed`: Devuelve un objeto iterable que contiene los valores de la lista en orden inverso.

**Funciones de List**

- `add`: Agrega un nuevo elemento a la lista.

- `insert`: Agrega un elemento en una posicion especifica.

- `removeAt`: Elimina un elemento de una posicion especifica.

- `clear`: Limpia la lista.

## Coleccion de tipo Sets

**Que es un Set**

- Definicion 

    Es parecido a una lista, con la diferencia de que NO puede tener valores duplicados.

- Creacion

    `List <int> = {1, 2, 3}`

    `List <String> = {}`

> Tienen las misma propiedades que una lista

**Funciones de Set**

- `.add`: Agrega un nuevo elemento al conjunto.

- `.remove`: Elimina un elemento del conjunto.

- `.clear`: Limpia el conjunto.

## Colecciones de tipo Mapa

**Que es un Map**

- Definicion 

    Es una coleccion de pares de llave - valor, tambien se les conoce como diccionarios, donde la llave no se puede repetir.

- Inicializacion

    ```dart
    var persona = {
        nombre: "Ana",
        edad: 12,
    }
    ```

**Propiedades de Map**

- `key`: Devuelve una lista con las llaves.

- `values`: Devuelve una lista con los valores.

- `isEmpty`: Devuelve true si la coleccion no tiene elementos.

- `isNotEmpty`: Devuleve true si la coleccion tiene al menos un elemento.

- `length`: Devuelve el tamanio de la lista.

**Funciones de Map**

- `addAll`: Agrega elementos a la coleccion.

- `remove`: Elimina un elemento del conjunto.

- `clear`: Limpia el conjunto.

**Valores nulos**

- Cuando aparece

    Cuando el conjunto no tiene una llave asociada, retorna null.

- Como lidiar con ellos

    `null` es un tipo de valor que representa nada, por lo que puede ser comparado mediante un `if`.

## Colecciones de Colecciones

**Colecciones Anidadas**

- Cuando sucede

    Es una coleccion contiene una coleccion dentro de sus valores, comun en Maps.

- Ejemplo

    ```dart
    Map restaurantes = {
        "nombre": "Pollos del monte",
        "estrellas": [5, 4, 3, 4,2]
    }
    ```

**Union de colecciones**

- Caso 

    Cuando se requiere unir el contenido de 2 o mas colecciones.

- Soluciones

    Usando la funcion `.addAll()`

    Durante el constructor con ...

    Hacer una nueva coleccion

## FLujos dentro de colecciones

**`if` entre colecciones**

- Que es

    Dentro de la declaracion de colecciones se puede tener un condicional de tipo `if`.

- Ejemplo

    ```dart
    var colores = [
        "verde",
        if (agregarAmarillo) "amarillo",
        "azul"
    ];
    ```

**`for` entre colecciones**

- Que es

    Dentro de la declaracion de colecciones se puede tener un condicionador de tipo `for`.

- Ejemplo

    ```dart
    var colores1 = [
        "verde",
        for (var color in colores2) color, "azul"
    ];
    ```