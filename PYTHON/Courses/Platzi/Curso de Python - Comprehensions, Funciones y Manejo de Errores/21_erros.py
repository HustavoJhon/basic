#---------- Errores ---------

# error de sintaxis
# print(0/0)
# --> SyntaxError: unmatched ')'

# error division por cero
# print(0/0)
# --> ZeroDivisionError: division by zero

#variable no declarada
# print(result)
# --> NameError: name 'result' is not defined

#verificacion con assert
suma = lambda x, y: x + y
assert suma(2,2) == 4
# --> AssertionError

# lanzar errores propios con raise
age = 10
if age < 18:
  raise Exception('No se permiten menores de edad')
# --> Exception: No se permiten menores de edad