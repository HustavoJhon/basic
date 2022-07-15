//!CUADRADO
console.group("CUADRADO"); //group sirve para agrupar los console.log
// const ladoCuadrado = 5;
// console.log(`Los lados del Cuadrado miden: ${ladoCuadrado} cm`);
// todo: Para sacar el perimetro de un Cuadrado
// const perimetroCuadrado = ladoCuadrado * 4;
// console.log(`El perimetro de el Cuadrado es ${perimetroCuadrado} cm`)
function perimetroCuadrado (lado) {
    return lado * 4;
}
// todo: Para sacar el Area del cuadrado
// const areaCuadrado = ladoCuadrado * ladoCuadrado;
// console.log(`El area de el Cuadrado es ${areaCuadrado} cm2`)
function areaCuadrado(lado) {
    return lado * lado;
}
console.groupEnd();



// *******************************************
//! TRIANGULO
// const ladoTriangulo1 = 6;
// const ladoTriangulo2 = 6;
// const baseTriangulo = 4;
// const alturaTriangulo = 5.5;
console.group("TRIANGULO");
// console.log(`Los lado del triangulo miden ${ladoTriangulo1} cm , ${ladoTriangulo2} cm y la base mide ${baseTriangulo} cm`);
// console.log(`La altura del triangulo es ${alturaTriangulo} cm`);
// todo: Perimetro
// const perimetroTriangulo = ladoTriangulo1 + ladoTriangulo2 + baseTriangulo;
// console.log(`El perimetro del triangulo es ${perimetroTriangulo} cm`);
function perimetroTringulo (lado1, lado2, lado3) {
    return lado1 + lado2 + lado3;
}
// todo: Area
// const areaTriangulo = (baseTriangulo * alturaTriangulo) / 2;
// console.log(`El area del triangulo es ${areaTriangulo} cm2`)
function areaTriangulo (base, altura){
    return (base * altura) / 2;
}
console.groupEnd();




// *******************************************
//! CIRCULO
// radio, diametro, circunferencia, area

// const radioCirculo =  4;
// const pi = 3.1415; //Math.PI
const PI = Math.PI;
console.group("CIRCULO");
// todo: Diametro
// const diametroCirculo = radioCirculo * 2;
// console.log(`El diametro del circulo es ${diametroCirculo} cm`);
function diametroCirculo(radio) {
    return radio * 2;
}
// todo: Perimetro - Circunferencia
// const perimetroCirculo = diametroCirculo * pi;
// console.log(`El perimetro del circulo es ${perimetroCirculo} cm`)
function perimetroCirculo (radio) {
    const diametro = diametroCirculo(radio);
    return diametro * PI;
}
// todo: Area
// const areaCirculo = (radioCirculo * radioCirculo) * pi;
// console.log(`El area del circulo es: ${areaCirculo} cm`);
function areaCirculo (radio) {
    return (radio * radio) * PI;
}
console.groupEnd();


//! AQUI INTERACTUAMOS CON HTML
function calPerimetroCuadrado() {
    const input = document.getElementById("InputCuadrado");
    const value = input.value;
    
    const perimetro = perimetroCuadrado(value);
    alert(`El Perimetro es: ${perimetro}`)
}

function calAreaCuadrado() {
    // const inpu
}