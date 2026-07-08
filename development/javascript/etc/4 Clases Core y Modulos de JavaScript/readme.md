<h1 align="center">Clases Core y Módulos de JavaScript</h1>

> Además de los tipos primitivos que vimos al principio de este libro, JavaScript tiene unas clases, llamadas **Core** que forman parte del lenguaje. Las que más se utilizan son **Object , Number , Array** y **String** . Todas ellan heradan de **Object** .

## **Object**
Un objeto es una colección de variables y funciones agrupadas de manera estructural. A las variables definidas dentro de un objeto se las denomina propiedades, y las funciones, métodos. Veamos un ejemplo de objeto que recoje los datos de un libro:
```js
    var libroAngular = {
    titulo: 'Desarrollo web ágil con AngularJS',
    autor: 'Carlos Azaustre',
    paginas: 64,
    formatos: ["PDF", "ePub", "Mobi"],
    precio: 2.79,
    publicado: false
    };
```
Como podemos ver, las propiedades son pares clave-valor, separados por comas, y
podemos acceder a ellas de forma independiente de varias formas, con la notación
punto o con la notación array:
```js
    libroAngular.titulo; // Desarrollo web ágil con AngularJS
    libroAngular['paginas']; // 64
```
También podemos modificarlas de la misma manera:
```js
    libroAngular.precio = 1.95;
    libroAngular['publicado'] = true;
```
Con la notación array, podemos acceder a las propiedades con variables. Ejemplo:
```js
    var propiedad = "autor";
    libroAngular[propiedad]; // "Carlos Azaustre"
```
Pero no funciona con la notación punto:
```js
    var propiedad = "autor";
    libroAngular.propiedad; // undefined
```
Como hemos dicho antes, si el objeto contiene funciones se les llama métodos. En el siguiente capítulo veremos como se inicializan e invocan funciones más en detalle. Si queremos crearlas dentro de un objeto e invocarlas, sería así:
```js
    var libroAngular = {
        paginas: 64,
        leer: function () {
            console.log("He leído el libro de AngularJS");
        }
    };
    libroAngular.leer(); // Devuelve: "He leído el libro de AngularJS"
```
Para crear un objeto podemos hacerlo con la notación de llaves {...} o creando una nueva instancia de clase:
```js
var miObjeto = { propiedad: "valor" };
var miObjeto = new Object({ propiedad: "valor" });
```

## ***Anidación***
> Un objeto puede tener propiedades y estas propiedades tener en su interior más
propiedades. Sería una representación en forma de árbol y podemos acceder a sus
propiedades de la siguiente manera:

```js
    var libro = {
        titulo: "Desarrollo Web ágil con AngularJS",
        autor: {
            nombre: "Carlos Azaustre",
            nacionalidad: "Española",
            edad: 30,
            contacto: {
                email: "carlosazaustre@gmail.com",
                twitter: "@carlosazaustre"
            }
        },
        editorial: {
            nombre: "carlosazaustre.es Books",
            web: "https://carlosazaustre.es"
        }
    };
    // Podemos acceder con notación punto, array, o mixto.
    libro.autor.nombre; // "Carlos Azaustre"
    libro['autor']['edad']; // 30
    libro['editorial'].web; // "https://carlosazaustre.es"
    libro.autor['contacto'].twitter; // "@carlosazaustre
```

### ***Igualdad entre objetos***
Para que dos objetos sean iguales al compararlos, deben tener la misma referencia. Debemos para ello utilizar el operador identidad <code>===</code> . Si creamos dos objetos con el mismo contenido, no serán iguales a menos que compartan la referencia. Veamos un ejemplo:
```js
    var coche1 = { marca: "Ford", modelo: "Focus" };
    var coche2 = { marca: "Ford", modelo: "Focus"};
    coche1 === coche2; // Devuelve false, no comparten referencia
    coche1.modelo === coche2.modelo; // Devuelve true porque el valor es el mismo.
    var coche3 = coche1;
    coche1 === coche3; // Devuelve true, comparten referencia
```
---
## **Number**
>Es la clase del tipo primitivo number . Se codifican en formato de coma flotante con doble precisión (Es decir, con 64 bits / 8 bytes) y podemos representar números enteros, decimales, hexadecimales, y en coma flotante. Veamos unos ejemplos:
```js
    // Número entero, 25
    25
    // Número entero, 25.5. Los decimales se separan de la parte entera con punto `.`
    25.5
    // Número hexadecimal, se representa con 0x seguido del número hexadecimal
    0x1F // 31 decimal
    0xFF // 255 decimal
    0x7DE // 2014
    // Coma flotante, separamos la mantisa del exponente con la letra `e`
    5.4e2 // Representa 5.4 * 10 elevado a 2 = 540
```
La clase Number incluye los números **Infinity** y **-Infinity** para representar números muy grandes:
```js
    1/0 = Infinty
    -1/0 = -Infinity
    1e1000 = Infinity
    -1e1000 = -Infinity
```
El rango real de números sobre el que podemos operar es ~1,797x10^308 ---
5x10^-324`. <br>

También disponemos del valor NaN (Not A Number) para indicar que un determinado valor no representa un número:
```js
    "a"/15 = NaN
