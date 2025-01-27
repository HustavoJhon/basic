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