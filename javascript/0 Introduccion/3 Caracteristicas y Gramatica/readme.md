
## **Caracteristicas**
* Lenguaje de Alto Nivel
* Interpretado
* Dinamico
* Debilmente Tipado
* Multi Paradigma
* Sensible a MAYUSCULAS y minusculas
* No mecesitas los puntos y comas al final de cada linea

    ### *ISOMORFISMO*
    > Hoy JavaScript, es el unico lenguaje capaz de ejecutarse en los 3 capas de una aplicacion:
    
    * Frontend (JavaScript).
    * Backend (Node.js).
    * Persistencia de Datos (MongoDB, Couch Db, Firebase, etc).
    
    ### *Con JavaScript Puedes:*
    
    * Diseño y Desarrollo Web.
    * Hacer Videojuegos.
    * Experiencias 3D, Realidad Aumentada, Realidad Virtual.
    * Control Hardware (Drones, Robots, Electrodomesticos).
    * Aplicaciones Hibridas y Moviles.
    * Machine Learning.
    * etc.
### Escritura de codigo
> Los **Identificadores** deben comenzar con:
* Una letra o 
* Un signo de dolar **$** o
* Un guion bajo **_**
* Nunca con numeros o caracteristicas especiales.

**Tegnicas para Escribir**

***Snake Case***
* Nombre de Archivos:
```js
    mi_archivo_script.js
```
***UPPER_CASE***
* Constantes:
```js
cont UNA_CONSTANTE = "Soy una constante"
cont PI = 3.141592
```
***UpperCamelCase***
* Clases:
```js
class SerHumano {
    constructor(nombre, genero){
        this.nombre = nombre;
        this.genero = genero;
    }

    miMombreEs() {
        return `Mi nombre es ${this.nombre}`;
    }
}
```
***lowerCamelCase***
* Objetos:
```js
const unObjeto = {
    nombre: "Jonathan",
    email: "hustavo@gmil.com",
};
```
* Primitivos:
```js
let unaCadena = "Hola Mundo",
    unNumero = 19,
    unBoolean = true;
```
* Funciones:
```js
function holaMundo(nombre) {
    alert(`Hola Mundo ${nombre}`);
}
holaMundo("Hustavo");
```
*Instancias:
```js
const ajax = new XMLHttpRequest(),
    jon = new SerHumano("Jonathan", "Hombre");
```
---
## Palabras reservadas
    A: abstract
    B: boolean, break, byte
    C: case, catch, char, class, const, continue
    D: debugger, default, delete, do, double
    E: else, enum, export, extends
    F: false, final, finally, float, for, function
    G: goto
    I: if, implements, import, in, instanceof, int, interface
    L: let, long
    N: native, new, null
    P: package, private, protected, public
    R: return
    S: short, static, super, switch, synchronized
    T: this, throw, throws, transient, true, try, typeof
    V: var, volatile, void
    W: while, with

## Ordenamiento de código
1. IMPORTACIÓN DE MÓDULOS.
2. DECLARACIÓN DE VARIABLES.
3. DECLARACIÓN DE FUNCIONES.
4. EJECUCIÓN DE CÓDIGO.

## Tipos de datos en JavaScript
**Primitivos:** Se accede directamente al valor.

* string
* number
* boolean
* null
*undefined
* NaN

**Compuestos:** Se accede a la referencia del valor.

* object = {}
* array = []
* function () { }
* Class {}
* etc.