<h1 align="center"> Operadores </h1>

## **Operadores aritméticos**
> JavaScript posee operadores para tipos y objetos. Estos operadores permiten formar expresiones. Las más comunes son las operaciones aritméticas.
* **Suma de números:** *5 + 2*
* **Resta:** *5 - 2*
* **Operaciones con paréntesis:** *(3 + 2) - 5*
* **Divisiones:** *3 / 3*
* **Multiplicaciones:** *6 * 3* 

*El operador suma ***+*** también puede usarse para concatenar strings de la siguiente
manera:<code> "Hola " + "mundo" + "!"</code> tendrá como resultado<code> "Hola mundo!"</code> .*

*JavaScript también posee los operadores post y pre incremento y decremento que añaden uno o restan uno a la variable numérica en la que se aplican. Dependiendo si son pre o post, la variable es autoincrementada o decrementada antes o después de la sentencia. Veamos un ejemplo:*
```javascript
    var x = 1; // x=1
    var y = ++x; // x=2, y=2
    var z = y++ + x;// x=2, y=3, z=4
```
*Como vemos en el ejemplo, la sentencia<code> y = ++x</code> lo que hace es incrementar x, que valía 1 y pasa a tener el valor 2, y la asignación<code> y = ++x</code> hace que y valga lo que<code> x</code> ,es decir 2.* <br>

*En la última sentencia tenemos un postincremento, esto lo que hace es primero evaluar la expresión y después realizar el incremento. Es decir en el momento de la asignación<code> z = y++ + x , y</code> vale 2 y x también 2, por lo tanto<code> z</code> vale 4, y después de esta asignación<code> y</code> es incrementada pasando a tener el valor 3.*

### **Operador** typeof
> El operador typeof es un operador especial que nos permite conocer el tipo que tiene la variable sobre la que operamos. Ejemplos:
```javascript
    typeof 5; // number
    typeof false; // boolean
    typeof "Carlos"; // string
    typeof undefined; // undefined
```
Esto es muy util para conocer en un momento dado que tipo estamos utilizando y prevenir errores en el desarrollo.

### ***Operadores booleanos***
> Los tipos booleanos sólo tienen dos valores posibles:<code> true</code> y<code> false</code> (Verdadero y
Falso). Pero disponen de varios operadores que nos permiten transformar su valor.
#### ___Negación___
Este operador convierte un valor booleando en su opuesto. Se representa con el signo<code> !</code> . Si se utiliza dos veces, nos devuelve el valor original.
```javascript
    !true = false
    !false = true
    !!true = true
```
### ___Identidad o Igualdad___
*El operador de igualdad (o igualdad débil), se representa con<code> ==</code> y el de identidad (o igualdad estricta), con<code> ===</code> . Se recomienda el uso del operador de identidad (los 3 iguales) frente al de igualdad débil, ya que el coste de procesamiento de éste último es mucho mayor y sus resultados en ocasiones son impredecibles. Es una de las partes malas de JavaScript, pero si se tiene cuidado no tiene por qué darnos ningún problema.*<br>
La desigualdad estricta se representa con <code> !== </code>.
```javascript
    true === true // true
    true === false // false
    true !== false // true
    true !== true // false
```

### ___Comparación___
*Podemos comparar si dos valores son menores, mayores o iguales con los operadores de comparación representados por los símbolos<code> < , > , <= y >=</code> . El resultado de la comparación nos devuelve true o false dependiendo de si es correcto o no.*
```javascript
    5 > 3 // true
    5 < 3 // false
    3 >= 3 // true
    2 <= 1 // false
    "a" < "b" // true
```
*Aunque se pueden utilizar comparaciones entre booleanos, strings y objetos se recomienda no usarlos ya que el orden que siguen no es muy intuitivo.*

## **Operadores lógicos**
### ***Operador*** **AND**
Es un operador lógico que devuelve<code> true </code>siempre que todos los valores comparados sean<code> true</code> . Si uno de ellos es<code> false</code> , devuelve <code>false</code> . Se representa con el símbolo<code> && </code>. Veamos un ejemplo
```javascript
    true && true // true
    true && false // false
    false && true // false
    false && false // false
```
Es muy utilizado para devolver valores sin que estos sean modificados, por ejemplo para comprobar si una propiedad existe, etc. La lógica que sigue es: Si el primer valor es <code>false</code> devuelve ese valor, si no, devuelve el segundo:
```javascript
    0 && true
    // 0, porque el número 0 se considera
    // un valor "false"
    1 && "Hola"
    // "Hola", porque el número 1 (o distinto de 0)
    // se considera un valor "true"
```

*En el ejemplo comparamos 0 y<code> true</code> , como 0 es un valor que retorna<code> false</code> , nos devuelve ese valor. En el segundo ejemplo 1 es un valor que retorna true , por lo que nos devolverá el segundo<code> "Hola"</code> .*

*Valores que devuelven false . Hay ciertos valores en JavaScript que son evaluados como false y son: el número 0 , un string vacío "" , el valor null , el valor undefined y NaN .*
### ***Operador*** **OR**

*Es otro operador lógico que funciona a la inversa que AND. Devuelve<code> false</code> si los valores comparados son<code> false</code> . En el caso de que un valor sea true devolverá<code> true</code> . Se representa con el símbolo<code> ||</code> .*
```javascript
    true || true // true
    true || false // true
    false || true // true
    false || false // false
```
*También es muy utilizado para asignar valores por defecto en nuestras funciones. La lógica que sigue es: Si el primer valor es true, devuelve ese valor. Por ejemplo:*
```javascript
    var port = process.env.PORT || 5000;
```
*En este ejemplo, la variable<code> port</code> contendrá el valor de<code> process.env.PORT</code> siempre que esa variable esté definida, si no su valor será 5000.*
