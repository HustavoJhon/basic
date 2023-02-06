# {key:value for var in iterable}
# (_______)  (_________________)
# elemento  | ciclo donde se extrae elementos
# llave-valor|de cualquier iterable

#example 1

#forma tradicional
dict = {}
for i in range(1,6):
    dict[i] = i * 2
print(dict)

#forma comprehension
dict2 = {i:i*2 for i in range(1,5)}
print(dict2)

#example 3
import random
countries = ['col','mex','bol','pe']
population = {}
for i in countries:
    population[i] = random.randint(1,100)
print(population)
#comprehension
population2 = {i: random.randint(1,100) for i in countries}
print(population2)

#example4
names = ['nico','zule','santi']
ages = [12, 56, 98]
# print(list(zip(names, ages))) nos devuelve una lista
new_dict = {nombre: edad for (nombre, edad) in zip(names, ages)}
print(new_dict)
#otra forma
dict_2 = {names[i]:ages[i] for i in range(len(names))}
print(dict_2)