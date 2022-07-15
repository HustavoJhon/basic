// const tiposDeSuscriptores = "Basic";

// if (tiposDeSuscriptores == "Free") {
//     alert("Solo puedes tomar los cursos gratis")
// } else if (tiposDeSuscriptores === "Basic") {
//     alert("Puedes tomar casi todos los cursos de Platzi durante un mes")
// } else if (tiposDeSuscriptores === "Expert") {
//     alert("Puedes tomar casi todos los cursos de platzi durante un año")
// } else if (tiposDeSuscriptores === "ExpertPlus") {
//     alert("Tu y alguien mas pueden tomar todos los cursos de paltzi durante un año")
// }
// *****************************
// const tipo = ["Free","Basic","Expert","ExpertPlus"];
// const context = ["Solo puedes tomar los cursos gratis","Puedes tomar casi todo los cursos por un mes","Puedes tomar casi todos los cursos de platzi durante un año","Tu y alguien mas pueden tomar todos los cursos de platzi durante un año"];

// if (tiposDeSuscriptores === tipo[0]) {
//     alert(context[0])
// } else {
//     alert(context[1])
// }
// *********************

// let i = 0;
// while(i < 5) {
//     alert(`El valor de i es: ${i}`)
//     i++;
// }

// let j = 10;
// while (j >= 2) {
//     alert(`El valor de i es: ${j}`);
//     j--; 
// }

// ********************

let yes = true;
while (yes === true) {
    let usuario = prompt("Cuanto es 2 + 2");
    let resultado = parseInt(usuario);

    if (resultado === 4) {
        alert("Correcto");
        break;
    } else {
        alert("Sigue intentando");
    }
}