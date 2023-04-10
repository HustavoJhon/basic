void main() {
  //  COMENTARIOS
  // String nombre = "Amanda";
  String nombre = "jose";
  String pais = "brasil";
  // CONCATENACION
  String texto1 = "Soy " + nombre + " y vivo en " + pais;
  print(texto1);

  // INTERPOLACION
  String text2 = "soy $nombre y vivo en $pais";
  print(text2);

  // CARACTERRES ESPECIALES
  String text3 = "soy \"$nombre\" y \n vivo en $pais";
  print(text3);

  // FUNCIONES
  String palabra1 = "Lukaku";
  print(palabra1.toLowerCase());
  print(palabra1.toUpperCase());
  print(palabra1.replaceAll("Lukaku", "Smith"));
}
