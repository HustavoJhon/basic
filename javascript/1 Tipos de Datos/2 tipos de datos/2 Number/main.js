//! Numbers

let a = 2;
let b = new Number(1);
let c = 6.192;
let d = "45.4";

console.log(a,b);

console.log(c.toFixed(1)); //para que muestre una cantidad (1) de los decimales
console.log(c.toFixed(2));

console.log(parseInt(c)); //lo convierte a un entero
console.log(parseFloat(d));

console.log(typeof c, typeof d);

console.log(a + d); //conbierte al numero(a) en un string para concadenarlo
// para que no pase eso le cambio el numero en string a un numero 
console.log(c + parseInt(d));
console.log(c + parseFloat(d));