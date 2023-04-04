#------- Escribir sobre un archivo ------------

# r para permiso de lectura
# w para permiso de escritura
# r+ para permisos tanto de lectura como de escritura. Enfocado en agregar
# nuevo contenido al contenido existente en el archivo
# w+ para permisos tanto de lectura como de escritura. Enfocado en reescribir
# el contenido existente

# leer archivo
with open('./text.txt', 'r+') as file:
  # leer archivo
  for line in file:
    print(line)

  # agregar nuevo contenido al existente
  file.write('\n')  # salto de linea
  file.write('una nueva linea\n')

# leer archivo
with open('./text.txt', 'w+') as file:
  # leer archivo
  for line in file:
    print(line)

  # sobreescribir sobre el contenido existente
  file.write('\n')  # salto de linea
  file.write('una nueva linea\n')