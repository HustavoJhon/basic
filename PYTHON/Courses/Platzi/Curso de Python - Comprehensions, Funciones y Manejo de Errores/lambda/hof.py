# ejecutar una funcion dentro de otra funcion

# HOF

def increment(x):
    return x + 1

increment_v2 = lambda x : x + 1

def high_ord_func(x, func):
    return x + func(x)

high_ord_func_v2 = lambda x , func: x + func(x)

result = high_ord_func(2, increment)

# 2 + (2 + 1)
print(result)


result_v2 = high_ord_func_v2(2, increment_v2)
print(result_v2)