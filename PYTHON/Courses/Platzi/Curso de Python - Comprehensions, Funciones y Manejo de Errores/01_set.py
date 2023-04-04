# CONJUNTOS
# - un conjunto no puede tener elementos duplicados
# - no tiene un orden especifico


set_countries = {'col','mex','bol'}
print(set_countries)
print(type(set_countries))

sets_numbers = {1,2,3,4,5,6,7,8,9}
print(sets_numbers)


set_types = {1, 'hola',False,12.12}
print(set_types)

set_from_string = set('hola')
print(set_from_string)

set_from_tuples = set(('abc','cdv','as'))
print(set_from_tuples)


numbers = [1,2,3,3,4,5,5,6,7,8,9]
set_number = set(numbers)
print(set_number)
unique_numbers = list(set_number) #numeros sin repetidos
print(unique_numbers)