//! IF

//? en una sola linea
if (10) console.log("10 es truthy"); //10 es truthy
if (false) 
    console.log("hola"); //no va a ejecutar nada por que la condicion es false

//? en blocke
if (10 === '10') {
    console.log("hola");
}

//! ELSE
let edad = 20;
if (edad >= 18) {
    console.log("Eres mayor de edad");
} else {
    console.log("Eres menor de edad");
}

//! ELSE IF
let calificacion = 9;

if (calificacion == 10) {
    console.log("Exelente");
} else if (calificacion > 7) {
    console.log("Muy Bien");
} else if (calificacion > 5){
    console.log("Puedes Mejorar");
} else{
    console.log("reprobaste")
}