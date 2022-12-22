//! Argumentos de funciones
// los parametros no tienen un tipo definido
function cuadrado(numero) {
    return numero * numero;
}
// al llamar la funcion se puede enviar cuantas cantidades decees como parametro
cuadrado(2);

// Example 2
function sumaTodos() {
    let suma = 0;
    for (let i = 0; i < arguments.length; i++) {
        suma += arguments[i]
    }
    console.log(suma);
}
sumaTodos(1,2,3,4);

// **********************************************************************

//! PASE POR VALOR Y POR REFERENCIA
//* example 1
let edad = 20;
function modificador(edad) { //en el parametro se recive una copia de la variable edad que tiene 20
    edad = 25;
    console.log("dentro de la funcion: " + edad);
}
console.log(edad); //20
modificador(edad); //25
console.log(edad); //20

//* example 2
let edades = [30];

function modificador1(edad) { //en el parametro se recive una copia de la variable edad que tiene 20
    edad[0] = 35;
    console.log("dentro de la funcion: " + edad);
}

console.log(edades); //30
modificador1(edades); //25
console.log(edades); //25

// **********************************************************************
//! FUNCIONES PURAS
//todo: No produce efectos secundarios
let num = [50];

function modificador2(num) { //en el parametro se recive una copia de la variable edad que tiene 20
    let copia = [...num]; //genero una copia
    copia[0] = 25; // Modifico la copia y no el valor original
    return copia; // para comunicar los cambios con el exterior es 
}

// console.log(num); //50
modificador2(num); //25
// console.log(num); //50

// **********************************************************************
//! FIRST CLASS OBJECTS

/*
Debe ser posible retornarlo (return 2;)
Debe ser posible asignarlo a una variable (let number = 2;)
Debe ser posible enviarlo como argumento (hola(2);)
*/

function executar(funcion) {
    funcion();
}

function decirHola() {
    console.log("Hola");
}

executar(decirHola); //"Hola