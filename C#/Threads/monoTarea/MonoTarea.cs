using System;
using System.Threading;

namespace MonoTarea {
    class Program {
        static void Main(String[] args) {
            Console.WriteLine("primer hilo");
            Thread.Sleep(900);
            Console.WriteLine("segundo hilo");
            Thread.Sleep(50);
            Console.WriteLine("tercer hilo");
            Thread.Sleep(900);
            Console.WriteLine("cuarto hilo");
            Thread.Sleep(50);

            MetodoSaludo();
        }
        static void MetodoSaludo(){

            Console.WriteLine("Buenos Dias");
            Thread.Sleep(900);
            Console.WriteLine("Buenas Tardes");
            Thread.Sleep(50);
            Console.WriteLine("Buenas Noches");
        }
    }
}