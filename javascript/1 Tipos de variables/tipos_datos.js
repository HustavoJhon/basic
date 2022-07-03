//! TIPOS DE DATOS 
// con typeof podemos saber de que tipo es el dato

// number - para  todos los tipos de numeros enteros y decimales
console.log(typeof 40);
console.log(typeof 3.2);

// string - cadena de texto
console.log(typeof "hola");
console.log(typeof 'hustavo');

// Boolean - false, true
console.log(typeof true);
console.log(typeof false);


//! TIPOS DE DATOS - COLECCION DE DATOS
/** 
* OBJETOS:
*     Coleccion de datos (informacion)
*     que tiene un identificador y un valor
*/

//? Array (Arreglo)
// ["Jose", "Josue", "Juan", "Jean", "Oto"]

console.log(typeof ["Jose", "Josue", "Juan", "Jean", "Oto"]);
console.log(["Jose", "Josue", "Juan", "Jean", "Oto"]);

//? OBJETOS
// {
//     nombre: "hustavo", 
//     apellido: "Velasquez"
// }
console.log({
    nombre: "hustavo", 
    apellido: "Velasquez"
});


//? Nulo null
// valores que no existen o nulo
console.log(typeof null);

//? Undifine
console.log(typeof undefined)

// ***********************************************************
// ***********************************************************
//! VARIABLES
/**
 * Let: Declarar variables limitando su alcance al bloque, declaración, o expresión donde  se está usando.
 * Var: Define una variable global o local en una función sin importar el ámbito del bloque.
 *? la diferencia es el alcance de las variables
 */

// Declaracion
var alumno;
let colega;

// Asignacion
alumno = "Hustavo Jhon";

// Reasignaicon
let profesor = "Alexys Lozada";
profesor = "Alvaro Felipe";
console.log(profesor);