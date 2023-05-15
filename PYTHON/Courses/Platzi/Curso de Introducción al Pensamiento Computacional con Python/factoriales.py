def fatorial(n):
    """ Calcula el factorial de n.
    
    n int > 0
    return n!
    """
    print(n)
    if n == 1:
        return 1 
    return n * fatorial(n - 1)

n = int(input('Escribe un entero: '))

print(fatorial(n))