void main() {
  Tiempo tiempo = Tiempo.Soleado;

  switch (tiempo) {
    case Tiempo.Soleado:
      print("El timpo esta Soleado");
      break;
    case Tiempo.Lluvioso:
      print("El tiempo esta LLuvioso");
      break;
    case Tiempo.Despejado:
      print("El tiempo esta Despejado");
      break;
  }
}

enum Tiempo { Soleado, Lluvioso, Despejado }
