void main() {
  List<String> coloresPrimarios = [
    "rojo",
    "amarillo",
    "azul",
  ];

  List<String> colores = [
    "morado",
    "verde",
    for (int i = 0; i < coloresPrimarios.length; i++) coloresPrimarios[i],
  ];
  print(coloresPrimarios);
  print(colores);
}
