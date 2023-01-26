var edad = 18;

if (edad === 18) {
    console.log("Puedes votar, sera tu 1 votacion");
} else if (edad > 18) {
    console.log("Puedes votar denuevo");
} else {
    console.log("Aun no puedes votar");
}

//!OPERADOR TERNARIO
//? condition ? true: false;

var numero = 1;
var resultado = numero === 1 ? "Si soy el numero 1" : "No, no soy el numero 1";
console.log(resultado)