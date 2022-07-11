## ___Tipos de Datos___
> JavaScript tiene 4 tipos primitivos de datos para almacenar en variables. Estos son:
* number <br>
* boolean <br>
* string <br>
* undefined <br>
### __number__
> *Sirve para almacenar valores numéricos. Son utilizados para contar, hacer cálculos y comparaciones. Estos son algunos ejemplos:*
```javascript
    var miEntero = 1;
    var miDecimal = 1.33;
```
### __boolean__
> *Este tipo de dato almacena un bit que indica<code> true</code> o<code> false</code> . Los valores **booleanos** se utilizan para indicar estados. Por ejemplo, asignamos a una variable el estado<code> false</code> al inicio de una operación, y al finalizarla lo cambiamos a<code> true</code> . Después realizamos la comprobación necesaria.*
```javascript
    var si = true;
    var no = false;
```
### __string__
> *Las variables de tipo string almacenan caracteres o palabras. Se delimitan entre comillas simples o dobles. Ejemplo*
```javascript
    var dato = "Esto es un string";
    var otroDato = 'Esto es otro string';
```
### __undefined__
> *Este tipo se utiliza cuando el valor de una variable no ha sido definido aún o no existe.*
Por ejemplo:
```javascript
    var dato; // su valor es undefined
    var dato = undefined;
```
_Otro tipo de almacenamiento de datos que tiene JavaScript son los Objetos. En JavaScript todo es un objeto, hasta las funciones. Todo *hereda* de la clase Object . Se pueden definir como una estructura donde se agregan valores. Dentro de las clases que heredan de <code>Object</code> tenemos<code> Array</code> ,<code>Date</code> , etc... que veremos más adelante._
