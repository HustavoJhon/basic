let edad = 18;
// if (edad >= 18) {
//     console.log("Eres mayor de edad")    
// } else {
//     console.log("Eres menor de edad")
// }

//! if else
// Operdor tenario (condicional) ? verdadero:falsa
// console.log("Operador Ternario");

// ?Operador Ternario
let ternario = (edad >= 18) ? "Eres Mayor de Edad" : "Eres menor de edad";
console.log(ternario);

// otra forma de crear un operador ternario
let ternario2 = (edad >= 18) 
    ? "Ereun adulto"
    : "Eres un niño";

console.log(ternario2);


// ! SWITCH
let dia = 7;

switch (dia) {
    case 0:
        console.log("Domingo")
        break;
    case 1:
        console.log("Lunes");
        break;
    case 2:
        console.log("Martes");
        break;
    case 3:
        console.log("Miercoles");
        break;

    default:
        console.log("Este dia no existe");
        break;
}