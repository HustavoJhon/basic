def fibonacci(n):
    print(n)
    if n == 0 or n == 1:
        return 1
    print(n)
    return fibonacci(n - 1) + fibonacci(n - 2)

n = int(input("Escribe un entero: "))
print(f"El numero fibonnacci para {n} es {fibonacci(n)}")
