# Caitulo 6 "REINO DE DRAGONES"
#48
'''Temas tratados en este capitulo
- La funcion rime.sleep()
- Creando nuestras propias funciones con la palabra reservada def
- La palabra reservada return
- Los operadores Booleanos and, or y not
- Tablas de verdad
- Entorno de variables(Global y Local)
- Parametros y Argumentos
- Diagrama de flujo'''

import random
import time

def mostrarIntroducion():
    print('''
        Estas en una tierra llena de dragones. Frente a ti\n
        hay dos cuevas. En una de ellas, el dragon es generoso y\n
        amigable y compartira su tesoro conrigo. EL otro dragon\n
        es codicioso y esta hambriento, y te devorara inmediatemente.\n
    ''')

def eligirCueva():
    cueva = ''
    while cueva != '1' and cueva != '2':
        print('Aque cueva quieres entrar? (1, o 2)')
        cueva = input('Elige: ')
    return cueva 

def explorarCueva(cuevaElegida):
    print('Te aproximas a la cueva...')
    time.sleep(2)
    print('Es oscura y espeluznante..')
    time.sleep(2)
    print('Un gran dragon aparace subitamente frente a ti! Abre sus fauces..')
    print()
    time.sleep(2)

    cuevaAmigable = random.randint(1,2)

    if cuevaElegida == str(cuevaAmigable):
        print('Te regala su tesoro')
    else:
        print('Te angulle de un bocado!')

jugarDeNuevo = 'si'

while jugarDeNuevo == 'si' or jugarDeNuevo =='s':
    mostrarIntroducion()
    numeroDeCueva = eligirCueva()
    explorarCueva(numeroDeCueva)
    print('Quieres jugar de nuevo? (si o no)')
    jugarDeNuevo = input("Escoje: ")
