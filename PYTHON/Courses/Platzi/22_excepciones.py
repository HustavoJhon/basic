#---------- Manejo de errores --------------

# python detiene la ejecucion al detectar un error
print(1/0)
print('hola')
# --> Error division by zero

# Para evitar que el programa se detenga podemos manejar 
# los errores cont try except
try:
  print(0/0)
  assert 1 != 1, 'Uno no es igual a uno'
except ZeroDivisionError as Error:
  print(Error)

print('hola')
# --> division by zero

# assert tiene una funcionalidad que permite lanzar un mensaje
# si no se cumple la condicion
# detiene la ejecucion del programa
assert 1 != 1, 'Uno no es igual a uno'
print('hola nuevamente')
# --> Error Uno no es igual a uno

#capturar el error de assert
try:
  assert 1 != 1, 'Uno no es igual a uno'
except AssertionError as error:
  print(error)

print('hola nuevamente')
# --> Uno no es igual a uno 
# hola nuevamente

# capturar los errores propios para que no detenga el programa
try:
  age = 10
  if age < 18:
    raise Exception('No se permiten menores de edad')
except Exception as error:
  print(error)

print('hola')
# --> No se permiten menores de edad
		# hola

# capturar varios errores en un mismo bloque
# al conseguir un error detiene el bloque try y continua con
# la ejecucion del programa
try:
  print(0/0)
  assert 1 != 1, 'Uno no es igual a uno'
  age = 10
  if age < 18:
    raise Exception('No se permiten menores de edad')
except ZeroDivisionError as Error:
  print(Error)  
except AssertionError as error:
  print(error)
except Exception as error:
  print(error)

print('hola 2')
# --> division by zero
		# hola 2