//**********1 SOLUCION************* */
// var marca = ["Abarth","Alfa Romeo","Aston Martin","Audi","Bentley","BMW","Cadillac","Caterham","Chevrolet","Citroen","Dacia","Ferrari","Fiat","Ford","Honda","Infiniti","Isuzu","Iveco","Jaguar"];
// var modelo = ["C-Max","Fiesta","Focus","Mondeo","Ka","S-MA","B-MAX","Grand C-Max","Tourneo Custom","Kuga","Galaxy","Grand Tourneo Connect","Tourneo Connect","EcoSport","Tourneo Courier","Mustang","Transit Connect","Edge","Ka+"];
// var anio = ["1988","1989","1978","1989","1928","1989","1968","1989","1888","1989","1288","1989","1938","1989","1988","1999","1983","1989","1918"];
// function auto(marca) {
    // this.marca = marca;
    // this.modelo = modelo;
    // this.anio = anio;
// };
// for(var i = 0; marca.length > i && modelo.length > i && anio.length > i; i++){
//     var nuevoAuto = new auto(marca[i], modelo[i], anio[i]);
//     console.log(nuevoAuto)
// }

//**********2 SOLUCION************* */
// var autos = [];
// function auto(marca, modelo, color) {
//   this.marca = marca;
//   this.modelo = modelo; 
//   this.color = color; 
// }
// for (var i = 0; i < 10; i++ ) {
//   var autoNuevo = new auto ("Tesla", "Model S",  "Blue" )
//   var agregarAuto = autos.push(autoNuevo);
// }
// console.log(autos);


//**********3 SOLUCION************* */
function auto (MARCA, MODELO, ANNIO){
  this.marca = MARCA;
  this.modelo = MODELO;
  this.annio = ANNIO;
}
var autos = [];
for(let i = 0 ; i < 30 ; i++){
  var marca = prompt("Ingresa la marca del auto");
  var modelo = prompt("Ingresa el modelo del auto");
  var annio = prompt("Ingresa el año del auto");
  autos.push(new auto (marca, modelo, annio));
}
for(let i = 0 ; i < autos.length ; i++){
  console.log(autos[i]);
}