# Curso de Introducción al Pensamiento Computacional con Python
 
> https://github.com/karlbehrensg/introduccion-pensamiento-computacional

> Comienza tu camino en el desarrollo de software con el lenguaje de programación Python. Entiende la estructura de pensamiento necesaria para resolver problemas en programación. Domina las estructuras de control para crear soluciones. Conoce las características de Python que te permiten reutilizar código. Prueba tu código e implementa correcciones y mejoras.

- Desarrolla un pensamiento lógico para escribir software
- Domina los tipos de datos, objetos y expresiones
- Conoce las características que hacen diferente a Python
- Prueba tu código de manera profesional para identificar fallas

## Introduccion al pensamiento computacional


## Introduccion al computo

**HISTORIA :**

- La primer computadora fue griega, calculaba las posiciones del sol, la luna y del zodiaco. Si sabes la posición del sol sabes cuando preparar tus campos, tus batallas.

> Se usaban plumas de ganzo para escribir.

- El **telar** de Jakart fue el siguiente paso, los ponch card representaban la información. Por primera vez nos dimos cuenta que podemos separar el cálculo de los procedimientos.
---
- **Babage** ideo una máquina que generara cálculos, mediante engranajes.Podíamos separar las instrucciones del cálculo.
El censaje en EEUU utilizaba ponch cards para de manera precisa y más rápida contar la cantidad de ciudadanos. Esto abrió otra demanda. 
La **“Computadora”** era una profesión, es decir personas que realizaban una serie de algoritmos para dar resultados. Las compañías de las que su existencia dependía los cálculos, como ferroviarias, bancos exigían cálculos de mayor escala.

> Las computadoras humanas no estaban a la altura.
---
- **Alan Turing y Alonso Church**, entendieron que todos los algoritmos de la humanidad eran en principio el mismo, la misma secuencia de soluciones matemáticas. A partir de aquí se dió la carrera de construir la primera computadora electrónica. ENIAC (Electronic Numerical Integrator and computer) creada en 1943.Era posible recalcular, pero la conexiones en esta máquina tomaba semanas y usaba sistemas decimales.

> Usamos el sistema binario porque da ventajas en términos de electrónica.

- **Von Neuman** se da cuenta que es posible dentro de los componentes electrónicos, almacenar los procediemitnos y resultados. Así nace la primer computadora de nombre EDVAC (Electronic Discrete Variable Automatic Computer), era una máquina basada en el sitema binario y fue considerada un éxito en el mundo de la informática. Nace la los cimientos de la computación como la conocemos.

- De allí existió la tendencia de la reducción de transistores, para disminuir el tamaño de la máquina, el waffer de silicon es el sitio donde ocurren los cálculos en la computadora en la actualidad, es una gran arquitectura minuaturizada, y solo fue nuestro poder de manipular la luz lo que nos ha permitido llegar a esto, al nivel de microns se ven las estructuras de los chips y solo los microscopios de electrones son aquellos instrumentos que permiten ver a estas escalas.

> Hoy existe la nube, existen miles de computadoras en un data center. Miles de computadoras operando en conjunto para darnos el mundo que tenemos.

- Existen probelmas que todavía no podemos resolver de manera computacional. Richar Feyman, profesor de física, dió las bases del cómputo cuantico.

> No es posible simular computos cuanticos sin sistemas cuanticos.

- Hoy en día existen proyectos para poder materializar las especulaciones matemáticas. En 2019 IBM lanzó el “Q System One” fue la primer computadora introducida para uso comercial, el 20 Septiembre, Google afirmó haber alcnazado la “supremacía cuantica”. Los resultados de estos avances están por sentirse.

## Introduccion a los lenguajes de programacion

**Instrucciones.**

- **Conocimiento delcarativo:** Relaciones existentes entre diversas variables
- **Conocimiento imperativo:** Como podemos llegar a un resultado.

**Algoroitmo :**

Es una lista finita de instrucciones que describen un cómputo, que cuando se ejecuta con ciertas entradas ejecuta pasos intermedios para llegar a un resultado.
Las ideas de los algoritmos dieron la idea de lenguajes progamación.

**¿Donde empezaron todo?**

Con Ada Lovelace contemporánea de Babbage, con las bases teóricas del motor analítica.
Ella escribió lo que sería la serie o número de Bernulli. La **serie o números de Bernulli** : Es una secuencie de números racionales que ocurren frecuentemente en la teoría de números, Ada, escribió para el motor analítico un algoritmo generado para esta secuencia y por ello, se conoce a los números de Bernoulli como el primer “programa de computación”.
Para una lectura profunda del tema:

[**Bernoulli Numbers**](https://en.wikipedia.org/wiki/Bernoulli_number#A_binary_tree_representation)

[**Bernoulli Numbers and Programming**](https://rosettacode.org/wiki/Bernoulli_numbers#Python)

---

Las computadoras entienden una secuencia finita de 0 - 1, representan qué instrucción voy a dar y donde.
Para desarrollar Grace Hopper fue la primera en entender que puede escribir una serie de instrucciones que leen otro progama que puede entender la computadora. Esta idea fue la fundó los lenguajes de programación.

Es decir un lenguaje humano que pudieramos traducir instrucciones de cómputo al lenguaje de 0 - 1.

Para ser gráficos hubo un salto desde:
```
00001010 01110000 01110010 01101001 01101110 01110100 00101000 00100010 01001110 01110101 01101110 01100011 01100001 00100000 01110000 01100001 01110010 01100101 01110011 00100000 01100100 01100101 00100000 01100001 01110000 01110010 01100101 01101110 01100100 01100101 01110010 00100010 00101001 
```
[Entender más sobre
Hasta esto:](https://es.convertbinary.com/)

```python
print("Nunca pares de aprender")
```

Esta misma idea de generar compiladores. El profesor **Dennis Ritchie** inventó el lenguaje C y cocreador, junto con Ken Thompson, del sistema operativo Unix. La sintáxis en que está escrito es muy eficiente, y su estructura está muy cerca al lenguaje máquina, es casi la base de todos los lenguajes de programación. Posterior tenemos a Guido Van Rossum, los volvió comprensibles, los acercó al lenguaje natural.
Eliminó casi todos los símbolos para poder volverlo más legible. Por eso Python es tan cercano al aprendizaje del lenguaje natural y es usado como lenguaje de cimientos.

- Los lenguajes de programación modernos se les llama Turing completness en los que podemos implementar cualquier algoritmo.
Todos los lenguajes de programación poseen:

- **Sintaxis :**
Define la secuencia de símbolos que está bien formada. La morfolgía de la palabra que usamos.

- **Semánica estática :**
Define qué enunciados con sintaxis correcta tiene significado. Habla de cuales combinación representan una idea.

- **Semánica :**
Define el significado. En los lenguajes de programación solo hay un significado. Cual de todos los significados puede ser el correcto. No existe ambiguedad, en programación no existe el contexto.