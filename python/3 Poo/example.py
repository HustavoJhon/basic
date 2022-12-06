#! Definicion
class Persona:
    def __init__(self, nombre, edad):
        self.name = nombre 
        self.age = edad 
    
    def saluda(self, otra_persona):
        return f'Hola {otra_persona.name}, me llamo {self.name}'

david = Persona('David', 35)
erica = Persona('Erica', 25)

print(david.saluda(erica))
print(erica.saluda(david))