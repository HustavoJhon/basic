/* La estación de trenes de la Ciudad Paleta requiere un sistema que despliegue cierta información en las consolas de los operarios todos los días en la mañana. La información le sirve tanto a los maquinistas como a los vendedores para tomar decisiones de cuántos boletos vender o cuál será la velocidad máxima del tren.

Realiza el programa en el lenguaje de programación Dart con el siguiente resultado:

Buenos días, trabajadores de TRENES CIUDAD PALETA!!!

El día de hoy la temperatura es de 27 grados
centígrados u 80,6 grados Fahrenheit.
El tren se detendrá en las siguientes ciudades: [Plateada, Celeste, Carmín, Azulona]

Los valores del programa serán cambiados todas las mañanas mediante la manipulación del código, pero se requiere lo siguiente:

El nombre de la estación de trenes puede cambiar, pero siempre debe mostrarse en mayúscula.
La temperatura en grados centígrados siempre será entera, pero Fahrenheit puede ser entera o decimal.
La lista de ciudades puede tener 1 o más nombres (cambiar diariamente).
Dentro del código deben estar todas las instrucciones, a modo de comentarios, necesarias para que otro programador sepa cuáles valores cambiar todos los días.

*/

void main() {
  // Nombre de la estación
  String estacion = "trenes ciudad paleta";
  // Convertimos a mayúscula el nombr de la estación
  estacion = estacion.toUpperCase();
  print("Buenos días, trabajadores de $estacion!!!");

  // Declaramos los grados y las ciudades
  int grados = 1;
  double fahrenheit = (grados * 9 / 5) + 32;
  List<String> ciudades = ["Plateada", "Celeste", "Carmín", "Azulona"];

  // Construimos el mensaje
  String result = """El día de hoy la temperatura es de $grados grados
  centigrados u $fahrenheit grados Fahrenheit.
  El tren se detendrá en las siguientes ciudades: $ciudades""";

  // Mostramos el mensaje
  print(result);
}
