using System;
using System.Threading;

namespace MultiTarea {
    class Program {
        static void Main(String[] args) {

            Thread t = new Thread(MetodoSaludo);
            t.Start();

            Thread t2 = new Thread(MetodoSaludo);
            t2.Start();

            Console.WriteLine("primer hilo");
            Thread.Sleep(900);
            Console.WriteLine("segundo hilo");
            Thread.Sleep(50);
            Console.WriteLine("tercer hilo");
            Thread.Sleep(900);
            Console.WriteLine("cuarto hilo");
            Thread.Sleep(50);

            // MetodoSaludo();
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