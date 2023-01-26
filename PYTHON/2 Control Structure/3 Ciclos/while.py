# contador = 0
# print('2 elevado a ' + str(contador) + ' es igual a: ' + str(2**contador))

# Uso de While

def run():
    # cuando esta en mayuscula == constante
    LIMITE = 1000

    contador = 0
    potencia_2 = 2**contador
    
    while potencia_2 < LIMITE:
        print(f'2 elevado a {contador} es igual a:  + {potencia_2}')

        contador = contador + 1
        potencia_2 = 2**contador


if __name__ == '__main__':
    run()


HOLA = 'hola'
HOLA = 'hi'
print(HOLA)
