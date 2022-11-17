tittle = 'Factorial de un Numero'

def calcular_factorial(number):
    factorial = 1 
    for n in range(1, (number + 1)):
        factorial *= n
    return factorial