void main() {
  bool agregarAmarillo = true;
  bool agregarAzul = true;

  List<String> colores = [
    "rojo",
    "verde",
    if (agregarAmarillo) "amarillo",
    if (agregarAzul) "azul",
  ];
  print(colores);
}
