'''
# text.txt debe estar al mismo nivel del archivo de lectura
line 1
line 2
line 3
line 4 con algo mas

otro mas
line 7
'''

#-------------- Leer un archivo de texto ------------

# leer archivo
file = open('./text.txt')
print(file.read())
# --> line 1
		# line 2
		# line 3
		# line 4 con algo mas
		# 
		# otro mas
		# line 7

# imprimir linea a linea del archivo
file = open('./text.txt')
print(file.readline())
print(file.readline())
print(file.readline())
print(file.readline())
# --> line 1
		# line 2
		# line 3
		# line 4 con algo mas

# leer el archivo con for iterando linea a linea
for line in file:
  print(line)
# --> line 1
		# line 2
		# line 3
		# line 4 con algo mas
		
		# otro mas
		# line 7
  
# es importante cerrar el archivo cuando se termina de trabajar con el
# libera espacio en memoria y cierra el archivo en python
file.close()

# cerrar el archivo una vez se ejecuten ciertas instrucciones con with
with open('./text.txt') as file:
  # leer archivo
  for line in file:
    print(line)
# --> line 1
		# line 2
		# line 3
		# line 4 con algo mas
		
		# otro mas
		# line 7