```
Para crear un número podemos hacerlo con la forma primitiva o con la clase Number . Por simplicidad se utiliza la forma primitiva.
```js
    var numero = 6;
    var numero = new Number(6);
```
---
### ***Funciones de Number***
>JavaScript tiene 2 funciones interesantes para convertir un string en su número
equivalente.
***parseInt()***

Devuelve el número decimal equivalente al string que se pasa por parámetro. Si se le indica la base, lo transforma en el valor correspondiente en esa base, si no, lo devuelve en base 10 por defecto. Veamos unos ejemplos:
```js
    parseInt("1111"); // Devuelve 1111
    parseInt("1111", 2); // Devuelve 15
    parseInt("1111", 16); // Devuelve 4369
```
***parseFloat()***

Función similar a parseInt() que analiza si es un número de coma flotante y devuelve su representación decimal
```js
    parseFloat("5e3"); // Devuelve 5000
```
***number.toFixed(x)***

Devuelve un string con el valor del numero number redondeado al alza, con tantos
decimales como se indique en el parámetro x .
```js
    var n = 2.5674;
    n.toFixed(0); // Devuelve "3"
    n.toFixed(2); // Devuelve "2.57"
```
***number.toExponential(x)***

Devuelve un string redondeando la base o mantisa a x decimales. Es la función
complementaria a parseFloat
```js
var n = 2.5674;
n.toExponential(2); // Devuelve "2.56e+0"
```
***number.toString(base)***

Devuelve un string con el número equivalente number en la base que se pasa por parámetro. Es la función complementaria a parseInt
```js
(1111).toString(10); // Devuelve "1111"
(15).toString(2); // Devuelve "1111"
(4369).toString(16) // Devuelve "1111"
```
---
## ***Módulo Math***
> Math es una clase propia de JavaScript que contiene varios valores y funciones que nos permiten realizar operaciones matemáticas. Estos son los más utilizados:
```js
    Math.PI // Número Pi = 3.14159265...
    Math.E // Número e = 2.7182818...
    Math.random() // Número aleatorio entre 0 y 1, ej: 0.45673858
    Math.pow(2,6) // Potencia de 2 elevado a 6 = 64;
    Math.sqrt(4) // raiz cuadrada de 4 = 2
    Math.min(4,3,1) // Devuelve el mínimo del conjunto de números = 1
    Math.max(4,3,1) // Devuelve el máximo del conjunto de números = 4
    Math.floor(6.4) // Devuelve la parte entera más próxima por debajo, en este caso 6
    Math.ceil(6.4) // Devuelve la parte entera más próxima por encima, en este caso 7
    Math.round(6.4) // Redondea a la parte entera más próxima, en este caso 6
    Math.abs(x); // Devuelve el valor absoluto de un número
    // Funciones trigonométricas
    Math.sin(x); // Función seno de un valor
    Math.cos(x); // Función coseno de un valor
    Math.tan(x); // Función tangente de un valor
    Math.log(x); // Función logaritmo
    ...
```
Existen muchos más, puedes consultarlo en la documentación de Mozilla: link
## ***Array***
>Es una colección de datos que pueden ser números, strings, objetos, otros arrays, etc... Se puede crear de dos formas con el literal [...] o creando una nueva instancia de la clase Array
```js
    var miArray = [];
    var miArray = new Array();

    var miArray = [1, 2, 3, 4]; // Array de números
    var miArray = ["Hola", "que", "tal"]; // Array de Strings
    var miArray = [ {propiedad: "valor1" }, { propiedad: "valor2" }]; // Array de objetos
    var miArray = [[2, 4], [3, 6]]; // Array de arrays, (Matriz);
    var miArray = [1, true, [3,2], "Hola", {clave: "valor"}]; // Array mixto
```

Se puede acceder a los elementos del array a través de su índice y con length
conocer su longitud.
```js
    var miArray = ["uno", "dos", "tres"];
    miArray[1]; // Devuelve: "dos"
    miArray.length; // Devuelve 3
```
Si accedemos a una posición que no existe en el array, nos devuelve undefined .
```js
    miArray[8]; // undefined
