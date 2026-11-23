//! Catálogo inicial de C#.

use crate::code_lab::catalog::{
    CatalogSeed, ExampleSeed, ExerciseSeed, LanguageSeed, LessonSeed, PathSeed, QuestionSeed,
    TestCaseSeed,
};
use crate::code_lab::domain::Difficulty;

const LANGUAGE: LanguageSeed = LanguageSeed {
    code: "csharp",
    name: "C#",
    sdk_command: "dotnet",
};

const PATHS: &[PathSeed] = &[PathSeed {
    slug: "csharp-fundamentos",
    title: "Fundamentos de C#",
    description: "Del primer programa a la resolución de ejercicios con pruebas automáticas.",
    level: 1,
}];

const LESSONS: &[LessonSeed] = &[
    LessonSeed {
        path_slug: "csharp-fundamentos",
        slug: "primer-programa",
        title: "Tu primer programa",
        summary: "Cómo se organiza un proyecto de consola y dónde empieza la ejecución.",
        content_md: r#"# Tu primer programa en C#

Un proyecto de consola de C# se compila a un ensamblado que se ejecuta con
`dotnet`. En este curso usarás **top-level statements**: el código se escribe
directamente en `Program.cs` y se ejecuta de arriba abajo.

```csharp
Console.WriteLine("Hola desde BASIC Code Lab");
```

## Piezas que intervienen

- **SDK**: el comando `dotnet` compila, restaura y ejecuta.
- **Proyecto**: el archivo `.csproj` describe la versión de .NET y las opciones.
- **Punto de entrada**: con top-level statements no necesitas escribir `Main`.
- **Salida**: `Console.WriteLine` añade un salto de línea; `Console.Write` no.

## Compilación

Code Lab crea un proyecto temporal, ejecuta `restore` sin fuentes externas,
compila en `Release` y luego ejecuta el resultado con cada caso de prueba.

> Abrir esta lección no la marca como completada. Debes responder el
> cuestionario para avanzar.
"#,
        concepts: &["sdk", "proyecto", "consola", "punto-de-entrada"],
        order_index: 1,
        estimated_minutes: 8,
        questions: &[
            QuestionSeed {
                prompt: "¿Qué comando detecta Code Lab para C#?",
                options: &["dotnet", "cargo", "python", "gcc"],
                correct_index: 0,
                explanation: "El SDK de .NET se invoca mediante el comando `dotnet`.",
            },
            QuestionSeed {
                prompt: "¿Qué método añade un salto de línea al imprimir?",
                options: &[
                    "Console.Write",
                    "Console.WriteLine",
                    "Console.Read",
                    "Console.Clear",
                ],
                correct_index: 1,
                explanation: "`Console.WriteLine` escribe el texto y un salto de línea.",
            },
        ],
    },
    LessonSeed {
        path_slug: "csharp-fundamentos",
        slug: "variables-y-tipos",
        title: "Variables y tipos",
        summary: "Declaración inferida, conversiones y lectura desde la entrada estándar.",
        content_md: r#"# Variables y tipos

C# es fuertemente tipado. Con `var` el compilador infiere el tipo a partir del
valor inicial, pero el tipo no cambia después.

```csharp
var nombre = "Ana";      // string
var edad = 34;           // int
var activo = true;       // bool
```

## Conversiones

`int.Parse(texto)` convierte texto a número. Si el texto no es válido lanza una
excepción, así que en ejercicios controlados conviene validar antes.

```csharp
var partes = Console.In.ReadToEnd()
    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);
var a = int.Parse(partes[0]);
```

## Entrada completa

Los ejercicios de Code Lab envían la entrada por `stdin`. `ReadToEnd()` lee
todos los datos del caso, no solo una línea.

> Un caso de prueba compara la salida exacta. No imprimas mensajes de depuración.
"#,
        concepts: &["variables", "tipos", "conversiones", "stdin"],
        order_index: 2,
        estimated_minutes: 10,
        questions: &[
            QuestionSeed {
                prompt: "¿Qué ocurre con una variable declarada con `var`?",
                options: &[
                    "Su tipo se infiere y no cambia",
                    "Su tipo cambia con cada asignación",
                    "Siempre es string",
                    "No necesita valor inicial",
                ],
                correct_index: 0,
                explanation: "`var` infiere el tipo en la declaración; el tipo es estático.",
            },
            QuestionSeed {
                prompt: "¿Qué método lee toda la entrada estándar?",
                options: &[
                    "Console.ReadLine()",
                    "Console.In.ReadToEnd()",
                    "Console.Get()",
                    "Read.All()",
                ],
                correct_index: 1,
                explanation: "`Console.In.ReadToEnd()` consume toda la entrada del caso.",
            },
        ],
    },
    LessonSeed {
        path_slug: "csharp-fundamentos",
        slug: "control-de-flujo",
        title: "Control de flujo",
        summary: "Condiciones, selección múltiple y bucles con ejemplos de consola.",
        content_md: r#"# Control de flujo

Las condicionales permiten tomar decisiones:

```csharp
if (numero % 2 == 0)
{
    Console.WriteLine("par");
}
else
{
    Console.WriteLine("impar");
}
```

## Selección múltiple

`switch` es útil cuando comparas una misma expresión contra varios valores.

## Bucles

- `for`: cuando conoces el número de iteraciones.
- `foreach`: cuando recorres una colección.
- `while`: mientras se cumpla una condición.

```csharp
for (var i = 0; i < 3; i++)
{
    Console.WriteLine(i);
}
```

> Evita `Console.ReadLine()` dentro de bucles si no sabes cuántas líneas hay:
> en Code Lab la entrada se agota y las llamadas posteriores devuelven `null`.
"#,
        concepts: &["condicionales", "switch", "bucles", "modulo"],
        order_index: 3,
        estimated_minutes: 12,
        questions: &[
            QuestionSeed {
                prompt: "¿Qué operador permite saber si un número es par?",
                options: &["/", "%", "^", "&&"],
                correct_index: 1,
                explanation: "`numero % 2 == 0` indica que el resto de la división es cero.",
            },
            QuestionSeed {
                prompt: "¿Qué bucle conviene para recorrer todos los elementos de una lista?",
                options: &["for", "foreach", "while", "do"],
                correct_index: 1,
                explanation: "`foreach` recorre una colección sin gestionar índices.",
            },
        ],
    },
    LessonSeed {
        path_slug: "csharp-fundamentos",
        slug: "metodos-y-colecciones",
        title: "Métodos y colecciones",
        summary: "Funciones, arrays y List<T> para organizar soluciones más grandes.",
        content_md: r#"# Métodos y colecciones

Un método agrupa lógica reutilizable. En top-level statements puedes declarar
funciones locales al final del archivo.

```csharp
static int Maximo(int a, int b, int c) => Math.Max(a, Math.Max(b, c));
```

## Arrays y listas

```csharp
var numeros = new[] { 4, 7, 2 };
var lista = new List<int> { 4, 7, 2 };
lista.Add(9);
```

`List<T>` crece dinámicamente. `Array.Sort`, `Array.Reverse` y LINQ (`OrderBy`,
`Reverse`, `Sum`) resuelven muchas operaciones comunes.

## Consejo para ejercicios

Separa **leer entrada**, **calcular** y **imprimir**. Así puedes cambiar los
casos de prueba sin reescribir la lógica.
"#,
        concepts: &["metodos", "arrays", "listas", "linq"],
        order_index: 4,
        estimated_minutes: 14,
        questions: &[
            QuestionSeed {
                prompt: "¿Qué colección puede crecer con `Add`?",
                options: &["int[]", "List<int>", "string", "bool"],
                correct_index: 1,
                explanation: "`List<T>` es una colección dinámica con método `Add`.",
            },
            QuestionSeed {
                prompt: "¿Dónde conviene declarar una función local en top-level statements?",
                options: &[
                    "Antes del primer using",
                    "Dentro de otra función solamente",
                    "Al final del archivo",
                    "En el archivo .csproj",
                ],
                correct_index: 2,
                explanation: "Las funciones locales se declaran al final del archivo de programa.",
            },
        ],
    },
    LessonSeed {
        path_slug: "csharp-fundamentos",
        slug: "entradas-y-pruebas",
        title: "Entradas y pruebas automáticas",
        summary: "Cómo interpretar enunciados, ejemplos visibles y casos ocultos.",
        content_md: r#"# Entradas y pruebas automáticas

Cada ejercicio define casos de prueba con `input` y `expected_output`. Code Lab
escribe `input` en `stdin`, ejecuta tu programa y compara la salida.

## Normalización

Se ignoran los espacios al final de cada línea y las líneas vacías finales.
El resto de caracteres importan: mayúsculas, signos y orden de las líneas.

## Ejemplos y casos ocultos

Los ejemplos del enunciado son visibles. Los casos ocultos comprueban bordes:
cero, números negativos, entradas vacías o valores repetidos.

## Estrategia

1. Lee el enunciado y copia los ejemplos en papel.
2. Escribe la solución más simple que pase los ejemplos.
3. Piensa en tres casos límite antes de ejecutar.
4. Si falla, compara salida esperada y real carácter por carácter.

> El botón de solución de referencia existe, pero úsalo solo después de
> intentarlo. Registrar un intento fallido también es progreso.
"#,
        concepts: &["pruebas", "stdin", "casos-ocultos", "depuracion"],
        order_index: 5,
        estimated_minutes: 9,
        questions: &[
            QuestionSeed {
                prompt: "¿Qué se ignora al comparar la salida?",
                options: &[
                    "Las mayúsculas",
                    "Los espacios al final y las líneas vacías finales",
                    "El orden de las líneas",
                    "Nada",
                ],
                correct_index: 1,
                explanation: "Solo se normalizan espacios finales y líneas vacías al final.",
            },
            QuestionSeed {
                prompt: "¿Para qué sirven los casos ocultos?",
                options: &[
                    "Para mostrar la solución",
                    "Para comprobar bordes no visibles en los ejemplos",
                    "Para aumentar el tiempo del ejercicio",
                    "Para reemplazar al compilador",
                ],
                correct_index: 1,
                explanation: "Los casos ocultos cubren situaciones límite no mostradas.",
            },
        ],
    },
];

