//! ARREGLOS

// [] -> esto es un arreglo 

const a = [];
const b = [1, true, "hola",["a","b","c"]];
console.log(a);
console.log(b.length);

// para poder acceder a los elemntos de de los arreglos
console.log(b[2]);
console.log(b[3]);
console.log(b[3][2]); //un arreglo dentro del otro arreglo

const c = [1,true,["a","b",[1,2,4]]];
console.log(c[2][2][2]); //4


//? otra forma de crear un arreglo

const d = Array.of("X","Y","Z",8,4,5);
console.log(d);

const e = Array(100).fill(false); //para crear 100 arrays y cada uno tiene un valor de false
console.log(e);

//? otras formas de crear arreglos pero que ya nadie usa (antiguio)

const f = new Array();
console.log(f);

const g = new Array(1,2,3,true);
console.log(g);


//? Metodos

const colores = ['rojo', 'verde', 'azul'];
console.log(colores);

// agregar - push
colores.push('negro');

//elimina el ultimo elemento - pop
colores.pop();
console.log(colores)

// ejecutar una funcino por cada elemento que tenga
colores.forEach(function (element) { //esto es un iterador para que recorra el array y muestre cada uno por uno
    console.log(element);
})

// tambien podemos mostrar el indice de cada elemento de un arreglo con foreach en los parametros
colores.forEach( function(elemento,indice) {
    console.log(indice + " -> " + elemento);
});