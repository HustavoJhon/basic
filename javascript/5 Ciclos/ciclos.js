//! CICLOS / BUCLES

/**
 * 1. Instruccion inicial
 * 2. Condicion
 * 3. Instruccion despues de cada interacion
*/

console.log("*************FOR***************");
//! FOR
// Imprimir numeros del 1 al 10=
for (let i = 1; i < 11; i++ ){ //or i = i + 1
    console.log(i);
}

console.log("*************WHILE***************");
//! WHILE
let number = 1;
while (number <= 10) {
    console.log(number);
    number++;
}

// while (prompt()) {
//     console.log("ejecucion");
// }

console.log("*************DO WHILE***************");
//! DO WHILE
do {
    console.log("menu")
} while (number == 3);

console.log("************BREAK****************");
//! Break
// para cualquier programa
for (let i = 1; i < 11; i++ ){ 
    console.log(i);
    if (i == 5) {break;}
}

console.log("*************CONTINUE***************");
//! Continue
// salta de la iteracion en la que vamos
for (let i = 1; i < 11; i++ ){ 
    console.log(i);
    if (i % 2 != 0) {continue;}

    console.log("Es par")
}