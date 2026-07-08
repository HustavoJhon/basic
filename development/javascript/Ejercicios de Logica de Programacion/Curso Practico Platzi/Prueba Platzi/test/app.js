//* Funciones
const name = "Juan David";
const lastname = "Castro Gallego";
const completeName = name + lastname;
const nickname = "juandc";
function saludar(name=completeName,nick=nickname) {
    console.log(`Mi nombre es ${completeName}, pero prefiero que me digas ${nickname}`);
}
saludar();

//* Condicionales
// Replica el comportamiento de tu condicional anterior con if, else y else if, pero ahora solo con if (sin else ni else if).
//?💡 Bonus: si ya eres una experta o experto en el lenguaje, te desafío a comentar cómo replicar este comportamiento con arrays u objetos y un solo condicional. 

//Ciclos

let contador = 0;
while (contador < 5) {
    console.log(`El valor es: ${contador}`);
    contador++;
}

let i = 10;
while (i >= 2) {
    console.log("xd"+ i);
    i--;
}

//Cuanto es 2 + 2
let respuesta;

while (respuesta != '4') {
    let pregunta = prompt("Cuanto es 2 + 2");
    respuesta = pregunta;
}
