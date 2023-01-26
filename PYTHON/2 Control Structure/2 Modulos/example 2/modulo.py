# Importacion de un modulo que se encuentra en la misma ruta:
# import modulo_1 
# print(modulo_1.sumar(a, b)(7, 8))
# print(modulo_1.restar(a, b)(7, 8))
# print(modulo_1.multiplicar(7, 8))
# print(modulo_1.dividir(a, b)(7, 8))

# from modulo_1 import multiplicar
# print(multiplicar(7, 8))

# *importacion desde un paquete:
# from mi_paquete.func_math import calcular_factorial, tittle
# print(tittle)
# print(calcular_factorial(5))

# import mi_paquete.func_math
# print(mi_paquete.func_math.calcular_factorial(4))
# print(mi_paquete.func_math.tittle)

# *with aliases 
# import mi_paquete.func_math as fun_math
# print(fun_math.calcular_factorial(3))
# print(fun_math.tittle)

# *importacion de un sub_paquete
from mi_paquete.sub_paquete.func_avanzadas import contar_letras
text = "Gran Chocolatada del Frenton"
print(contar_letras(text))