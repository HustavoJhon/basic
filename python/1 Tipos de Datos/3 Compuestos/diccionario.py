def run():
    mi_diccionario = {
        'llave1' : 1,
        'llave2' : 2,
        'llave3' : 3
    }
    print(mi_diccionario)
    print(mi_diccionario['llave1'])

    poblacion_paises = {
        'Argentina' : 449_387_123,
        'Brasil' : 546_456_544,
        'Colombia' : 456_546_434
    }
    print(poblacion_paises['Argentina'])

    for pais in poblacion_paises.keys():
        print(pais)
    
    for value in poblacion_paises.values():
        print(value)

    for pais,poblacion in poblacion_paises.items():
        print(f'{pais} tiene {poblacion} habitantes')

if __name__ == '__main__':
    run()