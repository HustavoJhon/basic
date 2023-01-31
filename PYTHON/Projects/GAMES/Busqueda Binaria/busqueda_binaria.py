import random
import time


#BUSQUEDA INGENUA (Naive search)
def busqueda_ingenua(lista, objetivo):
    #range(len(lista)) => 0,1,2,3,4,5..., len(lista)-1
    for i in range(len(lista)):
        if lista[i] == objetivo:
            return i
    return -1

#BUSQUEDA BINARIA
def busqueda_binaria(lista, objetivo, limite_inferior=None, limite_superior=None):
    #la lista debe estar ordenada de forma acendente
    if limite_inferior is None:
        limite_inferior = 0 # inicio de la lista
    if limite_superior is None:
        limite_superior = len(lista)-1 #final de la lista

    if limite_superior < limite_inferior:
        return -1

    #Recursion
    punto_medio = (limite_inferior + limite_superior)//2
    
    if lista[punto_medio] == objetivo:
        return punto_medio
    elif objetivo < lista[punto_medio]:
        return busqueda_binaria(lista, objetivo, limite_inferior, punto_medio-1)
    else:
        return busqueda_binaria(lista, objetivo, punto_medio+1, limite_superior)
    
if __name__=='__main__': #esto sirve si no vamos a importar es codigo en otro archivo
    # mi_lista = [1,3,5,10,12]
    # print(busqueda_binaria(mi_lista, 12))

    # Crear una lista ordenada con 1000 numeros aleatorios.
    tamanio = 100000
    conjunto_inicial = set()
    
    while len(conjunto_inicial) < tamanio:
        conjunto_inicial.add(random.randint(-3*tamanio, 3*tamanio))
        
    lista_ordenada = sorted(list(conjunto_inicial))#ordenamos la lista
    
    #Medir el tiempo  de busqueda ingenua.
    inicio = time.time()
    for objetivo in lista_ordenada:
        busqueda_ingenua(lista_ordenada, objetivo)
    fin = time.time()
    print(f"Tiempo de busqueda ingenua: {fin - inicio} segundos")
    
    #Medir el tiempo de busqueda binaria.
    inicio = time.time()
    for objetivo in lista_ordenada:
        busqueda_binaria(lista_ordenada, objetivo)
    fin = time.time()
    print(f"Tiempo de busqueda binaria: {fin - inicio} segundos.")