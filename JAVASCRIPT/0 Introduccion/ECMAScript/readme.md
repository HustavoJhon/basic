
> ## Que es?
> * ECMAScript es el nombre del estandar internacional que define JavaScript.
> * Definifo por un comite tecnico (TC-39) de ecma internacional.
> * Identificado como Ecma-262 y ISO/IEC 16262.
> * No es parte de la W3C.


|**Edicion**|**Publicacion**|**Cambios**|
|-------|-----------|-------|
|1|1997|Primera edicion|
|2|1998|Cambios editorales para mantener la especificacion completa alineada con el estandar internacional ISO/IEC 16262.|
|3|1999|Se agregaron expresiones regulares, mejor manejo de strings, nuevo control de declaraciones, manejo de excepciones con try/catch, definición más estricta de errores, formato para la salida numérica y otras mejoras.|
|4|Abandonado|La cuarta edición fue abandonada debido a diferencias políticas respecto a la complejidad del lenguaje. Muchas características propuestas para la cuarta edición fueron completamente abandonadas; algunas fueron propuestas para la edición ECMAScript Harmony.|
|5|2009|Agrega el modo estricto strict mode, un subconjunto destinado a proporcionar una mejor comprobación de errores y evitar constructores propensos a errores. Aclara varias ambigüedades de la tercera edición, y afina el comportamiento de las implementaciones del "mundo real" que difieren consistentemente desde esa especificación. Agrega algunas nuevas características, como getters y setters, librería para el soporte de JSON, y una más completa reflexión sobre las propiedades de los objetos.
|5.1|2011|Está completamente alineada con la tercera edición del estándar internacional ISO/IEC 16262:2011.|
|6|2015|ES2015 aka ES6.|
|7|2016|ES2016 aka ES7.|
|8|2017|ES2017 aka ES8.|
|9|2018|ES2018 aka ES9.|
|10|2019|ES2019 aka ES10.|
|ESNext|2020|A partir del 2020 las nuevas actualizaciones al estándar simplemente se bautizarán como ESNext.|

---
## **Babel**
Es un compilador de JavaScript, te permite usar el JavaScript del futuro, **HOY**.
* [Babel](https://babeljs.io/)
* [Learn ES Babel](https://babeljs.io/docs/en/learn)
* [Using Babel](https://babeljs.io/en/setup/)
* [CLI Tools](https://babeljs.io/docs/en/babel-cli/)
* [Plugins Babel](https://babeljs.io/docs/en/plugins/)

Instalacion de paquetes:

```js
    npm install -D babel-cli babel-core babel-preset-env
```

Crear el archivo de configuracion .babelrc

```js
{
    "presets": ["env"],
    "plugins": []
}
```

Crear el script necesario para compilar ES con Babel en el archivo .package.json:

```js
{
  "name": "taller-es",
  "version": "1.0.0",
  "description": "Aprendiendo ECMAScript",
  "main": "index.js",
  "scripts": {
    "es6": "babel src --watch --out-dir dist"
  },
  "devDependencies": {
    "babel-cli": "^6.24.1",
    "babel-core": "^6.25.0",
    "babel-preset-env": "^1.6.0"
  }
}
```
Ejecutar el script en la terminal:
```js
    npm run es6
```