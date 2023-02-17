def sum_with_range(n1, n2):
    print(n1,n2)
    sum = 0 
    for x  in range(n1, n2):
        sum += x 
    return sum

resultao = sum_with_range(1, 3)
print(resultao)

resultado2 = sum_with_range(resultao, resultao + 10)
print(resultado2)
