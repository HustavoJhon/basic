# Constante - Const

### _Resumen_
Las variables constantes presentan un ámbito de bloque (block scope) tal y como lo hacen las variables definidas usando la instrucción let, con la particularidad de que el valor de una constante no puede cambiarse a través de la reasignación. Las constantes no se pueden redeclarar.

> ⚠️ La redeclaración de la misma variable bajo un mismo ámbito léxico terminaría en un error de tipo **SyntaxError**. Esto también es extensible si usamos ```var``` dentro del ámbito léxico. Esto nos salvaguarda de redeclarar una variable accidentalmente y que no era posible  solo con ```var```.

# Sintaxis
```js
const name1 = value1 [, varname2 = value2 [, varname3 = value3 [,...[, varnameN]]]];
```
    * varnameN
        Nombre de la constante. Puede ser un identificador legal.
    * valueN
        Valor de la constante. Puede ser cualquier expresión legal.