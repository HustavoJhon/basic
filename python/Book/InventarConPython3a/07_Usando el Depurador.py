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

print('Lanzare una moneda 1000 veces. Adibina cuantas veces caera Cara.(Presiona enter para comensar)')
input()
lanzamientos = 0
caraz = 0 

while lanzamientos < 1000:
    if random.randint(0, 1) == 1:
        caras = caras + 1 
    lanzamientos('900 lanzamientos y hubo ' + str(caras)+ ' caras')
    if lanzamientos == 100:
        print('En 100 lanzamientos,cara salio ' + str(caras)+' veces.')
    if lanzamientos == 500:
        print('La midatad de los lanzamientos y salio' + str(catas)+' veces.')
        
print()

print('De 1000 lanzamientos, al final carca '+str(caras)+'veces')
print('Estube cerca')
