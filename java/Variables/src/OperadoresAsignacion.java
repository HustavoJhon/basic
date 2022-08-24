public class OperadoresAsignacion {
    public static void main(String[] args) {
        int lives = 5;
        lives = lives - 1;
        System.out.println(lives);
        //decremento
        lives--;
        System.out.println(lives);
        //incremento
        lives++;
        System.out.println(lives);


        //Prefija
        //Gana un regalo por ganar una vida
        int gift = 100 + lives++; //posfijo
        System.out.println(gift);
        System.out.println(lives);

    }
}
