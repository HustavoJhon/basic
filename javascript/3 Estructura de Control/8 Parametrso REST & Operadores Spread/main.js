//! PARAMETROS REST
//?nos sirve para tener parametros infinitos con los (...) tres puntos
// TODO calculadora 
function sumar(a,b,...c) {
    let resultado  = a + b;
    c.forEach(function (n) {
        resultado += n
    });
    return resultado;
}
console.log(sumar(30,50))
console.log(sumar(1,2,4,5,5,6,));


//! OPERADORES SPREAD
//? para juntar los arrys y tener una infinidad
//TODO normal
const arr1 = [1,2,3,4,5],
    arr2 = [6,7,8,9,0];
console.log(arr1,arr2);

//TODO con spread
const arr3 = [...arr1, ...arr2];
console.log(arr3)