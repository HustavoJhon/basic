<h1 align="center"> Tipos de variables </h1>

*JavaScript es un lenguaje débilmente tipado. Esto quiere decir que no indicamos de qué tipo es cada variable que declaramos. Todas las variables admiten todos los tipos, y pueden ser reescritas. Es una de las cosas buenas y malas que tiene JavaScript.*

### ***Definición***
*Las variables son espacios de memoria donde almacenamos temporalmente datos desde los que podemos acceder en cualquier momento de la ejecución de nuestros programas. Tienen varios tipos y clases que veremos a continuación.*<br>

*Para definir una variable en JavaScript, utilizamos la palabra reservada<code> var </code> y le damos un nombre, por ejemplo:*<br>   
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
*Debemos intentar que los nombres de las variables sean lo más descriptivos posibles,de manera que con solo leerlos sepamos que contienen y así nuestros desarrollos serán más ágiles.* <br>

*Los nombres de las variables siempre han de comenzar por una letra, el símbolo <code>$ o _ </code>, nunca pueden comenzar por números u otros caracteres especiales. JavaScript también distingue entre mayúsculas o minúsculas, por tanto no es lo mismo <code> miDato </code> que <code>MiDato</code> o <code>miDato</code> , para JavaScript son nombres diferentes y las tratará de manera diferente.*

---
## Resumen

**Variables**

        * Let (alcanse local)
        * Var (alcanse global)
        * Const (constante)

**Tipos de Datos** 

        * String
        * Number
        * Boolean

**Casos especiales**

        * Undefined (no definido)
        * Null (valor nulo o vacio)
        * Nan (not a number)

**Scope**
> el escope es un alcance de las variable por ejemplo: dentro de una funcion solo va a tener el alcance dentro de la funcion y afuera no

**Hoisting**
> ejecuta funciones sin antes tener un valor