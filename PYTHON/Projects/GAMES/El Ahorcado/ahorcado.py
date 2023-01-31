import random 

import string

from palabras import palabras #importamos el modulo palabras con la variable palabras
from ahorcado_diagramas import vidas_diccionario_visual

def obtener_palabra_valida(palabra):
    #Seleccionar una palabra al azar de la lista de palabras validas
    palabra = random.choice(palabra)

    while '-' in palabra or ' ' in palabra:
        palabra = random.choice(palabra)
        
    return palabra.upper()


def ahorcado():
    print("*" * 10)
    print("Bienvenido a el juego del Ahorcado")
    print("*"*10)

    palabra = obtener_palabra_valida(palabras)

    letras_por_adivinar = set(palabra) #con el conjunto hacermos que no se repitan las letras ingresadas 
    letras_adivinadas = set()
    abecedario = set(string.ascii_uppercase) #esto nos va a retornar una lista con todas las letras del abecedario en mayuscula 
    
    vidas = 7
    
    
    while len(letras_por_adivinar) > 0 and vidas > 0:
        print(f"Te quedan {vidas} vidas y has usado estas letras: {' '.join(letras_adivinadas)}") # con join une todos los elementos de un conjunto o secuencia con el caracter que especificamos (' ') -> espacio vacio
        # ' '.join({'A','B','C'}) -> 'A B C'
        
        
        #Mostrar el estado actual de la palabra
        palabra_lista = [letra if letra in letras_adivinadas else '-' for letra in palabra] #list comprehension
        
        #mostrar estado del ahorcado
        print(vidas_diccionario_visual[vidas])

        #mostrar las letras separadas por un espacio
        print(f"Palabra: {' '.join(palabra_lista)}")

        
        letra_usuario = input("Escoje una letra: ").upper()
        
        #Si la letra escojida por el usuario esta en el abecedario y no esta en el conjunto de letras que ya se han ingredado, se aniaden la letra al conjunto de letras ingresadas.
        if letra_usuario in abecedario - letras_adivinadas:
            letras_adivinadas.add(letra_usuario)
            
            #si letra esta en la palabra, quitar la letra del conjunto de letras pendientes por adivinar.
            if letra_usuario in letras_por_adivinar:
                letras_por_adivinar.remove(letra_usuario)
                print('')
            #Si no esta en la palabra, quitar una vida
            else:
                vidas -= 1
                print(f"\nTu letra, {letra_usuario} no esta en la palabra secreta")
        
        #si la letra escojida por el usuario ya fue ingresada.
        elif letra_usuario in letras_adivinadas:
            print("\nYa escojiste esa letra. Por favor escoje una letra nueva.")
        else:
            print("\nEste letra no es valida.")
    
    # El juego llega a esta linea cuando se adivinan todas las letras de la palabra o cuando se agotan las vidas del jugador
    if vidas == 0:
        print(vidas_diccionario_visual[vidas])
        print(f"Ahorcado! Perdiste. Lo lamento mucho. La palabra era: {palabra}")
    else:
        print(f"Excelente! Adivinaste la palabra {palabra}")        
        
ahorcado()