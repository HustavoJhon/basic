# Flujo de Datos

## Operaciones

**Aritmeticas**

- Que son?

    SOn expresiones matematicas que requieren 2 numeros y retornan un valor numerico.

- Expresiones

    `+, -, -expr, *, /, ~/, %`

**Adignacion**

- Que son

    Son operaciones aritmeticas abreviadas.

- Ejmplo

    `+=, -=, *=, /=, ~/=, =%`

**Relacionales**

- Que son

     Son expresines de comparacion que requieren 2 numeros que retorna un resultado booleano.

- Ejemplo

    `==, =!, =>, >, <, <=`

**Logicas**

- Que son

    Son Expresiones logicas que requierne 1 o 2 booleanos y retorna un resultado booleano.

- Ejemplo

    `||, &&, !, ?`

## Condicionales

**IF**

Es el primer condicionante logico, es el mas basico, se cumple cuando la expresion es verdadera.

```dart
if (expresion) {
    //codigo del if
}
```

**Else**

Es el complemento de `if` y se cumple cuando la expresion dentro del `if` es falso.

```dart
if (expresion) {
    //codigo del if
} else {
    //codigo del else
}
```

**Switch**

Es una condicional que acepta multiples alternativas, se cumple cuando alguna expresion es verdadera mediante casos.

```dart
switch (expresion) {
    case: break;
    case: break;
    default:
}
```
