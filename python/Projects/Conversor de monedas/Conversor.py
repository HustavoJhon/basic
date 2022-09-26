# MENU
menu = """
    Bienvenido al conversor de monedas💲
1 - Pesos colombianos
2 - Pesos argentinos
3 - Pesos mexicanos

Elige una opcion: 
"""
opcion = int(input(menu))

if opcion == 1:
    pesos = input("Cuantos pesos colombinos tines? ")
    pesos = float(pesos)
    valor_dolar = 3875
    dolares = pesos / valor_dolar
    dolares = round(dolares, 2)
    dolares = str(dolares)
    print(f"Tienes ${dolares} dolares")

elif opcion == 2:
    pesos = input("Cuantos pesos argentinos tines? ")
    pesos = float(pesos)
    valor_dolar = 65
    dolares = pesos / valor_dolar
    dolares = round(dolares, 2)
    dolares = str(dolares)
    print(f"Tienes ${dolares} dolares")

elif opcion == 3:
    pesos = input("Cuantos pesos mexicanos tines? ")
    pesos = float(pesos)
    valor_dolar = 24
    dolares = pesos / valor_dolar
    dolares = round(dolares, 2)
    dolares = str(dolares)
    print(f"Tienes ${dolares} dolares")

else:
    print('Elige una opcion correcta')

# Conversor de monedas de Peru