void main() {
  Map restaurante = {
    "nombre": "Pollos de monte",
    "estrellas": [5, 4, 3, 4, 2]
  };
  print(restaurante);

  if (restaurante["estrellas"] == null) {
    print("El restaurate no tiene estrellas");
  } else {
    print("El restaurante si tiene estrelllas");
    List<int> estrellas = restaurante["estrellas"];
    print(estrellas);
    int suma = 0;
    for (int i = 0; i < estrellas.length; i++) {
      suma += estrellas[i];
    }
    double promedio = suma / estrellas.length;
    restaurante.addAll({"promedio": promedio});
    print(promedio);
  }
  print(restaurante);
}
