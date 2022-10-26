import random


def adivina_computadora(x):
    print("Bienvenido")
    print(f"Selecciona un numero entre 1 y {x} para que la computadora intente adivinar")

    limite_inferior = 1
    limite_superior = x

    respuesta = ""

    while respuesta != "c":
        # Generar prediccion

        if limite_inferior != limite_superior:
            prediccion = random.randint(limite_inferior, limite_superior)
        else:
            prediccion = limite_inferior # tambien podria ser el limite superior

        #Obtener respuesta del usurio
        respuesta = input(f"""Mi prediccion es {prediccion}.
        > Si es muy alta, ingresa (A). 
        > SI es muy baja, ingresa (B).
        > Si es correcta (C)
        |-> """).lower()

        if respuesta == "a":
            limite_superior = prediccion - 1 
        elif respuesta == "b":
            limite_inferior = prediccion - 1

        #* Intervalo inicial: [1, 10]
        #* Prediccion: 6
        #* Respuesta: "a"
        #* Intervalo: [1, 5]

    
    print(f"SIII! la computadora adivino tu numero correctamente: {prediccion}")


adivina_computadora(10)