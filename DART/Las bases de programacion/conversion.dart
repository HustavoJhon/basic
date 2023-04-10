void main() {
  String texto = "12";
  String texto2 = "5.5";

  int numero = int.parse(texto);
  double numero2 = double.parse(texto2);

  print(numero);
  print(numero2);

  // SCRIPT ERROR
  String texto3 = "435.345";
  // int numero3 = int.parse(texto3);
  // print(numero3);

  // To String
  String texto4 = texto3.toString();
  print(texto4);
}
