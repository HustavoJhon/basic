// const name = document.getElementById('name');

// function saludar() {
//     const inputName = document.getElementById('username');
//     const nameuser = inputName.value;
//     alert(`Hola ${nameuser}`);
// }

// function saludarAll() {
//     const inputName = document.getElementById('username');
//     const nameuser = inputName.value;

//     const inputFriend = document.getElementById('friendname');
//     const nameFriend = inputFriend.value;

//     alert(`${nameuser} te saluda ${nameFriend}`);
// }
console.log("****************************");
// ! FOR
var estudiantes = ["maria","sergio","rosa","daniel"];

function saludar(estudiantes) {
    console.log(`Hola ${estudiantes}`);
}

for (var i = 0; i < estudiantes.length; i++) {
    saludar(estudiantes[i]);
}

console.log("****************************");

// ! FOR of
for(var estudent of estudiantes) {
    saludar(estudent);
}
console.log("****************************");
//! while
while (estudiantes.length > 0) {
    var estudiante = estudiantes.shift();
    saludar(estudiante);
}

console.log("****************************");

console.log("****************************");