```
## ***Métodos***

> Array es una clase de JavaScript, por tanto los objetos creados a partir de esta clase heredan todos los métodos de la clase padre. Los más utilizados son:
```js
var miArray = [3, 6, 1, 4];
miArray.sort(); // Devuelve un nuevo array con los valores ordenados: [1, 3, 4, 6]
miArray.pop(); // Devuelve el último elemento del array y lo saca. Devuelve 6 y miArray queda [1, 3, 4]
miArray.push(2); // Inserta un nuevo elemento en el array, devuelve la nueva longitud del array y miArray queda ahora [1, 3, 4, 2]
miArray.reverse(); // Invierte el array, [2,4,3,1]
```
Otro método muy útil es join() sirve para crear un string con los elementos del array uniéndolos con el "separador" que le pasemos como parámetro a la función. Es muy usado para imprimir strings, sobre todo a la hora de crear templates. Ejemplo:
```js
    var valor = 3;
    var template = [
    "<li>",
    valor
    "</li>"
    ].join("");
    console.log(template); // Devuelve: "<li>3</li>"
```
Lo cual es mucho más eficiente en términos de procesamiento, que realizar lo siguiente, sobre todo si estas uniones se realizan dentro de bucles.
```js
    var valor = 3;
    var template = "<li>" + valor + "</li>";
```

Si queremos aplicar una misma función a todos los elementos de un array podemos
utilizar el método map . Imaginemos el siguiente array de números [2, 4, 6, 8] y
queremos conocer la raíz cuadrada de cada uno de los elementos podríamos hacerlo
así:
```js
    var miArray = [2, 4, 6 ,8];
    var raices = miArray.map(Math.sqrt);
    });
    // raices: [ 1.4142135623730951, 2, 2.449489742783178, 2.8284271247461903 ]
```
O algo más específico:
```js
    var miArray = [2, 4, 6, 8];
    var resultados = miArray.map(function(elemento) {
    return elemento *= 2;
    });
    // resultados: [ 4, 8, 12, 16 ]
```
Otra función interesante de los arrays es la función filter . Nos permite "filtrar" los elementos de un array dada una condición sin necesidad de crear un bucle (que veremos más adelante) para iterarlo. Por ejemplo, dado un array con los números del 1 al 15, obtener un array con los números que son divisibles por 3:
```js
    var miArray = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    var resultado = miArray.filter(function(elemento) {
    return elemento % 3 === 0;
    });
    // resultados: [ 3, 6, 9, 12, 15 ]
```
Si queremos obtener una parte del array, podemos emplear la función slice
pasándole por parámetro el índice a partir del que queremos cortar y el final. Si no se indica el parámetro de fin, se hará el "corte" hasta el final del array, si no, se hará hasta la posición indicada y si se pasa un número negativo, contará desde el final del array hacia atrás. 

El método devuelve un nuevo array sin transformar sobre el que se está invocando la
función. Veamos unos ejemplos:
```js
    var miArray = [4, 8, 15, 16, 23, 42];
    miArray.slice(2); // [15, 16, 23, 42]
    miArray.slice(2, 4); // [15, 16] (la posición de fin no es inclusiva)
    miArray.slice(2, -1); // [15, 16, 23]
    miArray.slice(2, -2); // [15, 16]
```
## ***String***
Como vimos al principio de este libro, los strings son un tipo de variable primitivo en JavaScript, pero también, al igual que con ***Number*** tienen su clase propia y métodos. 

Un string se comporta como un *Array*, no es más que un conjunto de caracteres, con índices que van desde el 0 para el primer carácter hasta el último. Veamos algunos ejemplos de cómo acceder a los caracteres y los métodos que posee esta clase
```js
    // Supongamos el string con el texto "javascript"
    "javascript"[2] // Acceso como array, devuelve el tercer carácter "v", ya que la primera posición es 0
    "javascript".length() // Devuelve 10
    "javascript".charCodeAt(2) // Devuelve el caracter en formato UNICODE de "v", el 118
    "javascript".indexOf("script"); // Devuelve el índice donde comienza el string "script", el 4
    "javascript".substring(4,10); // Devuelve la parte del string comprendida entre los indices 4 y 10-1, "script"
```
Para crear un string podemos hacerlo con notación de tipo o creando un nuevo objeto.

Por simplicidad se utiliza la forma primitiva.
```js
    var texto = "Hola Mundo";
    var texto = new String("Hola Mundo");
```
Un string puede ser transformado en array con el método **split()** pasándole como parámetro el delimitador que queramos que separe los elementos. Por ejemplo:
```js
    var fecha = new Date();
    fecha = fecha.toString(); // "Wed May 20 2015 20:16:25 GMT+0200 (CEST)"
    fecha = fecha.split(" "); // ["Wed", "May", "20", "2015", "20:16:25", "GMT+0200", "(CEST)"]
    fecha[4]; // "20:16:25"
```