const EXAMPLES_SUMA: &[ExampleSeed] = &[
    ExampleSeed {
        input: "1 2",
        output: "3",
        explanation: "La suma de 1 y 2 es 3.",
    },
    ExampleSeed {
        input: "10 25",
        output: "35",
        explanation: "Los números pueden tener más de una cifra.",
    },
];

const TESTS_SUMA: &[TestCaseSeed] = &[
    TestCaseSeed {
        name: "ejemplo",
        input: "1 2\n",
        expected_output: "3\n",
        visible: true,
    },
    TestCaseSeed {
        name: "dos-cifras",
        input: "10 25\n",
        expected_output: "35\n",
        visible: true,
    },
    TestCaseSeed {
        name: "cero",
        input: "0 0\n",
        expected_output: "0\n",
        visible: false,
    },
    TestCaseSeed {
        name: "negativos",
        input: "-4 9\n",
        expected_output: "5\n",
        visible: false,
    },
];

const EXAMPLES_SALUDO: &[ExampleSeed] = &[ExampleSeed {
    input: "Ana",
    output: "Hola, Ana!",
    explanation: "El nombre se lee de la entrada.",
}];

const TESTS_SALUDO: &[TestCaseSeed] = &[
    TestCaseSeed {
        name: "ana",
        input: "Ana\n",
        expected_output: "Hola, Ana!\n",
        visible: true,
    },
    TestCaseSeed {
        name: "espacios",
        input: "  Luis  \n",
        expected_output: "Hola, Luis!\n",
        visible: false,
    },
];

