#creando un conjutno con set()
conjunto = set(["dato1","dato2"])

#metiendo un conjutno dentro de otro conjunto
conjunto1 = frozenset({"dato1","dato2"}) #frozenset es para meter un conjunto dentro de otro conjunto
conjunto2 = {conjunto1,"dalto3"}

print(conjunto)

#Teoria de conjuntos
conjunto1 = {1,3,5,7}
conjunto2 = {1,3,7}

resultado = conjunto2.issubset(conjunto1)#conjunto2 es un subconjunto de conjunto1?
print(resultado)