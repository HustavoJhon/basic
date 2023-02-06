import os


user_name = input(""" 
           *             +    |
       ()    .-.,="``"=.    - o -
             '=/_       \     |
     |    *   |  '=._    |
   - O -       \     `=./`,        '
     |      .   '=.__.=' `='      *
   +                         +
        O      *        '       .
Bienvenido, Como te llamas?: """)

def option_planet():
    print("""
=====| PLANETAS |===== | =====| LUNAS |=====
    1. Mercurio        |    9. La Luna 
    2. Venus           |    10. IO
    3. Marte           |    11. EUROPA
    4. Jupiter         |    12. Ganymede
    5. Saturno         |    13. Callisto
    6. Urano           | 
    7. Neptuno         | =====|  SOL  |===== 
    8. Pluton          |    14. Sol

Escoje una uno de estos planeta en los que quieres saber cuanto pesas: """)


def calcular_peso(peso):
    # Gravedad(m/s^2)
    MERCURIO = 3.7
    VENUS = 8,87
    TIERRA = 9.807
    MARTE = 3.721
    JUTIPER = 24.79
    SATURNO = 10.44
    URANO  = 8.87
    NEPTUNO = 11.15
    PLUTON = 0.62

    
    LUNA = 1.62
    EUROPA = 1.315
    PHOBOS = 0.0057

    SOL = 274

    while True:
        print(option_planet())
        user = int(input(">>> "))
        if user == 1:
            peso_mercurio = round((peso*MERCURIO/TIERRA),2)
            print(f"Ok {user_name} en Mercurio pesas: {peso_mercurio} kg")
            space = input()
            os.system("clear")
        elif user == 2:
            peso_venus = round((peso*VENUS/TIERRA),2)
            print(f"Ok {user_name} en Venus pesas: {peso_venus} kg") 
            space = input()
            os.system("clear")
        elif user == 3:
            peso_marte = round((peso*MARTE/TIERRA),2)
            print(f"Ok {user_name} en Marte pesas: {peso_marte} kg")
            space = input()
            os.system("clear")
        elif user == 4:
            peso_jupiter = round((peso*JUTIPER/TIERRA),2)
            print(f"OK {user_name} en Jupiter pesas: {peso_jupiter}")
            space = input()
            os.system("clear")
        else: 
            print("Gracias por usar el  programa <3")
            break

def run():
    calcular_peso(my_peso)

if __name__ == "__main__":
    my_peso = int(input("Cuanto Pesas?: "))
    run()
