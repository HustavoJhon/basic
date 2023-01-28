#DICT METHODS

diccionario = {
    "nombre" : 'lucas',
    "apellido" : 'dalto',
    "subs" : 10000
}

# nos devuelve un objeto dict_item
claves = diccionario.keys()

#obteniendo un elemento con get() -si no encuentra nada el programa continua(none)
valor_de_nombre =  diccionario.get("nombre") 

#eliminando todo del diccionario
# diccionario.clear()

#elimando un elemnto del diccionario
diccionario.pop('apellido')

#obteniendo un elemento dict_items iterebla
diccionario_iterable = diccionario.items()