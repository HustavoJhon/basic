//! Operadores Logicos
/**
 * (&&)  operador AND
 * (||)  operador OR
 * (!)   operador de negacion o NOT
 * (??)  operador nullish coalescing // fucion de nulos o union nulosa
 */

// *********************AND********************************
// retorna verdadero si todas las expreciones son verdaderas
console.log(true && true); //true
console.log(true && false); //false
console.log(false && true); //false
console.log(false && false); //false
//? nos da un valor falso o el ultimo
console.log(10 && 0); //0
console.log(19 && 2); //2
console.log(20 && 5 && false); //false
console.log(3 && true && false); //false
console.log(7 && true & true); //1
console.log("*****************************");

// *********************OR********************************
// retorna verdadero si almenos una exprecion es verdadero
console.log(true || true); //true
console.log(true || false); //true
console.log(false || true); //true
console.log(false || false); //false
//? si ambas son false imprime la ultima
console.log(0 || ""); // ""
console.log(20 || 5 || false); //20
console.log("*****************************");

// *********************NOT********************************
// tiene como trabajo negar una exprecion
// primero lo convierte en un booleano para luego cambiarlo a su opuesto 
console.log(!0); //true
console.log(!1); //false
console.log(!""); //true
console.log(!"a") //false
console.log("*****************************");

// *********************NULLISH COALESCING********************************
// Si el valor de la izquierda es null o undefined imprime el otro valor
//pero si tiene un valor normal imprime el valor de la izquierda
console.log(null ?? "hi");
console.log(false ?? "hello");
console.log("*****************************");

// *****************************************************