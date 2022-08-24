public class Casting {
    public static void main(String[] args) {
        //En un año ubicar 30 perritos
        //Cuantos perritos ubique al mes

        double monthlyDogs = 30.0/12.0;
        System.out.println(monthlyDogs);


        //Estimacion
        int estimatedMonthlyDogs = (int) monthlyDogs;
        System.out.println(estimatedMonthlyDogs);

        //Exactitud
        int dias = 30;
        int mes = 12;
        System.out.println(dias/mes);
        System.out.println((double) dias/mes);


        //Casteo entre tipo de datos
        double c = dias/mes;
        System.out.println(c);
        
        char n = '1';
        int nI = n;
        System.out.println(nI);

        short nS = (short) n;
        System.out.println(nS);
    }
}
