using System;

namespace Singleton
{   
    class Program {
        static void Main(String[] args) {

            Console.WriteLine(Singleton.Instance.mensaje);
            Singleton.Instance.mensaje = "hola Marte";
            Console.WriteLine(Singleton.Instance.mensaje);

        }
    }
}