const EXAMPLES_PAR: &[ExampleSeed] = &[
    ExampleSeed {
        input: "4",
        output: "par",
        explanation: "4 es divisible entre 2.",
    },
    ExampleSeed {
        input: "7",
        output: "impar",
        explanation: "7 no es divisible entre 2.",
    },
];

const TESTS_PAR: &[TestCaseSeed] = &[
    TestCaseSeed {
        name: "par",
        input: "4\n",
        expected_output: "par\n",
        visible: true,
    },
    TestCaseSeed {
        name: "impar",
        input: "7\n",
        expected_output: "impar\n",
        visible: true,
    },
    TestCaseSeed {
        name: "cero",
        input: "0\n",
        expected_output: "par\n",
        visible: false,
    },
    TestCaseSeed {
        name: "negativo",
        input: "-3\n",
        expected_output: "impar\n",
        visible: false,
    },
];

const EXAMPLES_MAXIMO: &[ExampleSeed] = &[ExampleSeed {
    input: "4 9 2",
    output: "9",
    explanation: "9 es el mayor de los tres números.",
}];

const TESTS_MAXIMO: &[TestCaseSeed] = &[
    TestCaseSeed {
        name: "ejemplo",
        input: "4 9 2\n",
        expected_output: "9\n",
        visible: true,
    },
    TestCaseSeed {
        name: "negativos",
        input: "-1 -5 -2\n",
        expected_output: "-1\n",
        visible: false,
    },
    TestCaseSeed {
        name: "empate",
        input: "7 7 3\n",
        expected_output: "7\n",
        visible: false,
    },
];

