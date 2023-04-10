import 'dart:math';

void main() {
  int tazasDeCafe = 0;
  int maximo = 3;

  // WHILE
  while (tazasDeCafe < maximo) {
    tazasDeCafe++;
    print("cantidad de tazas: $tazasDeCafe");
  }

  // DO WHILE
  do {
    tazasDeCafe++;
    print("Cantidad de tazas de cafe: $tazasDeCafe");
  } while (tazasDeCafe < maximo);

  // FOR

  for (int i = 0; i < maximo; i++) {
    print("Iteracion: $i");
    tazasDeCafe = i + 1;
    print("Cantidad de tazas de cafe: $tazasDeCafe");
  }
}
