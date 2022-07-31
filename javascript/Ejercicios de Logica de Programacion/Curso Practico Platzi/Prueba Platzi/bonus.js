//let type = [ 'free', 'basic',
//    'expert'
//]

//let description = [
//    'GRATIS',
//    'BASICA',
//    'EXPERTA'
//]

//let user = 'expert'

//for (let i = 0; i < type.length; i++) {
//    if (user == type[i]) {
//        console.log(`escojiste la suscripcion ${description[i]}`)
//    }    
//}
//
//
//
//! CONO OBJETOS
const tipos = {
    free: 'cursos gratis',
    basic: 'curos durante un mes',
    expert: 'cursos durante un año'
}

// tipos.free == tipos['free']


function suscripcion(suscripcion) {
    if (tipo[suscripcion]){
        console.log(tipo[suscripcion]);
        return;
    }
    console.warn('este tipo de suscripcion no exite!');
    
}

suscripcion('free');
