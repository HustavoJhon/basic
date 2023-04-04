#{key: value for var in iterable if condition}
#(__________)(_________________)(____________)
#elemento    ciclo donde se extren condicion 
#llave-valor elementos de cualquier opcional para 
#                  iterable         filtrar elementos


import random
contries = ['col','mex','bol','pe']
popultaion = {contry:random.randint(1,100) for contry in contries}
print(popultaion)

result = {contry: popultaion for (contry, popultaion) in popultaion.items() if popultaion > 70}
print(result)


#EXAMPLE 2  
text = 'Hola, soy Nicolas'
unique = {c: c.upper() for c in text if c in 'aeiou'}
print(unique)