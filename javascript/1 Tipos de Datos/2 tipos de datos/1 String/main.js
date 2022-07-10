//! CADENAS DE TEXTO - String

let nombre = 'Hustavo';
let apellido = "Jhon";
console.log(nombre.length,apellido.length);


let saludo = new String("Hola Mundo");
console.log(saludo);


let lorem = "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Veniam a ipsam provident rem nostrum ducimus totam mollitia reiciendis omnis, at quisquam suscipit facilis autem dolores architecto voluptatem ea laboriosam nihil."


console.log(
    nombre.toLowerCase(), //?minuscula
    apellido.toUpperCase(), //?Mayucula
    lorem.includes("amet"), //? esta amet en la variable lorem?
    lorem.trim(), //? quitan los espacio en blanco que aprecen al principio y al final
    lorem.split(" ") //? genera un arreglo separandolo por un espacio" "
);