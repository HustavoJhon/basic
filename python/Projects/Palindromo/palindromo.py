# es una palabra que se lee igual al derecho o al reves como 
#luz azul

def palindromo(palabra):
    palabra = palabra.replace(' ','') #eliminamos los espacios
    palabra = palabra.lower() #convertimos a minuscula
    palabra_invertida = palabra[::-1]
    if palabra == palabra_invertida:
        return True
    else:
        return False

def run():
    palabra = input("Escribe una palabra: ")
    es_palindromo = palindromo(palabra)
    if es_palindromo == True:
        print('Es un palindromo')
    else:
        print('No es un palindromo')


if __name__ == "__main__":
    run()