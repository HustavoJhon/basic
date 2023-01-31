# Variables

## Tener en cuenta:

+ Todo archivo de GO tiene como extensión: `<nombre-archivo>.go`

+ Comando para ejecutar código en el terminal: `go run <nombre-archivo>.go`

### **Declarar y asignar un valor a una variable**

La sintaxis para declarar una variable es `var nombre-de-la-variable tipo-de-variable` y para asignar un valor es `nombre-de-la-variable = valor`

```go
// Declarando una variable (tipo string) llamada name
var name string

// Asignado el valor "Pepe" a name
name = "Pepe"

// Imprimir en consola el valor de name
fmt.Println(name)
```

+ **Nota:** En Go, __toda variable declarada debe usarse__ si no se usa, al ejecutar el programa producirá un error.

  + Se puede declarar y asignar una o más variables en una sola línea.
  
  + No es necesario especificar el tipo de la variable (tipado dinámico).
  
  + Para reasignar una variable, el nuevo valor debe ser del mismo tipo de la variable declarada anteriormente (tipado estático).
### Operador de variable corta (:=)
+ Permite declarar y asignar variables en una línea de manera más sencilla.

+ **Nota:** No se puede usar el operador := dos veces para una misma variable, porque se estaría declarando 2 veces la misma y esto producirá un error.

+ Excepto cuando se declare al menos una variable nueva en la misma línea.