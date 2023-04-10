void main() {
  int valor1 = 7;
  int valor2 = 3;

  // ARITMETICA
  // SUM
  int resultado = valor1 + valor2;
  print("$valor1 + $valor2 = $resultado");

  // DIVISION
  double resultado1 = valor1 / valor2;
  print("$valor1 / $valor2 = $resultado1");

  // MULTIPLICACION
  int resultado2 = valor1 * valor2;
  print("$valor1 * $valor2 = $resultado2");

  // RESULTADO ENTERO DE UNA DIVISION
  int resultado3 = valor1 ~/ valor2;
  print("$valor1 ~/ $valor2 = $resultado3");

  // MODULO
  int resultado4 = valor1 % valor2;
  print("$valor1 % $valor2 = $resultado4");

  // ASIGNACION
  print("$valor1 += $valor2");
  valor1 += valor2;
  print("$valor1");

  print("$valor1 -= $valor2");
  valor1 -= valor2;
  print("$valor1");

  print("$valor1 *= $valor2");
  valor1 *= valor2;
  print("$valor1");

  print("$valor1 /= $valor2");
  double n1 = 7;
  double n2 = 3;
  n1 /= n2;
  print("$n1");

  // RELACIONALES
  String texto1 = "Hola";
  String texto2 = "Adios";
  int v1 = 1;
  int v2 = 2;

  bool resultado5 = texto1 == texto2;
  print("$texto1 == $texto2 = $resultado5");

  bool resultado6 = texto1 != texto2;
  print("$texto1 != $texto2 = $resultado6");

  bool resultado7 = v1 >= v2;
  print("$v1 >= $v2 = $resultado7");

  bool resultado8 = v1 <= v2;
  print("$v1 <= $v2 = $resultado8");

  bool resultado9 = v1 > v2;
  print("$v1 > $v2 = $resultado9");

  bool resultado10 = v1 < v2;
  print("$v1 < $v2 = $resultado10");

  // LOGICAS

  //Operaciones Lógicas
  bool llueve = true;
  bool haceFrio = false;

  //Operación Or: es falsa solo cuando ambas condiciones son falsas, el resto de los casos es verdadera
  bool llevoAbrigo = llueve || haceFrio;
  print("$llueve || $haceFrio == $llevoAbrigo");

  //Operador And: Es verdadero solo si ambas variables son verdaderas, el resto de los casos es falsa
  bool llevoAbrigoAND = llueve && haceFrio;
  print("$llueve && $haceFrio == $llevoAbrigoAND");

  //Operador Ternario ?: Funciona como un if de forma abreviada
  //Operador Negación !: Nos devuelve el valor opuesto de la variable, si es true nos devuelve false y viceversa.
  print(!llueve);
}
