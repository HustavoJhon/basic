# Conversor de monedas de Peru

pesos = input("Cuantos pesos colombinos tines? ")
pesos = float(pesos)
valor_dolar = 3875
dolares = pesos / valor_dolar
dolares = round(dolares, 2)
dolares = str(dolares)
print(f"Tienes ${dolares} dolares")