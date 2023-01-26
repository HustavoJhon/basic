import random

def run():
    numero_aleatorio = random.randint(1,10)
    numero_elegido = int(input('Elige un numero del 1 al 10: '))
    while numero_elegido != numero_aleatorio:
        if numero_elegido < numero_aleatorio:
            print('Busca un numero mas grande')
        else:
            print('Busca un numero menor')
        numero_elegido = int(input('Elegige otro numero: '))
    print('Ganaste')

if __name__ == '__main__':
    run()