//! IF

//? en una sola linea
// if (10) console.log("10 es truthy"); //10 es truthy
// if (false) 
//     console.log("hola"); //no va a ejecutar nada por que la condicion es false

//? en blocke
// if (10 === '10') {
//     console.log("hola");
// }

//! ELSE
// let edad = 20;
// if (edad >= 18) {
//     console.log("Eres mayor de edad");
// } else {
//     console.log("Eres menor de edad");
// }

//! ELSE IF
// let calificacion = 9;

// if (calificacion == 10) {
//     console.log("Exelente");
// } else if (calificacion > 7) {
//     console.log("Muy Bien");
// } else if (calificacion > 5){
//     console.log("Puedes Mejorar");
// } else{
//     console.log("reprobaste")
// }

//* ejercice 1
// var nombre = "Grover";
// var esCasado = false;

// if (esCasado == true) {
//     console.log(nombre + " es casado");
// } else {
//     console.log(nombre + " es soltero")
// }

//* ejercice 2 - if anidados
// var nombre ="Lucas";
// var edad = 61;
/*
    edad < 12 es un niño
    edad > 11, es < 18, es un adolescente
    edad > 17, es < 60, es un adulto
    edad > 60 es un anciano
*/
// if (edad < 12){
//     console.log(nombre + ' es un niño');
// } else if ((edad > 11) && (edad < 18)){
//     console.log(nombre + ' es un adolescente');
// } else if ((edad > 17) && (edad < 60)){
//     console.log(nombre + ' es un adulto');
// } else if (edad > 60){
//     console.log(nombre + ' es un anciano');
// } else{
//     console.log("tu edad no tiene sentido")
// }

//* ejercice 3 - SWITCH
var mes = 1;

switch (mes) {
    case 1:
        console.log("Enero");
        break;
    case 2:
        console.log("Febrero");
        break
    case 3:
        console.log("Marzo");
        break;
    case 4:
        console.log("Abril");
        break
    case 5:
        console.log("Mayo");
        break
    case 6:
        console.log("Junio");
        break;
    case 7:
        console.log("Julio");
        break
    case 8:
        console.log("Agosto");
        break;
    case 9:
        console.log("Septiempre");
        break
    case 10:
        console.log("Octubree");
        break;
    case 11:
        console.log("Noviembre");
        break
    case 12:
        console.log("Diciembre");
        break;

    default:
        console.log("mes incorrecto")
        break;
}