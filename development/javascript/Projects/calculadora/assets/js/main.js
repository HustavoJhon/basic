const btnNumber = document.getElementsByName('data-number');
const btnOperation = document.getElementsByName('data-operation');
const btnIgual = document.getElementsByName('data-igual')[0];
const btnDelete = document.getElementsByName('data-delete')[0];

var result = document.getElementById('result');
var opeActual = '';
var opeAnterior = '';
var operacion = undefined;

// console.log(btnDelete);

btnNumber.forEach(function(boton) {
    boton.addEventListener('click',function(){
        agregarNumero(boton.innerText);
    }
    )
});

btnOperation.forEach(function(boton) {
    boton.addEventListener('click', function(){
        selectOperacion(boton.innerText);
    }
    )
});

btnIgual.addEventListener('click', function(){
    calcular();
    actualizarDisplay();
});

btnDelete.addEventListener('click', function(){
    clear();
    actualizarDisplay();
});

function selectOperacion(op){
    if(opeActual === '') return;
    if(opeAnterior !== ''){
        calcular()
    }
    operacion = op.toString();
    opeAnterior = opeActual;
    opeActual = '';
}

function calcular(){
    var calculo;
    const anterior = parseFloat(opeAnterior);
    const actual = parseFloat(opeActual);
    if(isNaN(anterior) || isNaN(actual)) return;
    switch (operacion) {
        case '+':
            calculo = anterior + actual;
            break;
        case '-':
            calculo = anterior - actual;
            break;
        case 'x':
            calculo = anterior * actual;
            break;
        case '/':
            calculo = anterior / actual;
            break;
    
        default:
            break;
    }
    opeActual = calculo;
    operacion = undefined;
    opeAnterior = '';
}

function agregarNumero(num){
    opeActual = opeActual.toString() + num.toString();
    actualizarDisplay();
}

function clear(){
    opeActual = '';
    opeAnterior = '';
    operacion = undefined;
}

function actualizarDisplay(){
    result.value = opeActual;
}

clear();