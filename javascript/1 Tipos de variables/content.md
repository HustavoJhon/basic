# Tipos de variables
JavaScript es un lenguaje débilmente tipado. Esto quiere decir que no indicamos de qué tipo es cada variable que declaramos. Todas las variables admiten todos los tipos, y pueden ser reescritas. Es una de las cosas buenas y malas que tiene JavaScript.

### Definición
Las variables son espacios de memoria donde almacenamos temporalmente datos
desde los que podemos acceder en cualquier momento de la ejecución de nuestros
programas. Tienen varios tipos y clases que veremos a continuación.<br>
Para definir una variable en JavaScript, utilizamos la palabra reservada<code> var </code> y le damos un nombre, por ejemplo: <br>   
```javascript
    var miDato;
```
También le podemos asignar un valor en la misma línea que la declaramos, por ejemplo, a continuación a la variable<code> dato</code> le asignamos el valor<code> 5</code> :
```javascript
    var dato = 5;
```
O podemos primero declarar la variable y más adelante, en otra línea, asignarle un valor:
```javascript
    var dato;
    dato = 5;
```
Debemos intentar que los nombres de las variables sean lo más descriptivos posibles,de manera que con solo leerlos sepamos que contienen y así nuestros desarrollos serán más ágiles. <br>
Los nombres de las variables siempre han de comenzar por una letra, el símbolo <code>$ o _ </code>, nunca pueden comenzar por números u otros caracteres especiales. JavaScript también distingue entre mayúsculas o minúsculas, por tanto no es lo mismo <code> miDato </code> que <code>MiDato</code> o <code>miDato</code> , para JavaScript son nombres diferentes y las tratará de manera diferente.

---
## Tipos
JavaScript tiene 4 tipos primitivos de datos para almacenar en variables. Estos son:
* number <br>
* boolean <br>
* string <br>
* undefined <br>
### number
> Sirve para almacenar valores numéricos. Son utilizados para contar, hacer cálculos y comparaciones. Estos son algunos ejemplos:
```javascript
    var miEntero = 1;
    var miDecimal = 1.33;
```
### boolean
>Este tipo de dato almacena un bit que indica<code> true</code> o<code> false</code> . Los valores **booleanos** se utilizan para indicar estados. Por ejemplo, asignamos a una variable el estado<code> false</code> al inicio de una operación, y al finalizarla lo cambiamos a<code> true</code> . Después realizamos la comprobación necesaria.
```javascript
    var si = true;
    var no = false;
```
### string
>Las variables de tipo string almacenan caracteres o palabras. Se delimitan entre comillas simples o dobles. Ejemplo
```javascript
    var dato = "Esto es un string";
    var otroDato = 'Esto es otro string';
```
### undefined
>Este tipo se utiliza cuando el valor de una variable no ha sido definido aún o no existe.
Por ejemplo:
```javascript
    var dato; // su valor es undefined
    var dato = undefined;
```
Otro tipo de almacenamiento de datos que tiene JavaScript son los Objetos. En
JavaScript todo es un objeto, hasta las funciones. Todo *hereda* de la clase Object . Se pueden definir como una estructura donde se agregan valores. Dentro de las clases que heredan de <code>Object</code> tenemos<code> Array</code> ,<code>Date</code> , etc... que veremos más adelante.