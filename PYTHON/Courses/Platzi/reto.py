# para resolver este desafio, tu resot es utilizar la funcion map de Python y una duncion lambda para trasformar una lista de numeors. Debes retornar una lista en la que cada numero de la lista orifinal sea mulltiplicado por dos.
# La funcion  multiply_numbers revibira como entrada una lista con numeros. Finalmente, la funcion retornara la lista transformada.

#Ejemplo:
#input : [2,4,5,6,7]
#output : [4,8,10,12,16]

def multiply_numbers(numbers):
    result = list(map(lambda x : x * 2, numbers))
    return result

numbers = [2,4,6,8]

resolve = multiply_numbers(numbers)
print(resolve)


# reto 24

# filtra las palabras que tengan mas de 4 letras

def x(words):
    new_list = list(filter(lambda item : len(item) >= 4, words))
    return new_list

words = ['amor','dia','piedra','sol']
response = x(words)
print(response)