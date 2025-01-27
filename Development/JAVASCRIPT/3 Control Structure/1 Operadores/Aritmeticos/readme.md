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
