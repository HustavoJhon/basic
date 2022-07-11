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