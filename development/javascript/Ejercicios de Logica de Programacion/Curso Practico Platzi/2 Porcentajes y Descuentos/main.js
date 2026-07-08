// * SOLUCION 1
// const PRECIO_ORIGINAL = 120;
// const DESCUENTO = 18;
// const PORCENTAJE_PRECIO_CON_DESCUENTO = 100 - DESCUENTO;
// const PRECIO_CON_DESCUENTO = (PRECIO_ORIGINAL * PORCENTAJE_PRECIO_CON_DESCUENTO) / 100;
// console.log({
//     PRECIO_ORIGINAL,
//     DESCUENTO,
//     PORCENTAJE_PRECIO_CON_DESCUENTO,
//     PRECIO_CON_DESCUENTO
// });


// * SOLUCION 2
function calPrecioConDescuento(precio, descuento) {    
    const PORCENTAJE_PRECIO_CON_DESCUENTO = 100 - descuento;
    const PRECIO_CON_DESCUENTO = (precio * PORCENTAJE_PRECIO_CON_DESCUENTO) / 100;

    return PRECIO_CON_DESCUENTO;
}

//? CUPONES
const coupons = [
    "cupon1",
    "cupon2",
    "cupon3"
];


// * SOLUCION 3

function calPriceDiscount() {
    const inputPrice = document.getElementById("inputPrice");
    const priceValue = inputPrice.value;

    const inputDiscount = document.getElementById("inputDiscount");
    const discountValue = inputDiscount.value;

    const precioConDescuento = calPrecioConDescuento(priceValue,discountValue);

    // alert(`El total a pagar es de ${precioConDescuento}`);
    const resultPrice = document.getElementById("resultPrice");
    resultPrice.innerText = `El precio con descuento es de $${precioConDescuento}`;
}

//? RETO CON CON SWITCH
function calPriceCupones() {
    const inputPrice = document.getElementById("inputPrice");
    const priceValue = inputPrice.value; 
    const inputDiscount = document.getElementById("inputDiscount");
    const discountValue = inputDiscount.value;
    // resultado del descuento
    const precioConDescuento = calPrecioConDescuento(priceValue,discountValue);
    
    const inputCupon = document.getElementById("inputCupones");
    const cupon = inputCupon.value;
    switch (cupon) {
        case 'messi':
            const cuponMessi = (precioConDescuento * 50) / 100;
            const precioFinal1 = (precioConDescuento - cuponMessi)
            resultCupon.innerText = `El precio con descuento del cupon es de $${precioFinal1}`;
            break;
        case 'cr7':
            const cuponCr7 = (precioConDescuento * 5.00) / 100;
            const precioFinal2 = (precioConDescuento - cuponCr7)
            resultCupon.innerText = `El precio con descuento es de $${precioFinal2}`;
            break;
        case 'scofield':
            const cuposcofield = (precioConDescuento * 30) / 100;
            const precioFinal3 = (precioConDescuento - cuposcofield)
            resultCupon.innerText = `El precio con descuento es de $${precioFinal3}`;
            break;
        default:
            resultCupon.innerText = `El Cupon no existe Vuelve a intentar`;
            break;
    }

    
    
    // RESULTADO DEL DESCUENTO DE CUPONES
    // const resultCupon = document.getElementById("resultCupon");
    // resultCupon.innerText = `El precio con descuento es de $${precioConDescuento}`;
}