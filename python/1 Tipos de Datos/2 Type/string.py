name = "Nicolas"
last_name = "Molina Monroy"
print(name) 
print(last_name)

# concadenacion
full_name = name + ' ' + last_name
print(full_name)

quote = "I'm Nicolas"
print(quote)

quote_2 = 'She said "Hello gust"'
print(quote_2)


# FORMAT
template_1 = "Hola, mi nombres es " + name + " y mi apellido es " + last_name 
print(template_1)

template_2 = "Hola, mi nombre es {} y mi apellido es {}".format(name, last_name)
print(template_2)

template_3 = f"Hola, mi nombres es {name} y mi apellido es {last_name}"
print(template_3)