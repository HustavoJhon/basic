void main() {
  String hola = saludar('Fernando', 'Hola,');
  // asignacion con llaves
  // String hola = saludar(nombre: 'Lucas', texto: "hola,");
  print(hola);

  var mensaje = nombre();
  print(mensaje);
}

String nombre() {
  return "Lucas";
}

// si la funcion no retorna nada le colocamos void
String saludar(String texto, String nombre) {
  return '$texto $nombre';
}
