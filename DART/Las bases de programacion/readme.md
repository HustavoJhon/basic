# Las bases de programacion

### Que es una variable?

- Concepto

    El espacio de memoria donde se puede guardar un valor.

- Identificador

    Es el nombre simbolico que puede tener cada variable y debe ser una sola palabra


### Que es un tipo de dato?

- Concepto

    Son las restricciones que puede tener un valor, para definir su dominio y comportamiento.

- Modernidad

    Algunos lenguajes actuales infieren o deducen el tipo de dato si el programa no le define.

### Como se almacena un valor

- Proceso 

    Sucede cuando un valor de un tipo determinado es introducido en la variable

- Ejemplo

    Cuando en matematicas se utiliza variables para almacenar un numero en especifico.
    `tipo variable = valor`

## Tipos de datos basicos

**Boolenanos**
- Concepto

    El tipo mas primitivo, puede almacenar valores de verdadero o falso.

- Caracteristicas 

    Tipo: `bool`

    Cuando es verdadero se coloca `true` y cuando es falso se coloca `false`.

**Numeros**

- Concepto

    Se utiliza para almacenar solo valores numericos, con o sin decimales.

- Caracteristicas

    Tipo: `int` o `double`

    El double ocupa mas memoria pero puede almacenar los dos tipos.

**String**

- Concepto

    Permite almacenar texto, todo tipo de contenido alfanumerico.

- Caracteristicas

    Tipo: `String`

    Se coloca entre "comillas dobles".

**Colecciones**

- Concepto

    Permite almacenar muchos valores de un mismo tipo a la vez en una misma variable.

- Caracteristicas

    Tipo: `List`

    Se coloca el tipo entre `<>` y los valores entre `[]`

## Tipos de datos avanzados

**Uso de `var`** 

- Inferencias 

    Son cambios en tiempo de compilacion por la computadora.

- Comodin

    Es una forma de usar un comodin que permite despreocuparse por la declaracion.

**`final` vs `const`**

- Inmutabilidad

    Son variables de solo lectura, una vez que se le asigna un valor, no puede ser cambiado.

- Diferencias

    `const` debe ser conocido en tiempo de compilacion, y `final` no.

**Tipo `dyanmic` y su uso**

- Inferencia

    Es igual que el `var`, Dart infiere el tipo de dato.

- Mutabilidad

    Puede cambiar su tipo en cualquier momento, debe usarse solo en casos especificos, porque es mucha responsabilidad.

**Diferencias**

|Puede cambiar|El tipo|El valor|
|--|--|--|
|final/const| X | X |
|var| X | 1 |
|dynamic|1|1|