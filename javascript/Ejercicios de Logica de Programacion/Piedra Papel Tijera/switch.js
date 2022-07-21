// var op1 = "Piedra";
// var op2 = "Papel";
// var op3 = "Tijera";

// var bot = op1;

// function game() {
//     var user = document.getElementById("input");
//     var input = parseInt(user.value);

//     switch (true) {
//         case (input === 1):
//             alert(`Escojiste ${input}. Piedra > EMPATE!`)
//             break;
//         case (input === 2):
//             alert(`Escojiste ${input}. Papel > GANASTE!`);
//             break;
//         case (input === 3):
//             alert(`Escojiste ${input}. Tijera > PERDISTE!`);
//             break;
    
//         default:
//             alert("Vuelve a intentar")
//             break;
//     }
// }

var pregunta = prompt("Ingresa tu opción: piedra, papel o tijera "); 
var user = pregunta.toLowerCase();
var options = ["piedra", "papel", "tijera"];
var machine = options[Math.floor(Math.random() * 3)];

switch (true) {
    case (user === machine):
        alert(`Empate`)
        break;
    case (machine === 'piedra' && user === 'papel'):
        alert('Ganaste')
        break;
    case (machine === 'papel'  && user === 'tijera'):
        alert('Ganaste')
        break;
    case (machine === 'tijera' && user === 'piedra'):
        alert('Ganaste')
        break;
    default:
        alert('¡Perdiste!');       
}