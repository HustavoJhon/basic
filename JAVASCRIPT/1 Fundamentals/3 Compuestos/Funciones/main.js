//! FUNCIONES

//Funciones Declarada
function estoEsUnaFuncion() {
    console.log("Uno");
    console.log("Dos")
    console.log("Tres")
}


function naFuncionQueDevuelveValor() {
    return "La funcione a retornado  una cadena de texto";
}

//Invocacion de Funciones 
estoEsUnaFuncion();
estoEsUnaFuncion();
estoEsUnaFuncion();


let valorDeFuncion = naFuncionQueDevuelveValor();
console.log(valorDeFuncion)


//Parametros

function saludar(nombre="Desconocido",edad=0) {
    console.log(`Hola Mi nombre es ${nombre} y tengo ${edad} años`)
}

saludar("hustavo",18);

//cuando no le ponemos datos a los parametros podemos poner por defecto en la funcion los datos en los parametros como a nombre-> Desconocido y a edad->0  ya que si no le ponemos esos datos en la funcion nos va a dar como resultado undefiend en nombre y en edad
saludar();


// FUNCIONES DECLARADAS VS FUNCIONES EXPRESADAS
// esta funcion puede ser declarada antes de ser inicializada y funciona igualmente tanto antes como despues

funcionDeclarada();

function funcionDeclarada () {
    console.log("Esto es una funcion declarada puede invocarse desde cualquier parte del codigo incluso antes de que la variable sea declarada");
}

funcionDeclarada();

// FUNCINOES ANONIMAS
// esta funcion que esta dentro de una variable no puede ser declarada antes de ser inicializada

// functionExpresada(); ///ERROR 

const functionExpresada = function () {
    console.log("Esto es una funcion expresada, es decir una funcino que se le ha asignado como valor a una variable, si invocamos esta funcion antes de su definicion JS nos dira que no tiene acceso a la funcino por que aun no esta inicializada")
}

functionExpresada();
