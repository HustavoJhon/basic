public class Main {
    public static void main(String[] args) {
        boolean isBluetoothEnabled = false;
        int fileSended = 3;

        if (isBluetoothEnabled) {
            //Send file
            fileSended++;
            System.out.println("Archibo Enviado");
        } else {
            fileSended--;
            System.out.println("Por favor enciende tu bluetooth, para iniciar la transferencia");
        }
    }
}