#include <iostream>
#include <string>
using namespace std;

int main() {
  // Tipos numericos numericos
  int edad = 21;              // Enteros con signo
  unsigned int puntos = 1500; // Entero sin signo (solo valores positivos)
  short dias = 7;             // Entero corto
  long poblacion = 800000000; // Enteroo largo

  // Tipos numericos decimales
  float temperatura = 36.6f;    // Decimal con menor precision
  double altura = 1.75;         // Decimal con mayor precision
  long double pi = 3.141624524; // Decimal de alta precision

  // Caracter y cadena
  char inicial = 'M';      // Un solo caracter
  string nombre = "Maria"; // Cadena de texto (requiere #innclude <string>)

  // Booleano
  bool estudiante = true; // treu o false

  // Mostrar los valores
  cout << "Nombre: " << nombre << endl;
  cout << "Inical: " << inicial << endl;
  cout << "Edad: " << edad << endl;
  cout << "Altura: " << altura << endl;
  cout << "Temperatura corporal: " << temperatura << endl;
  cout << "Puntos Acumulados: " << puntos << endl;
  cout << "Estudainte: " << (estudiante ? "SI" : "NO") << endl;
  cout << "Poblacion estimada: " << poblacion << endl;
  cout << "Dias de la semana: " << dias << endl;
  cout << "Valores de PI: " << pi << endl;

  return 0;
}
