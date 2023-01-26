# TABLA DE MULTIPLICACION
   
contador = 0
x = int(input("end: "))
y = int(input("numero: "))
   
while contador < x:
     contador+=1
     resultado = y*contador
     print(f"{y} X {contador} =  {resultado}")
