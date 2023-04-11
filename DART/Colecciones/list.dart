void main() {
  // lista de amigos
  List<String> amigos = [];
  amigos = ["Amanda", "Beto", "Carmen"];
  print(amigos);

  print(amigos.isEmpty);
  print(amigos.length);
  amigos.add("Amanda");
  amigos.remove("Beto");
  amigos.insert(1, "Damian");
  // amigos.clear();
  print(amigos);

  for (int i = 0; i < amigos.length; i++) {
    print("amigos[$i]: ${amigos[i]}");
  }

  if (amigos.isEmpty) {
    print("No hay amigos");
  }
}
