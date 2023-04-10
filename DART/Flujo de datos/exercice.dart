void main() {
  int valor1 = 2;
  int valor2 = 4;
  List<int> valores = [2, 3, 6];
  dynamic resultado = 0;
  Operacion operacion = Operacion.producto;

  switch (operacion) {
    case Operacion.suma:
      resultado = valor1 + valor2;
      print(resultado);
      break;
    case Operacion.resta:
      resultado = valor1 - valor2;
      if (resultado < 0) {
        print("NEGATIVO");
      }
      break;
    case Operacion.multiplicacion:
      resultado = valor1 * valor2;
      break;
    case Operacion.modulo:
      resultado = valor1 % valor2;
      break;
    case Operacion.factorial:
      resultado = 1;
      for (int i = valor1; i >= 1; i--) {
        resultado = resultado * i;
      }
      break;
    case Operacion.sumatoria:
      for (int i = 0; i < valores.length; i++) {
        resultado += valores[i];
      }
      break;
    case Operacion.producto:
      resultado = 1;
      for (int i = 0; i < valores.length; i++) {
        resultado *= valores[i];
      }
      break;
    default:
      print("Elige una operación válida");
  }
  print("El resultado de la operación es $resultado");
}

enum Operacion {
  suma,
  resta,
  multiplicacion,
  modulo,
  factorial,
  sumatoria,
  producto
}