const EXAMPLES_INVERTIR: &[ExampleSeed] = &[ExampleSeed {
    input: "hola",
    output: "aloh",
    explanation: "La cadena se lee al revés.",
}];

const TESTS_INVERTIR: &[TestCaseSeed] = &[
    TestCaseSeed {
        name: "hola",
        input: "hola\n",
        expected_output: "aloh\n",
        visible: true,
    },
    TestCaseSeed {
        name: "palindromo",
        input: "radar\n",
        expected_output: "radar\n",
        visible: false,
    },
    TestCaseSeed {
        name: "espacios",
        input: "dos palabras\n",
        expected_output: "sarbalap sod\n",
        visible: false,
    },
];

const EXAMPLES_FRECUENCIAS: &[ExampleSeed] = &[ExampleSeed {
    input: "uno dos uno",
    output: "dos: 1\nuno: 2",
    explanation: "Se ordenan las palabras y se cuenta cada una.",
}];

const TESTS_FRECUENCIAS: &[TestCaseSeed] = &[
    TestCaseSeed {
        name: "ejemplo",
        input: "uno dos uno\n",
        expected_output: "dos: 1\nuno: 2\n",
        visible: true,
    },
    TestCaseSeed {
        name: "una-palabra",
        input: "soledad\n",
        expected_output: "soledad: 1\n",
        visible: false,
    },
    TestCaseSeed {
        name: "repetidos",
        input: "b a b a c\n",
        expected_output: "a: 2\nb: 2\nc: 1\n",
        visible: false,
    },
];

