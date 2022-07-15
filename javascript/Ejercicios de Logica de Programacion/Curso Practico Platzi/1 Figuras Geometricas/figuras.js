//!CUADRADO
console.group("CUADRADO"); //group sirve para agrupar los console.log

const ladoCuadrado = 5;
console.log(`Los lados del Cuadrado miden: ${ladoCuadrado} cm`);
// todo: Para sacar el perimetro de un Cuadrado
const perimetroCuadrado = ladoCuadrado * 4;
console.log(`El perimetro de el Cuadrado es ${perimetroCuadrado} cm`)
// todo: Para sacar el Area del cuadrado
const areaCuadrado = ladoCuadrado * ladoCuadrado;
console.log(`El area de el Cuadrado es ${areaCuadrado} cm2`)

console.groupEnd();
// *******************************************
//! TRIANGULO
const ladoTriangulo1 = 6;
const ladoTriangulo2 = 6;
const baseTriangulo = 4;
const alturaTriangulo = 5.5;

console.group("TRIANGULO");

console.log(`Los lado del triangulo miden ${ladoTriangulo1} cm , ${ladoTriangulo2} cm y la base mide ${baseTriangulo} cm`);
console.log(`La altura del triangulo es ${alturaTriangulo} cm`);
// todo: Perimetro
const perimetroTriangulo = ladoTriangulo1 + ladoTriangulo2 + baseTriangulo;
console.log(`El perimetro del triangulo es ${perimetroTriangulo} cm`);
// todo: Area
const areaTriangulo = (baseTriangulo * alturaTriangulo) / 2;
console.log(`El area del triangulo es ${areaTriangulo} cm2`)

console.groupEnd();

// *******************************************
//! CIRCULO
// radio, diametro, circunferencia, area
const radioCirculo =  4;
const diametroCirculo = radioCirculo * 2;
const pi = 3.1415; //Math.PI
const perimetroCirculo = diametroCirculo * pi;
const areaCirculo = (radioCirculo * radioCirculo) * pi;

console.group("CIRCULO");

console.log(`El diametro del circulo es ${diametroCirculo} cm`);
console.log(`El perimetro del circulo es ${perimetroCirculo} cm`)
console.log(`El area del circulo es: ${areaCirculo} cm`);

console.groupEnd();