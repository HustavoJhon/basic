//! CICLOS / BUCLES

/**
 * 1. Instruccion inicial
 * 2. Condicion
 * 3. Instruccion despues de cada interacion
*/

console.log("*************FOR***************");
//! FOR
//? for ([expresion-inicial]; [condicion]; [expresion-final])sentencia
// Imprimir numeros del 1 al 10=
// for (let i = 1; i < 11; i++ ){ //or i = i + 1
//     console.log(i);
// }

console.log("*************WHILE***************");
//! WHILE
// let number = 1;
// while (number <= 10) {
//     console.log(number);
//     number++;
// }

// while (prompt()) {
//     console.log("ejecucion");
// }

console.log("*************DO WHILE***************");
//! DO WHILE
// do {
//     console.log("menu")
// } while (number == 3);

console.log("************BREAK****************");
//! Break
//? para cualquier programa
// for (let i = 1; i < 11; i++ ){ 
//     console.log(i);
//     if (i == 5) {break;}
// }

console.log("*************CONTINUE***************");
//! Continue
// salta de la iteracion en la que vamos
// for (let i = 1; i < 11; i++ ){ 
//     console.log(i);
//     if (i % 2 != 0) {continue;}

//     console.log("Es par")
// }



//* ejercice 1 - FOR 
// imprimir del 10 al 1
// for (let i = 10; i >= 1; i--) {
//     console.log(i);   
// }
//      del 0 al 50 de 5 en 5
// for (let n = 0; n <= 50; n+=5){
//     console.log(n);
// }

//* ejercice 2 - WHILE
// var i = 10;
// while(i >= 1){
//     console.log(i);
//     i--;
// }

//* ejercice 3 - DO WHILE
var i = 1;
do{
    console.log(i);
    i++;
} while(i <= 10);