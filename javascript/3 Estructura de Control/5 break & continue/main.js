const NUMEROS =[1,2,3,4,5,6,7,8,9,10];

for(let i = 0; i < NUMEROS.length;i++) {
    if (i === 5) {
        continue; //desaparece el 6 por que es el indice que ocupa su espacio
    }
    console.log(NUMEROS[i])
}

for (let e = 0; e < NUMEROS.length; e++) {    
    if (e ===8) {
        break;
    }
    console.log(NUMEROS[e]);
}