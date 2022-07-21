alert("welcome a calculator");

let input = prompt("cual est tu numero");
let user = parseInt(input);

function sumar() {
    let one = prompt("fist Number:");
    let two = prompt("secont Number:");

    let resultado = parseInt(one) + parseInt(two);
    document.write(`El resultado es: ${resultado}`)
}

let menu = ["sumar","restar","multiplicar"];
let contador = true;
let count = 1

while (contador === true) {
    alert("escoje una Opcion");
    alert(menu[])
    if (user === 1) {
        sumar();
    }
}
