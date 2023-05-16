# PRUEBAS Y DEBUGGING

## Pruebas de caja negra

Las pruebas de caja negra se basan en la especificación de la función o el programa, aquí debemos probas sus inputs y validar los outputs. Se llama caja negra por que no necesitamos saber necesariamente los procesos internos del programa, solo contrastar sus resultados.

Estos tipos de pruebas son muy importantes para 2 tipos de test:

- **Unit testing:** se realizan pruebas a cada uno de los módulos para determinar su correcto funcionamiento.

- **Integration testing:** es cuando vemos que todos los módulos funcionan entre sí.

Es una buena práctica realizar los test antes de crear tus lineas de código, esto es por que cualquier cambio que se realice a futuro los test estaran incorporados para determinar si los cambios cumplen lo esperado.

En Python existe la posibilidad de realizar test gracias a la libreria unittest. Puede ser que el siguiente código no lo entiendas en su totalidad, pero en una próxima guía detallare mas el tema de clases en programación. Por ahora te mostrare como se realizan estos test.

## Pruebas de caja de cristal

Se basan en el flujo del programa, por lo que se asume que conocemos el funcionamiento del programa, por lo que podemos probar todos los caminos posibles de una función. Esto significa que vamos a probar las ramificaciones, bucles for y while, recursiónes, etc.

Este tipo de pruebas son muy buenas cuando debemos realizar:

- **Regression testing o mocks:** descubrimos un bug cuando corremos el programa, por lo que vamos a buscar el bug gracias a que conocemos como esta estructurado el código.

## Debugging

**Reglas generales**

- Note molestes con el debugger. Aprende a utilizar el print statement.
- Estudia los datos disponibles
- Ten una mente abierta. Si entendieras el programa, probablemente no habria bugs.
- Lleva un registro de los que has trabajado, preferentemente en forma de tests.

**Disenio de experimentos**

- Debugar es un proceso de busqueda. Cada prueba debe acotar el espacio de busqueda.
- Busqueda binaria con print statements.

**Errores comunes**

- Encuentra a los sospechosos comunes.
- En lugar de preguntarte por que un programa no funciona, preguntate por que esta funcionando de esta manera.
- Pasar los argumentos en un orden incorrecto
- Nombres escritos de manera errónea
- No inicializar una variable que era necesaria inicializar
- Tratar de comparar 2 flotantes con igualdad exacta, en lugar de usar una aproximación
- Usar un “is” cuando era un “==” o viceversa
- Las funciones pueden tener efectos secundarios
- Generar alias