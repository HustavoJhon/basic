//! CONCADENACION
let nombre = "jhon";
let apellido = "hustavo";

let saludo = "Hola mi nombre es " + nombre+ " "+apellido+".";

console.log(saludo)


//! INTERPOLACION DE VARIABLES
//?Template String
// se usa con un backtik (``)
let saludo2 = `Hola mi nombre es ${nombre} ${apellido}.`;
console.log(saludo2);


// otro uso para los basktics seria usar codigo de html identado y no en una sola linea como con las comillas dobles y simples
let ul = `
    <ul>
        <li>Primavera</li>
        <li>Verano</li>
        <li>Otoño</li>
        <li>Invierno</li>
    </ul>
`
console.log(ul)

// otra forma de mostrar codigo html es
let ul2 = "<ul>";
    ul2 += "<li> Primavera </li>";
    ul2 += "<li> Verano </li>";
    // ... y asi susesibamente