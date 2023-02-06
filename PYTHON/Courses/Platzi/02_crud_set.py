set_countries = {'col','mex','per','bol'}

#tamanio del conjunto
size = len(set_countries)
print(size)

#preguntar si el elemento esta dentro del conjunto
print('col' in set_countries)

# add 
set_countries.add('ecu')
print(set_countries)

#update
set_countries.update({'arg','pan'})
print(set_countries)

# remove
set_countries.remove('col')
print(set_countries) 
# set_countries.remove('argentina') no no muestra nada 

# clear
set_countries.clear()
print(len(set_countries))

