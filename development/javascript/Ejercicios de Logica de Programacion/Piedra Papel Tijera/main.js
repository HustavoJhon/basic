var op1 = "Piedra";
var op2 = "Papel";
var op3 = "Tijera";

var bot = op1;

function game() {
    var user = document.getElementById("input");
    var input = parseInt(user.value);

    if (input === 1) {
        alert(`Escojiste ${input}. Piedra`);
        alert(`Buelve a escoger, Es un empate!`);
    } else if (input === 2) {
        alert(`Escojiste ${input}. Papel`);
        alert(`Ganaste`);
    } else if (input === 3) {
        alert(`Escojiste ${input}. Tijera`);
        alert(`Perdiste`)
    }
}