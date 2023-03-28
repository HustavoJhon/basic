import sys # for system
print(sys.path) # direccion de directorio actual

import re #expreciones regulares
text = 'Mi Numero de telefono es 311 123 456, el codigo del pais es 57, mi numero de la suerte es el 3'
resultado = re.findall('[0-9]+', text)
print(resultado)

import time
timestamp = time.time()

local = time.localtime()
result = time.asctime(local)
print(result)


import collections
numbers = [1,2,3,4,5,5,3,3,2,8]
counter = collections.Counter(numbers)
print(counter)
