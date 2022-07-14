//! Menu de una calculadora

let menu = ['1->suma','2->resta'];
alert(`${menu[0]} \n ${menu[1]}`);

let userNumber = parseInt(prompt("Escoje una opcion"));

let one = parseInt(prompt("fist number"));
let two = parseInt(prompt("secont number"));

let contador = 0;
while (contador <= 10) {
    if (userNumber === 1) {
        alert(one + two)
    }
    contador++;
}

if (userNumber === 1) {
    
}