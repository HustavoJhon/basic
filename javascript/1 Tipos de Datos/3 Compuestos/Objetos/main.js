//! OBJETOS

let a = new String("Hola")

//todo los objetos se crean en constantes para evitar que cambien la referencia de este en la memoria
const b = {}
console.log(b)

// ?otra forma de crear un objeto pero antigua es
const c = new Object();
console.log(c);

// todo: dentro de un objeto a las variables se le van a llamar atributos/propiedades y a las funciones se les llama metodo
const jhon = {
    nombre: "hustavo",
    apellido: "C.Velasquez",
    edad: 18,
    pasatiempos: ["correr", "teclear"],
    soltero: true,
    contacto: {
        email: "hustavojgon@gmail.com",
        twitter: "@hustavo",
        movil: "923423"
    },
    saludar: function() {
        console.log("Hola");
    },
    decirMiNombre: function () {
        console.log(`Hola me llamo ${this.nombre} ${this.apellido} y tengo ${this.edad} y me puedes seguir como ${this.contacto.twitter} en twitter`)
    }
}

console.log(jhon);

// acceder a los atributos
console.log(jhon["nombre"])
// ?es mejor acceder a los atributos desde un punto que con los corchetes
console.log(jhon.edad);
// acceder a una funcion
console.log(jhon.soltero);

//!arreglos dentro de un objeto
console.log(jhon.pasatiempos); 
console.log(jhon.pasatiempos[1]); //acceder a un elemento de un arreglo

//! objetos dentro de un objeto
console.log(jhon.contacto);
console.log(jhon.contacto.twitter); //accceder a un elemnto del metodo entro del metodo

//! funciones dentro de un objeto
jhon.saludar();
jhon.decirMiNombre();



// !METODOS
// para que me muestre las llaves
console.log(Object.keys(jhon));

// para que me muestre los valores
console.log(Object.values(jhon))

// si tiene una propiedad
console.log(jhon.hasOwnProperty("nombre")); //la llave nombre si tiene un valor (true)
console.log(jhon.hasOwnProperty("nacimiento")); //la llave nacimiento no tiene un valor(false)