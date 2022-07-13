//! MANEJO DE ERRORS

// try {
//     //aqui va el codigo que se va evaluar
//     console.log("Mensaje de Try")
    
//     console.log(/errororxd);
// } catch(error) {
//     //aqui captura cualquier error surgido o lanzado en el try
//     console.log("Mensaje de catch");
//     console.log(error);
// } finally {
//     //aqui se ejecuta siempre al final de un bloque try-catch
//     console.log("Mensaje de Finally");
// }

try {
    let numero = "y";

    if(isNaN(numero)) {
        throw new Error("El caracter introducido no es un Numero");
    }
} catch (error){
    console.log(`se profujo el siguiente error: ${error} `)
}