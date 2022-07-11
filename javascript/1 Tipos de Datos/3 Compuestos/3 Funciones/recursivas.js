/*
! FUNCIONES RECURSIVAS
*/

// ejercice 1 - factorial
var factorial = function(n){
    if ((n == 0) || (n == 1)){
        return 1;
    } else {
        return(n * factorial(n - 1));
    }
}

console.log(factorial(5));

// factorial(5) = 5 * factorial(4) = 4 * 24 * factorial(3);