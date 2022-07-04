//! Introduccion

//? Declaracion de Funcion
// function name(parametros) { <code> }

// Example 1
function saluda() {
    console.log("Hola");
}
saluda(); //tambien podemos llamar la funcion antes de la funcion

// Example 2
function cuadrado(numero) {
    return numero * numero;
    console.log("hola"); //TODO: no se ba a ejecutar ya que esta despues de return
}
let cuadrado_de_dos = cuadrado(2);
console.log(cuadrado_de_dos); 
console.log(cuadrado(3));

//? Expresion de Funcion
// let func = function(parametro) { <code> }
// no podemos llamar la funcion despues de la declaracion de la funcion