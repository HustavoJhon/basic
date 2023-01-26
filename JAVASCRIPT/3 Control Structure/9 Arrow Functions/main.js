//? nos ayuda a escribir las funciones de una forma mas facil

// todo normal asignada a una cosntante
const saludar = function () {
    console.log("Buenos dias");
}

saludar();


// todo una arrow function
const hola = () => {
    console.log("Hola");
    // console.log("Otra cosita ps");
}
hola();

// ?cuando tengo una sola instruccion en mi funcion puedo usarla asi:
// y solo cuando tien un parametro se puede usar sin parentesis
const hey = name => console.log(`Hey como estas ${name}`);
hey("hustavo");


// todo tambien puede usarse como un return
const sumar = (a,b) => a+b;
console.log(sumar(1,8));

// todo en varias lineas de codigo se respeta la funcion
const funcionDeVariasLineas = () => {
    let fistvariable;
    let nada;
    let xdfdf;
}

// todo en un arreglo
const numeros = [1,2,3,4,5];
numeros.forEach((elemento,index) => console.log(`Ele elmento ${elemento} esta en la posicion ${index}`));

//! USO DE THIS
// ?en una funcion normal
function perro() {
    console.log(this);
}
perro(); //nos muestra algo global
//ahora dentro de un objeto
const dog = {
    nombre: "firu",
    //la funcion nomal
    ladrar() {
        console.log(this); //nos muestra el contexto en el que estan 
        // nos muestra que esta en el objeto
    }
}
dog.ladrar();


//? en una arrow fuction
// todo en esta funcion nos dice
const cat = {
    nombre: "michi",
    ladrar: () => console.log(this) 
}
cat.ladrar(); //nos muesta una funcion global


//! no es una buena practica crear dentro de un objeto literal el arrow funtion por el problema del contexto (this)

// los objetos dentro de un metodo literalse crean de la forma normal