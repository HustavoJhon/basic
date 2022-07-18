<h1 align="center"> CSS </h1>
> hoja de estilos en cascada

## Selectores
de tipo: div
de clase: .elemeto
de id: id-del-elemento
de atributo: a[href=""]
universal: *

## Selectores combinadores
Ahora que ya entendemos los selectores básicos, vienen los combinadores, que realmente nos ayudan a ser muchos más específicos 👀.
.

Descendientes: En este selector, el que está más a la izquierda es el selector padre y los que están a su derecha serán los selectores hijos, por ejemplo:
.
carbon (8).png
.
En este selector, el div es el padre y el .rojito es el hijo. ¿Esto qué significa? Básicamente que este selector va a agarrar cualquier elemento que tenga la clase “rojito”, pero este elemento tiene que estar DENTRO de un div. O sea, si encuentra algún elemento que tenga la clase .rojito, pero este elemento NO está dentro de un div, pues ese elemento NO lo va a agarrar 😄.
.
carbon (9).png

Hijo directo: Este caso es similar al anterior, pero ahora solo agarrará a los hijos que este directamente adentro del padre. Recuerda que en el elemento padre nosotros podemos tener más cajitas con más elementos dentro, en este caso, este selector NO agarrará a esos otros elementos, por ejemplo:
.
carbon (11).png
.
Al poner el signo > estoy especificando que quiero seleccionar a cualquier elemento que tenga la clase .rojito, pero este elemento tiene que estar directamente dentro de un <div>.
.
carbon (12).png
.
En este ejemplo el <span> que está dentro del <section> NO sería seleccionado porque, aunque está dentro del <div> NO está directamente dentro del <div> 😄.

Elemento adyacente: Básicamente selecciona a la etiqueta que esté debajo de la primera etiqueta 👀.
.
carbon (13).png carbon (14).png
.

General de hermanos: Es similar al adyacente, pero esta vez no solo selecciona al de abajo, sino a cualquiera que esté al mismo nivel que el selector original, es decir, a sus hermanos :3
.
¡Y podemos combinar más selectores! OwO. Por ejemplo:
.
Puedo seleccionar a todas las etiquetas <p> que estén directamente adentro de cualquier elemento con la clase azulitos y que a su vez, estos elementos sean hermanos de algún elemento con la clase .rojito y que este elemento con la clase .rojito esté dentro de un <div> OwO:
.
carbon (15).png
.
Este es un selector MUY específico, y podemos ser aún más específicos, pero quiero que veas como podemos empezar a combinar selectores, y muchas veces es más fácil leer estos selectores de derecha a izquierda 😄.
.
Este puede ser un tema un poco complicado, ¡no te desanimes! Recuerda los consejos, para a practicar un rato, experimenta, curiosea, repite la clase si es necesario, pero hazlo hasta que lo consigas, ¡sé que tú puedes! >:3

https://platzi.com/clases/2467-frontend-developer/40835-tipos-de-selectores-basicos-y-combinadores/