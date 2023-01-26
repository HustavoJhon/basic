# Diccionario

- El siguiente paso a las listas y tuplas que vimos en anteriores, son los diccionarios. Y un diccionario en Python, es como un diccionario en la vida del día a día.

- Es decir, cada palabra tiene asociado un significado.

- Pues exactamente igual tenemos en Python, cada palabra, que llamaremos clave, tiene asociado un significado, que llamaremos valor. Es decir, que un diccionario no es mas que un conjunto de parejas clave – valor. Los diccionarios en Python son un tipo de dato realmente muy potente.

- Indicar que si bien en el caso de un diccionario al uso, la clave (palabra que buscamos), siempre es una cadena de texto, en Python, la clave puede ser cualquier tipo de dato, un entero, una cadena de texto.

- Incluso, es posible que el tipo de dato varíe, es decir, que para un elemento sea de un tipo y para otro elemento del diccionario, sea de otro tipo.
```python
#aprederemos sobre la estructura de los diccionarios, encontraremos una palabra y por ende su definición ==> en Python sería una llave (key), y su definición 
#los diccionarios siempre se definen entre llaves 
my_dic = {'persona,niño'}
print(my_dic)
print(type(my_dic))

my_dic = {
  'persona': 'niño',
  'nombre': 'nicolas',
  'genero':'masculino',
  'edad': '12 años'#aca estamos definiendo los datos de diccionario
}
print(my_dic)
print(len(my_dic))#con len podemos saber cuántos elementos tenemos en el dic

#tambien puedo leer ese diccionario con los métodos 
print(my_dic['persona'])
print(my_dic['nombre'])
print(my_dic['genero'])
print(my_dic['edad'])

#tambien hay otra forma de hacer que es con el método get
print(my_dic.get('persona'))
print(my_dic.get('nombre'))
print(my_dic.get('genero'))
print(my_dic.get('edad'))
print(my_dic.get('juan jose'))
#la diferencia con get es que imprímelos datos que no hay nada definido(none)
#es recomendable siempre trabajar con get, porque si no existe el diccionario no dará un error solo pondrá None 

#tambien podemos verificar si una llave no existe ni (IN), que nos arrojara valores bulenos 
print('persona' in my_dic)
print('apellidos' in my_dic)
#en esta clase vimos los primeros acercamientos a los diccionarios
```
## web
[metodos](https://docs.hektorprofe.net/python/metodos-de-las-colecciones/metodos-de-los-diccionarios/)

## Metodos 

> Python tiene un conjunto de métodos integrados que pueden ser usar en los diccionarios:

- `clear()` --> Elimina todos los elementos del diccionario
- `copy()` --> Devuelve una copia del diccionario
- `fromkeys()` --> Devuelve un diccionario con las claves y el valor especificados
- `get()` --> Devuelve el valor de la clave especificada
- `items()` --> Devuelve una lista que contiene una tupla para cada par de valores clave
- `keys()` --> Devuelve una lista que contiene las claves del diccionario
- `pop()` --> Elimina el elemento con la clave especificada
- `popitem()` --> Elimina el último par clave-valor insertado
- `setdefault()` --> Devuelve el valor de la clave especificada. Si la clave no existe: inserte la clave, con el valor especificado
- `update()` --> Actualiza el diccionario con los pares clave-valor especificados
- `values()` --> Devuelve una lista de todos los valores en el diccionario
