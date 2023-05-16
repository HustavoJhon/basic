# TIPOS ESTRUCTURADOS, MUTABILIDAD Y FUNCIONES DE ALTO NIVEL

## Funciones como objetos

## Tuplas

1. Una tupla consiste en objetos inmutables. (Objetos que no pueden cambiar después de la creación)

2. Una tupla tiene poca memoria.

3. Una tupla se almacena en un solo bloque de memoria.

4. Crear una tupla es más rápido que crear una lista.

5. Un elemento en una tupla no se puede quitar ni reemplazar.

## Rangos

- Representan una secuencia de enteros, y se definen como range(inicio, fin, paso), y al igual que las cadenas y las tuplas, los rangos son inmutables.
- Son muy eficientes en uso de memoria y normalmente son utilizados en for loops.
- Nuevamente, el final no es inclusivo 👀.
- Para ver la dirección en memoria de un objeto, usamos la función id(objeto).
- Si lo que queremos es comparar igualdad de objetos en lugar de igualdad de valores, podemos hacer objeto is objeto_2.
- Solución al reto: range(1, 100, 2)
```py
my_range = range(1, 10, 1)
print(type(my_range)) #Tipo 

for i in range(2, 10, 2): #Imprime pares
	print(i)
```

## Listas y mutabilidad

Son secuencias de objetos pero a diferencia de tuplas si son mutables
Cuando modificas una lista pueden existir efectos secuendarios (sideffects), Podemos ir creciendo la memoria.
Cada vez que mutamos la lista puede haber un bug.
Es posible iterar con ellas.

Podemos utilizar, appedn, pop, remove, insert, etc… métodos de lista.
```py
lista=[]
```
Tambien podemos usar la notación de slices.
```py
lista_one=[1,2,3]
lista_one.append(n) => Me igresa n valor de cualquier tipo.
```
Los side effect
Sucede por ejemplo cuando estamos apuntando al mismo lugar en memoria desde dos variables
```py
a =[1,2]
b = a
id(a) = id(b) 
Y esto no debe ocurrir
Al asignar b y a volviendolos el mismo objeto si hago un append en uno será reflejado en el otro.
```
**CONCEPTO**:Clonación

Muchos patrones de diseño obligan a que no exista muabilidad precismanete para eliminar el problema de llistas
apuntando al mismo espcio en memoria.

Casi siempre e mejor clonar na lista en vez de mutarla
Para clonar podemos usar la notación de slices o podemos usar la función list que vien por defecto en python.
Esto evita los side effects.
```py
    a = [1, 2, 3, 4]
    b = a
    c = list(a)
    print(f' Mi  lista a {a[:]} tiene en memoria el id de: {id(a)}')
    print(f' Mi  lista b {b} tiene en memoria el id de: {id(b)}')
    print(f' Mi  lista {c} tiene en memoria el id de: {id(c)}')
```
Una forma de clonar más compacta es:
```py
d = a[::] => Le estoy indicando que me tome desde el principio hasta el final contando de 1 en 1.
```
Siempre es mejor clonar las listas
CONCEPTO:List comprehension

Es una forma concisa de crear listas en las que se puede aplicar operaciones a los valores de una secuencia o lista.
También se pueden aplicar condiciones para filtrar.
```py
my_list = list(range(100))
    print(f'quiero operar my_list de valores: \n {my_list}')
    print('A continuación la notación de List comprehensión \n')
    double =[i ** 2 for i in my_list]
    print('[i ** 2 for i in my_list] ')
    print(f'Mi lista con todos sus valores elevados al cuadrado:\n {double}')
    pares = [i for i in my_list if i % 2 == 0]
    print(f'Mi lista con todos sus valores pares:\n {pares}')
```

https://docs.python.org/3/tutorial/datastructures.html#more-on-lists

## Diccionarios

- Son como listas, pero en lugar de usar índices utilizan llaves, y no tienen un orden especifico.
- Los diccionarios son mutables y pueden iterarse.
- Es muy eficiente acceder a sus valores, mucho mas que con una lista.
- Para iterar en un diccionario:
    - Podemos iterar a lo largo de los valores con dict.keys()
    - A lo largo de las llaves dict.values()
    - Por llave y valor dict.items(), (tienes que hacer unpacking)
```py
#Estructura general de un diccionario.
my_dict = {'key1' : 'value1',
            'key2' : 'value2',
            'key3' : 'value3'}

value_3 = my_dict['key3']  #Para acceder a un valor
#Para revisar si hay una entrada
'key5' in my_dict

#Regresa otra cosa si no existe
my_dict.get('key4', 'No key in dict')

#Para reasiganar un valor: 
my_dict['key2'] = 'new_value2'

#Nuevo valor en diccionario
my_dict['key4'] = 'value4'

#Para borrar valor
del my_dict['key1']
```
Para hacer dictionary comprehension:
```py
dict_variable = {key:value for (key,value) in dictonary.items()}
```

