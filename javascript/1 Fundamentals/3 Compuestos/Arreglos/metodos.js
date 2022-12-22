/*
    !ARREGLOS
*/

var nombre = ['Hustavo','Jose','Lucas','Carlos'];

var apellido = new Array('Velasquez','Torres','Paguros','Quispe');

console.log(nombre[3]); //Carlos
console.log(apellido[1]); //Torres

//? con length nos muestra la longitud (cantidad)
console.log(nombre.length); 

console.log("***********")
// recorrer los elementos de nombres con un loop for
for (let i = 0; i <= nombre.length - 1; i++){
    console.log(nombre[i]);
}


console.log("***********")
// recorrer con forEach
apellido.forEach(function (elemento, indice) {
    console.log(elemento, indice)
})

console.log("******METODOS PARA ARREGLOS*****")

console.log(nombre);

//? con push agrego un elemento al final de la lista
nombre.push('Pedro');
console.log(nombre)

//? con unshift agrego un elemento al principio del arreglo
nombre.unshift('Lili');
console.log(nombre);

//? con pop eliminamos el ultimo elemento del arreglo
nombre.pop();
console.log(nombre);

//? con shift dejo el arreglo como lo tenia al principio
nombre.shift();
console.log(nombre);

//? con indexOf podemos saber el indice de cada elemento
var pos = nombre.indexOf('Carlos');
console.log(pos);

//? con splice podemos eliminar sabiendo su (indice, cantidad de elementos)
nombre.splice(0,2)
console.log(nombre);
