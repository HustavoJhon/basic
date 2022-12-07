import random

def choice(a):
    if a == 1:
        return ('Piedra')
    elif a == 2:
        return ('Papel')
    elif a == 3:
        return ('Tijeras')
    else:
        return ('Perder')

def tex():
    print(f'\nHas Elegido: {choice(usuario)}')
    print(f'El Pc ha elegido: {choice(pc)}')

print('\njuego de piedra, papel o tijera \n')
usuario = int(input('ingrese: \n1 para Piedra \n2para Papel \n3 para Tijera =>'))
pc =  random.randint(1,3)
if pc == usuario:
    tex()
    print('Empate')
elif (usuario == 1 and pc == 3) or (usuario == 2 and pc == 1) or (usuario == 3 and pc == 2):
    tex()
    print('Has ganado')
elif usuario not in range(1, 3):
    tex()
    print('Elegiste perder')
else:
    tex()
    print('Has perdido')