const EXERCISES: &[ExerciseSeed] = &[
    ExerciseSeed {
        path_slug: "csharp-fundamentos",
        lesson_slug: Some("variables-y-tipos"),
        slug: "suma-dos-numeros",
        title: "Suma de dos números",
        difficulty: Difficulty::Easy,
        topic: "entrada y aritmética",
        level: 1,
        concepts: &["stdin", "int", "conversiones"],
        description_md: "Lee dos enteros separados por espacios e imprime su suma.",
        examples: EXAMPLES_SUMA,
        constraints: &[
            "Los enteros caben en `int`.",
            "La entrada puede contener saltos de línea.",
        ],
        hints: &[
            "Usa `Console.In.ReadToEnd()` y divide por espacios.",
            "`int.Parse` convierte cada fragmento a número.",
        ],
        starter_code: "var partes = Console.In.ReadToEnd()\n    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);\n\n// TODO: convierte los dos primeros valores e imprime la suma.\n",
        reference_solution: "var partes = Console.In.ReadToEnd()\n    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);\nvar a = int.Parse(partes[0]);\nvar b = int.Parse(partes[1]);\nConsole.WriteLine(a + b);\n",
        test_cases: TESTS_SUMA,
    },
    ExerciseSeed {
        path_slug: "csharp-fundamentos",
        lesson_slug: Some("variables-y-tipos"),
        slug: "saludo-nombre",
        title: "Saludo personalizado",
        difficulty: Difficulty::Easy,
        topic: "cadenas",
        level: 1,
        concepts: &["strings", "trim", "interpolacion"],
        description_md: "Lee un nombre de la entrada e imprime `Hola, <nombre>!`.",
        examples: EXAMPLES_SALUDO,
        constraints: &["El nombre puede tener espacios alrededor."],
        hints: &[
            "Aplica `Trim()` al texto leído.",
            "Usa una cadena interpolada.",
        ],
        starter_code: "var nombre = Console.In.ReadToEnd().Trim();\n\n// TODO: imprime el saludo.\n",
        reference_solution: "var nombre = Console.In.ReadToEnd().Trim();\nConsole.WriteLine($\"Hola, {nombre}!\");\n",
        test_cases: TESTS_SALUDO,
    },
    ExerciseSeed {
        path_slug: "csharp-fundamentos",
        lesson_slug: Some("control-de-flujo"),
        slug: "numero-par",
        title: "Par o impar",
        difficulty: Difficulty::Easy,
        topic: "condicionales",
        level: 1,
        concepts: &["if", "modulo", "stdin"],
        description_md: "Lee un entero e imprime `par` o `impar` en minúsculas.",
        examples: EXAMPLES_PAR,
        constraints: &["El entero puede ser negativo."],
        hints: &["Comprueba `numero % 2 == 0`."],
        starter_code: "var texto = Console.In.ReadToEnd().Trim();\nvar numero = int.Parse(texto);\n\n// TODO: imprime \"par\" o \"impar\".\n",
        reference_solution: "var texto = Console.In.ReadToEnd().Trim();\nvar numero = int.Parse(texto);\nConsole.WriteLine(numero % 2 == 0 ? \"par\" : \"impar\");\n",
        test_cases: TESTS_PAR,
    },
    ExerciseSeed {
        path_slug: "csharp-fundamentos",
        lesson_slug: Some("metodos-y-colecciones"),
        slug: "maximo-tres",
        title: "Máximo de tres números",
        difficulty: Difficulty::Medium,
        topic: "métodos",
        level: 2,
        concepts: &["metodos", "math", "stdin"],
        description_md: "Lee tres enteros e imprime el mayor. Separa la lectura del cálculo.",
        examples: EXAMPLES_MAXIMO,
        constraints: &["Los tres números pueden ser negativos o repetidos."],
        hints: &[
            "`Math.Max` acepta dos argumentos.",
            "Declara una función local al final.",
        ],
        starter_code: "var datos = Console.In.ReadToEnd()\n    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);\n\n// TODO: convierte los valores y llama a Maximo.\n\nstatic int Maximo(int a, int b, int c)\n{\n    // TODO\n    return 0;\n}\n",
        reference_solution: "var datos = Console.In.ReadToEnd()\n    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);\nvar a = int.Parse(datos[0]);\nvar b = int.Parse(datos[1]);\nvar c = int.Parse(datos[2]);\nConsole.WriteLine(Maximo(a, b, c));\n\nstatic int Maximo(int a, int b, int c) => Math.Max(a, Math.Max(b, c));\n",
        test_cases: TESTS_MAXIMO,
    },
    ExerciseSeed {
        path_slug: "csharp-fundamentos",
        lesson_slug: Some("metodos-y-colecciones"),
        slug: "invertir-cadena",
        title: "Invertir una cadena",
        difficulty: Difficulty::Medium,
        topic: "cadenas y colecciones",
        level: 2,
        concepts: &["strings", "linq", "foreach"],
        description_md: "Lee una línea e imprime sus caracteres en orden inverso.",
        examples: EXAMPLES_INVERTIR,
        constraints: &[
            "Conserva los espacios interiores.",
            "No elimines espacios al final de la línea.",
        ],
        hints: &[
            "`new string(caracteres)` construye una cadena.",
            "`texto.Reverse()` devuelve los caracteres al revés.",
        ],
        starter_code: "var texto = Console.In.ReadToEnd().TrimEnd('\\r', '\\n');\n\n// TODO: imprime el texto invertido.\n",
        reference_solution: "var texto = Console.In.ReadToEnd().TrimEnd('\\r', '\\n');\nvar invertido = new string(texto.Reverse().ToArray());\nConsole.WriteLine(invertido);\n",
        test_cases: TESTS_INVERTIR,
    },
    ExerciseSeed {
        path_slug: "csharp-fundamentos",
        lesson_slug: Some("entradas-y-pruebas"),
        slug: "frecuencias-palabras",
        title: "Frecuencia de palabras",
        difficulty: Difficulty::Hard,
        topic: "diccionarios",
        level: 3,
        concepts: &["diccionarios", "ordenacion", "stdin"],
        description_md: "Lee varias palabras y imprime `palabra: cantidad` en orden alfabético.",
        examples: EXAMPLES_FRECUENCIAS,
        constraints: &[
            "Las palabras se separan por espacios.",
            "La salida debe estar ordenada por palabra.",
        ],
        hints: &[
            "Usa `SortedDictionary<string, int>`.",
            "Recorre las palabras y aumenta el contador.",
        ],
        starter_code: "var palabras = Console.In.ReadToEnd()\n    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);\n\n// TODO: cuenta y ordena las frecuencias.\n",
        reference_solution: "var palabras = Console.In.ReadToEnd()\n    .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);\nvar frecuencias = new SortedDictionary<string, int>();\nforeach (var palabra in palabras)\n{\n    frecuencias.TryGetValue(palabra, out var cantidad);\n    frecuencias[palabra] = cantidad + 1;\n}\nforeach (var par in frecuencias)\n{\n    Console.WriteLine($\"{par.Key}: {par.Value}\");\n}\n",
        test_cases: TESTS_FRECUENCIAS,
    },
];

