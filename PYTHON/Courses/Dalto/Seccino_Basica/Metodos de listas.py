# Funciones de listas

#creando un lista con list()
lista = list(["hola", "dalto", 24])

# deuelve la cantidad de elementos de la lista
cantidad_eliementos = len(lista)

#agregando un elemento a la lista
lista.append("suscribete")

#agregamos un elemento a la lista en un indice con especifico
lista.insert(2, "youtube")

#agregamos varios elementos a la lista
lista.extend([False, 2023]) # agregando una lista

#eliminando un elemnto de la lista (por su indice)
lista.pop(3) #-1 para eliminar el ultimo, -2 para eliminar el penultimo ...

#removiendo un elemento de la lista por su valor
lista.remove("suscribete")

#eliminandno todos los elemento de la lista por su valor
# lista.clear()

#ordenando la lista  siempre y cuando solo hay una lista con puros numeros
lista.sort()

#inviertiendo los elemntos de una lista
lista.reverse()


#verificamos si un elemento se encuentre en la lista
elemento_encontrado = lista.index("hola")