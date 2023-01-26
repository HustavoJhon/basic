package paquete;

public class Main {
    public static void main(String[] args){
        Thread hilo = new paquete.Proceso("proceso 1");
        Thread hilo2 = new paquete.Proceso("proceso 2");

        hilo.start();
        hilo2.start();
    }
}