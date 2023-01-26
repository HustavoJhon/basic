# Capitulo 7 "Usando el Depurador"
#77 

#Encuentra el Bug 
'''
import random 

numero1 = random.randint(1, 10)
numero2 = random.randint(1, 10)

print('Cuanto es ' + str(numero1) + ' + ' + str(numero2) + '?')
respuesta = int(input('>>>: '))
if respuesta == numero1 + numero2:
    print('Correcto')
else: 
    print('Nops la respuesta es ' + str(numero1+numero2))
'''

import random

print('Lanzare una moneda 1000 veces. Adibina cuantas veces caera Cara.\n(Presiona enter para comenzar)')
input()

lanzamientos = 0
caras = 0 

while lanzamientos < 1000:
    if random.randint(0, 1) == 1:
        caras += 1 
    lanzamientos += 1
    if lanzamientos == 100:
        print(f'En 100 lanzamientos > {caras} caras.')
    if lanzamientos == 500:
        print(f'En 500 lanzamientos >  {caras} caras.')
    if lanzamientos == 900:
        print(f'En 900 lanzamientos > {caras} caras.')
        
print()

print(f'En 1000 lanzamientos > {caras} caras')
print('Estuviste cerca?')
