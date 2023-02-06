# [element  for element in iterable]
# (______)  (_____________________)
# elemento | ciclo donde se extraen elementos
#          |   de cualquier iterable


# [element for element in iterable if condition]
# (______)(______________________) (___________)
# elemento|ciclo donde se extraen | condicion opcional
#         |elementos de cualquier |para filtrar elementos
#         |  iterable
# example -> [i*2 for i in range(1,101)if i %2 == 0]

#de la forma tradicional
numbers = []
for element in range(1,11):
    numbers.append(element * 2)
print(numbers)

#example 1
numbers_v2 = [element * 2 for element in range(1,11)]
print(numbers_v2)


# example 2
# forma tradicional
numbers1 = []
for i in range(1,11):
    if i % 2 == 0:
        numbers1.append(i*2)
print(numbers1)
#forma list comprehension
numbers2 = [i * 2 for i in range(1,11) if i % 2 == 0]
print(numbers2)


#example 3
days = ["lunes", "martes", "miercoles", 
        "jueves", "viernes", "sabado", "domingo"]
newlist = [i for i in days if "l" in i]
print(newlist) # ["martes", "sabado"]

