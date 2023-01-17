# Juego de Piedra Papel y Tijera

import random


lista = ('piedra', 'papel', 'tijera')
name_user = input("What is your name?: ")

computer_wins = 0
user_wins = 0
rounds = 1

while True:

    print('*' * 10)
    print('ROUND', rounds)
    print('*' * 10)

    print(f'van {computer_wins} | {user_wins}')

    me = input('Escoje: ')
    me = me.lower()

    rounds+=1

    if not me in lista:
        print('esa no es una opcion correcta')
        continue

    pc = random.choice(lista)
    user = name_user.upper()
    print(user ,me) 
    print('COMPUTER: ', pc+'\n')

    if me == pc:
        print('EMPATE')

        # piedra > tijera
    elif me == 'piedra':
        if pc == 'tijera':
            print('Piedra gana a tijera')
            print(f'{name_user} ganaste!!!')
            user_wins+=1
        else:
            print('Papel > Piedra')
            print('computer gano')
            computer_wins+=1

    # Papel > piedra
    elif me == 'papel':
        if pc == 'piedra':
            print('Papel > Piedra')
            print(f'{name_user} ganaste!!!')
            user_wins+=1
        else:
            print('Tijera > Papel')
            print('computer gano')
            computer_wins+=1

    # Tijera > papel
    elif me == 'tijera':
        if pc == 'papel':
            print('Tijera > Papel')
            print(f'{name_user} ganaste!!!')
            user_wins+=1
        else:
            print('Piedra > Tijera')
            print('computer gano')
            computer_wins+=1

    # intentos
    if computer_wins == 2:
        print('El ganador es la computadora')
        break
    if user_wins == 2:
        print(f'El ganador es {name_user}')
