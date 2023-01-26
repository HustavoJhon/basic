console.log(document.title);

// cambia el titulo de la pagina 
document.title = 'Prueba de titulo';
console.log(document.title);

// me muestra el contenido de el archivo index (body)
console.log(document.body);

// con esta funcion se usa buscando por las etiquetas y selecciondo por el indice para modificarlo a "prueva" el h1 
let h1 = document.getElementsByTagName('h1');
h1[0].innerHTML = "Prueba";
console.log(h1);

// buscamos por el id que le dimos al boton para lugo hacer la funcion de que si hace click mande un alert que dice hola
let btn = document.getElementById('btn');
console.log(btn);
btn.addEventListener('click',mensaje);
function mensaje() {
    alert('Hola')
}