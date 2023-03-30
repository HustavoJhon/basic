# CURSO DE PYTHON: COMPRENSION

## El Zen de Python
> https://elpythonista.com/zen-de-python 

```python
import this
```
<details align="center">
<summary> &#128427;</summary>
Lo bonito es mejor que lo feo. <br>
Lo explícito es mejor que lo implícito. <br>
Lo simple es mejor que lo complejo. <br>
Lo complejo es mejor que lo complicado. <br>
Lo plano es mejor que lo anidado. <br>
Lo disperso es mejor que lo denso. <br>
La legibilidad cuenta. <br>
Los casos especiales no son tan especiales como para romper las reglas. <br>
Aunque la practicidad gana a la pureza. <br>
Los errores nunca deben pasar en silencio. <br>
A menos que se silencien explícitamente. <br>
Ante la ambigüedad, rechaza la tentación de adivinar. <br>
Debe haber una -y preferiblemente sólo una- forma obvia de hacerlo. <br>
Aunque esa manera puede no ser obvia al principio, a menos que seas holandés. <br>
Ahora es mejor que nunca. <br>
Aunque nunca es mejor que *ahora mismo*. <br>
Si la implementación es difícil de explicar, es una mala idea. <br>
Si la implementación es fácil de explicar, puede ser una buena idea. <br>
Los espacios de nombres son una gran idea, ¡hagamos más de ellos!
</details>


## EL Scope 

```
Built-in
    ^
    |
Global scope
    ^
    |
Enclosing scope
    ^
    |
Local scope
```

### ERROS



|**Exception**|**Description**|
|--|--|
|ArithmeticError|	Se genera cuando se produce un error en los cálculos numéricos|
|AssertionError|	Se genera cuando falla una declaración de afirmación|
|AttributeError|	Se genera cuando falla la asignación o la referencia de atributo|
|Exception|	Clase base para todas las excepciones|
|EOFError|	Se genera cuando el método input() alcanza una condición de “fin de archivo” (EOF)|
|FloatingPointError|	Se genera cuando falla un cálculo de punto flotante|
|GeneratorExit|	Se genera cuando se cierra un generador (con el método close())|
|ImportError|	Se genera cuando no existe un módulo importado|
|IndentationError|	Se genera cuando la sangría no es correcta|
|IndexError|	Se genera cuando no existe un índice de una secuencia|
|KeyError|	Se genera cuando una clave no existe en un diccionario|
|KeyboardInterrupt|	Se genera cuando el usuario presiona Ctrl+c, Ctrl+z o Eliminar|
|LookupError|	Se genera cuando no se pueden encontrar los errores generados|
|MemoryError|	Se genera cuando un programa se queda sin memoria|
|NameError|	Se genera cuando una variable no existe|
|NotImplementedError|	Se genera cuando un método abstracto requiere una clase heredada para anular el método|
|OSError|	Se genera cuando una operación relacionada con el sistema provoca un error|
|OverflowError|	Se genera cuando el resultado de un cálculo numérico es demasiado grande|
|ReferenceError|	Se genera cuando no existe un objeto de referencia débil|
|RuntimeError|	Se genera cuando ocurre un error que no pertenece a ninguna expectativa específica|
|StopIteration|	Se genera cuando el método next() de un iterador no tiene más valores|
|SyntaxError|	Se genera cuando se produce un error de sintaxis|
|TabError|	Se genera cuando la sangría consta de tabulaciones o espacios|
|SystemError|	Se genera cuando se produce un error del sistema|
|SystemExit|	Se genera cuando se llama a la función sys.exit()|
|TypeError|	Se genera cuando se combinan dos tipos diferentes|
|UnboundLocalError|	Se genera cuando se hace referencia a una variable local antes de la asignación|
|UnicodeError|	Se genera cuando se produce un problema Unicode|
|UnicodeEncodeError|	Se genera cuando se produce un problema de codificación Unicode|
|UnicodeDecodeError|	Se genera cuando se produce un problema de decodificación Unicode|
|UnicodeTranslateError|	Se genera cuando se produce un problema de traducción Unicode|
|ValueError|	Se genera cuando hay un valor incorrecto en un tipo de datos especificado|
|ZeroDivisionError|	Se genera cuando el segundo operador en una división es cero|