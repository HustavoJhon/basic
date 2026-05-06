/*
 * Operaciones aritmeticas
 *   + suma
 *   - resta
 *   * multiplicacion
 *   / division
 *   % modulo
 *
 * Operaciones relacionales:
 *   == igual a
 *   != diferente de
 *   > mayor que
 *   < menor que
 *   >= mayor igual que
 *   <= menor igual que
 *
 * Operadores Logicos
 *   && and
 *   || or
 *   !  not
 */

#include <iostream>
using namespace std;

int main() {
  int a = 10, b = 5;
  cout << a + b << endl;             // suma
  cout << (a > b) << endl;           // a es mayor que b : 1 (true)
  cout << (a > 5 && b < 10) << endl; // ambas condiciones verdaderas : 1 (true)
}
