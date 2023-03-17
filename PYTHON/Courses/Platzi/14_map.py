numbers = [1,2,3,4,]
numbers_v2 = []

for i in numbers:
    numbers_v2.append(i * 2)

numbers_v3 = list(map(lambda i:i*2, numbers))

print(numbers)
print(numbers_v2)
print(numbers_v3)


n = [1,2,3,4,5,6]
n2 = [5,6,7]

print(n)
print(n2)
result = list(map(lambda x, y: x + y, n, n2))
print(result)