// var auto = {
//     marca: "Toyota",
//     modelo: "Corola",
//     annio: 2021,
//     detalles: function () {
//         console.log(`Auto  ${this.marca} ${this.modelo} ${this.annio}`)
//     }
// };

// console.log(auto);

// ? acceder
// console.log(auto.marca);
// console.log(auto.modelo);
// console.log(auto.annio);

// console.log(auto.detalles());


//! Funciones Constructoras

function auto(marca,modelo,annio) {
    this.marca = marca;
    this.modelo = modelo;
    this.annio = annio;
}

var autoNuevo = new auto("tesla","modelo3","2020");
console.log(autoNuevo);

var autoNuevo2 = new auto("toyota","corola","2019");
console.log(autoNuevo2);

