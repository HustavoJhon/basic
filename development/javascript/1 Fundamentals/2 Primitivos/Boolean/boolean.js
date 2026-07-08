//! BOOLEAN 
//? true/false

// Ciclos / Condiciones
// Boolean() //result ► true / false 

let uno = Boolean(1);
console.log(uno); //true

let cero = Boolean(0);
console.log(cero); //false

//? todo string con contenido es verdadero
let cadena = Boolean("texto");
console.log(cadena); //true

//? y todo string vacio es falso
let cadena2 = Boolean("");
console.log(cadena2); //falso

/**
 *!  Truthy y Falsy
    * En JavaScript y a lo largo del curso me escucharás usar dos conceptos que de hecho son bastante divertidos de pronunciar, los valores Truthy y Falsy.

    * Decimos que un valor es Falsy cuando su representación booleana es falso, como mencioné en el tema anterior, los valores Nan, null, 0, -0, “”, y false son los considerados falsy.

    * Los valores truthy por su parte, son todos aquellos que no sean falsy, es decir que su representación booleana sea verdadero.

    * En muchos contextos del lenguaje, decir que retorna verdadero o falso no es correcto si no están retornando un booleano, por eso solemos usar las expresiones truthy para referrnos a cualquier valor verdadero, no solamente true, y falsy, para referirnos a cualquier valor falso, no solamente false.

    * Cuando el intérprete necesita saber si un valor es truthy o falsy hace un proceso llamado type coercion, del que hablaremos más adelante, que en términos simples significa que hará una conversión implícita, si lo simplificamos más significa que el lenguaje convertirá el valor a verdadero para evaluar si es truthy o falsy. Esta conversión es, digamos, momentánea, el valor original o la variable no cambian su valor, javaScript sólo obtendrá su representación booleana para saber si es truthy o falsy, sin modificar el valor original.
 */