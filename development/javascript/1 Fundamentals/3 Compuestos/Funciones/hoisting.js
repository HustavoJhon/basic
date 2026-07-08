//! HOISTING
// nos permite llamar variables antes de ser declaradas 
// pero siempre deben estar declaradas
// ? solo funciona con (var) y funciones

// console.log(x);
// var x;

// ==========================================

// function demo() {
//     var x;
//     console.log(x);
// }
// demo();

// ==========================================

// console.log(num); //undefined
// var num = 10;
// console.log(num); //10

// ==========================================

console.log(suma(10 , 10)); //20

function suma(a,b) {
    return a + b;
}
console.log(suma(12 , 3)); //15