//! DESTRUCTURACION


//? Con los arreglos
const numeros = [1,2,3,4];

//todo sin desustructuracion
let uno = numeros[0],
    dos = numeros[1],
    tres = numeros[2],
    cuatro = numeros[3]

//todo con destructuracion
const [one, two, three, four] = numeros;
console.log(one,two,three,four)

//todo desustructuracion en los objetos
const persona = {
    nombre: "Hustavo",
    apellido: "Ccarita",
    edad: 18
}
// en este caso debemos poner el mismo nombre que las claves nombre=>nombre
// ademas debemos respetar el orden de las variables
let {nombre,apellido,edad} = persona;
console.log(nombre,apellido,edad)