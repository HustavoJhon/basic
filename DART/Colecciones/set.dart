void main() {
  // Set de paises

  Set<String> paises = {};
  paises = {"Argentina", "Brasil", "Colombia"};
  print(paises);

  paises.add("Mexico");
  print(paises);

  for (int i = 0; i < paises.length; i++) {
    print(paises.elementAt(i));
  }
}
