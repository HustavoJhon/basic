//! SCOOPE
// es una coleccion de variables funciones y objetos que estan a tu alcance en algun punto de tu codigo o tambien es decir que variables estan a mi alcance de uso y cuales no

/**
 * Scoope Global 
 * Scoope Local (funcion)
*/

//* Scoope Global -► el valor de la variable puede cambiar fuera y dentro de la funcion
var nombre = "Hustavo";
console.log(nombre);
decirHola();
console.log(nombre);
function decirHola() {
    nombre = "Juan";
    console.log("Hola " + nombre);
}

//* Scoop Local
// cuando tiene un alcance local puede tener otra variable con el mismo nombre ya que no afecta a la variable que tienes un alcance local que esta dentro de una funcion

var edad = 22;

function clear(params) {
    var edad = 12; //tambien se puede crear la variable sin declararce(sin var) y se hace una variable de alcance global
    console.log(edad)
}
clear();


// **********************************************************************
//! ALCANCE DE FUNCION Y DE BLOQUE

// el alcande de variables con 
//? Var tiene un alcance de funcion
function hola(nombre) { //alcance de funcion

    if(nombre){ //alcance de bloque
        var saludo = "hola" + nombre;
    }

    console.log(saludo);
}
hola(" Uriel");

//? Let y Const tiene un alcance de bloque
function hola(nombre) { //alcance de funcion

    if(nombre){ //alcance de bloque
        let saludo = "hola" + nombre;
        console.log(saludo); // aqui si se va a poder ejecutar la variable
    }

    console.log(saludo);
}
hola(" Uriel"); //error