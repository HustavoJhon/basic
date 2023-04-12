void main() {
  List<String> coloresPrimarios = [
    "rojo",
    "Amarillo",
    "azul",
  ];

  List<String> coloresSecundarios = ["morado", "verde", "naranja"];

  coloresSecundarios.addAll(coloresPrimarios);

  print(coloresSecundarios);
  print(coloresPrimarios);

  // or

  List<String> coloresSecondari = [
    "morado",
    "verde",
    "naranja",
    ...coloresPrimarios
  ];

  print(coloresSecondari);

  // or

  List<String> coloresC = [...coloresSecundarios, ...coloresPrimarios];
  print(coloresC);
}
