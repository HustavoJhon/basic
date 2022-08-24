public class DataTypes {
    public static void main(String[] args) {
        //Number > byte, short, int, long

        //Punto Flotante > float, double

        //Caracteres > char (un solo digito)

        //Logico > Boolean (true or false)

        //tambien se puede usar var para crear una variable desde la version 10
        var salary = 1000; //int
        var pension = salary * 0.03; //double
        var totalSalary = salary - pension;

        System.out.println(pension);
        System.out.println(totalSalary);

        var employeeName = "Analy Salgado";
        System.out.println("Employye: " + employeeName + " Salry " + totalSalary);
    }
}
