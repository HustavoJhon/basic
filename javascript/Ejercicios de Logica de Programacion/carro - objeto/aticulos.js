var articulos = [
	{ nombre: '📱', precio: 1000 },
	{ nombre: '💻', precio: 1500 },
	{ nombre: '🖥', precio: 2000 },
	{ nombre: '⌨️', precio: 100 },
	{ nombre: '🖱', precio: 70 },
	{ nombre: '🚗', precio: 30000 },
];

//? METODO FILTER
var articulosFiltrados = articulos.filter(function(articulos){
    return articulos.precio <= 1000;
});
console.log(articulosFiltrados);
//* con arroy function 
var object = articulos.filter((filtrado) => filtrado.precio <= 5000);
console.log(object);


//? METODO MAP  
var nombreArticulos = articulos.map(function(articulos) {
    return articulos.nombre;
});
console.log(nombreArticulos);


//? FIEND
var encuentraArticulo = articulos.find(function(articulo) {
    return articulo.nombre === '📱';
});
console.log(encuentraArticulo);


//? FOREACH
articulos.forEach(function() {
    console.log(articulos.nombre);
});


//? SOME
var articulosBaratos = articulos.some(function(articulo) {
    return articulo.precio <= 1000;
});
console.log(articulosBaratos); //true

//? PUSH
let numArray = [1,2,3,4,5];
function newNum() {
    numArray.push(6,7)
    console.log(numArray);
}
console.log(newNum());

//? SHIFT
let areglo = [9,8,7,6]
let shitArray = areglo.shift()
console.log(areglo);

//? POP
let popArray = areglo.pop();
console.log(areglo);