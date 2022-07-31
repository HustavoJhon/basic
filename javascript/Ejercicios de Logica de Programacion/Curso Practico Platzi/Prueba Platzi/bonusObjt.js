//! CONO OBJETOS
const tipos = {
    free: 'cursos gratis',
    basic: 'curos durante un mes',
    expert: 'cursos durante un año'
}

// tipos.free == tipos['free']


function tipoDeSubs(suscripcion) {
    if (tipos[suscripcion]){

        console.log(tipos[suscripcion]);
        return;
    }
    console.warn('este tipo de suscripcion no exite!');
} 

tipoDeSubs('fresde')
