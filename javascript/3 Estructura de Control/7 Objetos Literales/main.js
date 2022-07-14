//? FORMA ESTANDAR QUE SE ESCRIBIA ANTES DEL 2015
let nombre = "Hustavo",
    edad = 18;

    const perro = {
        nombre: nombre,
        edad: edad,
        ladrar:function () {
            console.log("Perro")
        }
    }

    console.log(perro);
    perro.ladrar()

//? ESTA ES LA FORMA ACTUAL DE ESCRIBIR DESDE EL ESMAC 6

    const dog = {
        nombre,
        edad,
        raza:"Buldog",
        ladrar() {
            console.log("GUa")
        }
    }
    console.log(dog)
    dog.ladrar()