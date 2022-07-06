//! Operadores de Comparacion

/*
  (==)  Igual
  (===) Estrictamente Igual
  
  (!=)  Desigualdad
  (!==) Desigualdad Estricta
  
  (>)   Mayor que
  (<)   Menor que
  
  (>=)  Mayor o igual que
  (<=)  Menor o igual que

  una buena practica es usar los operadores estrictos que los normales
*/


// ***********************************************************
let resultado1 = 2 == 2; //true
let resultado2 = 3 === 4; //false
let cadena1 = "uriel" == "uriel"; //true
let cadena2 = "uriel" === "Uriel"; //false
//? lo conbierten en un valor en comun empatan en tipo y empatan en contenido
let cadena5 = "24" == 24; //true
// en igualdad estricta diferencia de string y number
let cadena6 = "24" === 24; //false
// ***********************************************************
let resultado3 = 13 != 20; //true
let resultado4 = 15 !== 15; //false
let cadena7 = "13" !== 24; //true
let cadena8 = "24" != 24; //false
// ***********************************************************
let resultado5 = 2 > 1; //true
let resultado6 = 5 < 2; //false
//? se compara el lexico grafico (valor numerico de los caracteres)
// las que estan al final(alfabeticamente) tienen un valor mayor que las que estan al principio tienen un valor menor
// cuando las dos letras principales son iguales se compara las siguientes
// es de las minusculas hasta las mayusculas a-z despues A-Z por lo que las minusculas tiene un mayor valor que las mayusculas (lexico grafico)
let cadena3 = "Uriel" > "Cody"; //true
let cadena4 = "aurelio" < "Aurelio"; //false
// ***********************************************************
let resultado7 = 5 >= 5; //true
let resultado8 = 4 <= 6; //false
// ***********************************************************
console.log(cadena7);

