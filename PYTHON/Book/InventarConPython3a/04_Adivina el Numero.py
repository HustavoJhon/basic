# Capitulo 4 "ADIVINA EL NUMERO" 
#25

import random 

intentosRealizados = 0

numero = random.randint(1, 20)

name = input('Como te llamas?: ')
print(f'Bueno, {name}, estoy pensando en un numero entre 1 y 20')

print("===========")

while intentosRealizados < 6:
    estimacion = int(input('En que numero estoy pensando? : '))
    print(f'intentos {intentosRealizados}')

    intentosRealizados+=1

    if estimacion < numero:
        print('Tu etimacion es muy baja.')
    elif estimacion > numero:
        print('Tu estimacion es muy alta.')
    elif estimacion == numero:
        break
if estimacion == numero:
    print(f'Buen trabajo {name}! Has adivinado mi numero en {intentosRealizados} intentos!')

if estimacion != numero:
    print(f'Pues no. El numero que estaba pensando era {numero}')
        
