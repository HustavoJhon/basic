# Excepciones y afirmaciones

## Manejo de excepciones

Los manejos de excepciones son muy comunes en la programación, no tienen nada de excepcional. Las excepciones de Python normalmente se relacionan con errores de semántica, también podemos crear nuestras propias excepciones, pero cuando una excepción no se maneja (unhandled exception), el programa termina en error.

Las excepciones se manejan con los keywords: try, except, finally. Se pueden utilizar también para ramificar programas.

No deben manejarse de manera silenciosa (por ejemplo, con print statements). Para crear tu propia excepción utiliza el keyword raise.

## Excepciones y control de flujo

Hasta ahora hemos visto como las excepciones nos permiten controlar los posibles errores que pueden ocurrir en nuestro código. Sin embargo, dentro de la comunidad de Python tienen otro uso: control de flujo.

En este momento ya debes estar familiarizado con las estructuras de control flujo que ofrece Python (if... elif...else); entonces, ¿por qué es necesaria otra modalidad para controlar el flujo? Una razón muy específica: el principio EAFP (easier to ask for forgiveness than permission, es más fácil pedir perdón que permiso, por sus siglas en inglés).

El principio EAFP es un estilo de programación común en Python en el cual se asumen llaves, índices o atributos válidos y se captura la excepción si la suposición resulta ser falsa. Es importante resaltar que otros lenguajes de programación favorecen el principio LBYL (look before you leap, revisa antes de saltar) en el cual el código verifica de manera explícita las precondiciones antes de realizar llamadas.

Veamos ambos estilos:
```py
# Python

def busca_pais(paises, pais):
    &quot;&quot;&quot;
    Paises es un diccionario. Pais es la llave.
    Codigo con el principio EAFP.
    &quot;&quot;&quot;
    
    try:
        return paises[pais]
    except KeyError:
        return None
```
```javascript
// Javascript

/**
* Paises es un objeto. Pais es la llave.
* Codigo con el principio LBYL.
*/
function buscaPais(paises, pais) {
  if(!Object.keys(paises).includes(pais)) {
    return null;
  }

  return paises[pais];
}
```
Como puedes ver, el código de Python accede directamente a la llave y únicamente si dicho acceso falla, entonces se captura la excepción y se provee el código necesario. En el caso de JavaScript, se verifica primero que la llave exista en el objeto y únicamente con posterioridad se accede.

Es importante resaltar que ambos estilos pueden utilizarse en Python, pero el estilo EAFP es mucho más "pythonico".

## Afirmaciones

Es un mecanismo por el cuál podemos determinar si una función se cumple o no se cumple. Y poder seguir adelante con la ejecución de nuestro programa o terminar dicha ejecución.

Programación defensiva
Pueden utilizarse para verificar que los tipos sean correctos en una función.
También sirven para debuguear.
Para generarlas tenemos que utlizar el keyword `assert` y dar una expresión boleana y un mensaje de error

- Programacion defensiva
- Pueden utilizar para verificar que los tipos sean correctos en una funcion
- Tambien sirven para debuguear

https://www.programiz.com/python-programming/assert-statement