/// Devuelve el catálogo de C# con lecciones y ejercicios.
pub fn csharp_catalog() -> CatalogSeed {
    CatalogSeed {
        language: LANGUAGE.clone(),
        paths: PATHS,
        lessons: LESSONS,
        exercises: EXERCISES,
    }
}

/// Acceso directo a los ejercicios para las fases de seed.
pub fn csharp_exercises() -> &'static [ExerciseSeed] {
    EXERCISES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lessons_are_ordered_and_have_questions() {
        let catalog = csharp_catalog();
        assert_eq!(catalog.lessons.len(), 5);
        for (index, lesson) in catalog.lessons.iter().enumerate() {
            assert_eq!(lesson.order_index, index as i32 + 1);
            assert_eq!(lesson.questions.len(), 2, "lección {}", lesson.slug);
            for question in lesson.questions {
                assert!(question.correct_index < question.options.len());
            }
        }
    }

    #[test]
    fn exercises_cover_all_difficulties() {
        let exercises = csharp_exercises();
        assert_eq!(exercises.len(), 6);
        assert!(exercises.iter().any(|e| e.difficulty == Difficulty::Easy));
        assert!(exercises.iter().any(|e| e.difficulty == Difficulty::Medium));
        assert!(exercises.iter().any(|e| e.difficulty == Difficulty::Hard));
        for exercise in exercises {
            assert!(!exercise.test_cases.is_empty(), "{}", exercise.slug);
            assert!(
                exercise.test_cases.iter().any(|case| !case.visible),
                "{} necesita casos ocultos",
                exercise.slug
            );
            assert!(!exercise.reference_solution.is_empty());
        }
    }
}
