/* 
    ! FUNCIONES PREDEFINIDAS 
    ? Javascript tiene varias funciones predeterminadas de nivel superior
        * eva (string)
        * isFinite (para numeros finitos)
        * isNaN (para saber si no es un numero)
        * parseInt and parseFloat (convertir a decimal o entero)
        * Number and String (para convertir a number o string)
        * etc
        *url: developer.mozilla.org/es/docs/Web/JavaScript/Guide/Funciones/
*/

//* ejercice 1 - Funciones de Number o String

// var objRef;
// objRef = Number(objRef);
// objRef = String(objRef);

var date = new Date(430054663215),
    x;
x = String(date);
console.log(x);

// Resultado:► Thu Aug 18 1983 04:37:43 GMT-0700 (Pacific Daylight Time)