//function tipoDeSuscripcion(suscripcion) {
//    if ('free') {
//        console.log('El tipo de suscripcion es Free');
//        return;
//    }

//    if ('basic') {
//       console.log('El tipo de suscripcion es basica');
//        return;
//    }

//    if ('expert') {
//        console.log('El tipo de suscripcion es Experta');
//    }
//    
//    if ('expert duo') {
//        console.log('El tipo de suscripcion es Expert Duo');
//    }

    //una alerta en la consola es con warn
//   conosle.log('este tipo de suscripcion no existe');
//}

//tipoDeSuscripcion('free');


//BONUS:
//si ya eres un experto o experta en el lenguaje , te desafio a comentar como replicar este comportamiento con arrys o objetos y un solo condicional


let typeSuscripción = [
"Free",
"Basic", 
"Expert", 
"ExpertDuo"
];
let infoSuscripción = [
"solo puedes tomar los cursos gratis", 
"puedes tomar casi todos los cursos de Platzi durante un mes", 
"puedes tomar casi todos los cursos de Platzi durante un año", 
"tú y alguien más pueden tomar TODOS los cursos de Platzi durante un año"];

let userSuscription = "ExpertDuo";

for (let i=0; i < typeSuscripción.length; i++) {
  if (userSuscription == typeSuscripción[i]) {
  	console.log(`Si estas suscrito al plan ${typeSuscripción[i]} en el cual ${infoSuscripción[i]}`)
  }
}
