import random


def adivina(x):
    print("==============")
    print("::Bienvenido::")
    print("==============")
    print("Tu meta es adivinar el numero generado por la computadora.")
    n_aleatorio = random.randint(1, x) # Numero aleatorio entre 1 y X.
    prediccion =  0

    while prediccion != n_aleatorio:
        #El usuario ingresa un numero
        prediccion = int(input(f"Adivina un numero entre 1 y {x}: ")) # f-string

        if prediccion < n_aleatorio:
            print("Intenta otra vez. Este numero es menor")
        elif prediccion > n_aleatorio:
            print("Intenta otra vez. Este numero es mayor")
        
    print(f"Felicitaciones Adivinaste el numero {n_aleatorio} correctamente")
            

adivina(10)