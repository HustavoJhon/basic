// Ejercicios

//! WHILE
//let contador = 0;

// while (contador < 10) {
//     console.log(contador);
//     contador++;
// }
//! DO WHILE
// do {
//     console.log(contador)
//     contador++;
// } while (contador < 10);

//! FOR

// for (let i = 0; i < 10; i++) {
//     console.log("for " + i);
// }

let numeros = [10,20,30,40,50,60,70,80,90];
// for (let i = 0; i < numeros.length; i++) {
//     console.log(numeros[i]);
// }

//! FOR IN 
// este buble solo sirbe para objetos primitivos de js

// const hustavo = {
//     nombre: "jhon",
//     apellido: "Ccarita",
//     edad: 18
// }

// for(const propiedad in hustavo) {
//     console.log(`Key: ${propiedad} , Value: ${hustavo[propiedad]}`);
// }

//! FOR of
// este buble es mas para arreglos 0 tipos de datos primitivos
//este que sea iterable(que pueda mostrar cada parte)
for (const elemento of numeros) {
    console.log(elemento)
}

let cadena = "Hola";

for (const e of cadena) {
    console.log(e);
}