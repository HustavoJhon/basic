// La información es almacenada en bases de datos no relaciones y puede ser extraído en la siguiente estructura mediante el lenguaje de programación Dart.

void main() {
  Set restaurantes = {
    {
      "nombre": "El novillo alegre",
      "tipo": "Argentina",
      "calificaciones": [4, 5, 2, 1, 2]
    },
    {
      "nombre": "Baires Grill",
      "tipo": "Argentina",
      "calificaciones": [3, 1, 3, 5, 5, 2, 3]
    },
    {
      "nombre": "Mario Brothers",
      "tipo": "Italiana",
      "calificaciones": [4, 3, 2, 1, 1]
    },
    {
      "nombre": "Molto bene",
      "tipo": "Italiana",
      "calificaciones": [4, 3, 1, 1, 5]
    }
  };

// Un conjunto de restaurantes que no se puede repetir, que contiene varios mapas con nombre, tipo y una lista con calificaciones.

// Google requiere que un programador realice un código en Dart para obtener un resumen de la calificación de los diferentes tipos de restaurantes de una zona. Realizando un promedio de las calificaciones por tipo, el resultado debe ser el siguiente.

// Map rating = {
// “Italiana”: 3.0,
// “Argentina”: 2.5,
// “Todos”: 2.77
// };

// Los tipos de comida pueden variar, al igual que las cantidades de la lista de calificación.

// ¡Comparte tu código!
//Declaro las variables necesarias para calcular los promedios según sera restaurante Argentino o Italiano
  int sumaArg = 0;
  int longArgCal = 0;
  double avgArg = 0;

  int sumaIt = 0;
  int longItCal = 0;
  double avgIt = 0;

  double avgTotal = 0;

  Map<String, dynamic> rating = {};

  //Ejecuto un ciclo for para obtener los datos de cada Restaurant de forma individual.
  //Luego se analiza el tipo de restaurant y se separan los cálculos según cada tipo
  //Se suman los valores de la lista de calificaciones y se suman las longitudes de las listas según tipo de restaurant
  for (int i = 0; i < restaurantes.length; i++) {
    Map<String, dynamic> restaurante = restaurantes.elementAt(i);
    if (restaurante["tipo"] == "Argentina") {
      List<int> calificaciones = restaurante["calificaciones"];
      longArgCal += calificaciones.length;

      for (int i = 0; i < calificaciones.length; i++) {
        sumaArg += calificaciones[i];
      }
    } else if (restaurante["tipo"] == "Italiana") {
      List<int> calificacionesIt = restaurante["calificaciones"];
      longItCal += calificacionesIt.length;

      for (int i = 0; i < calificacionesIt.length; i++) {
        sumaIt += calificacionesIt[i];
      }
    }
  }

  //Se calculan los promedios de cada Resto y el total
  avgArg = sumaArg / longArgCal;
  avgIt = sumaIt / longItCal;
  avgTotal = avgArg + avgIt / 2;

  //Se cargan los datos al Conjunto rating para imprimir en pantalla los resultados
  rating.addAll({"Italiana": avgIt, "Argentina": avgArg, "Todos": avgTotal});

  print(rating);
}
