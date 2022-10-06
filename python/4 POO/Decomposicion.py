#! DECOMPOSICION    

#* Parti un problema en problemaS mas pequenos.
#* Las clases permitir crear mayores abstracciones
#* Cada clase se encarga de una parte del problema y el programa se vuelve mas facil de mantener

#? Example
print("\n\n")

class Automovil:
    def  __init__(self, modelo, marca, color):
        self.modelo = modelo
        self.marca = marca
        self.color = color 
        self._estando = "en_reposo"
        self._motor = Motor(cilindros = 4) #esto es una  variable privada, por eso se empieza con _
        self._seguridad = AirBag ()

    def acelerar (self, tipo ):
        if tipo == "rapida":
            self._motor.inyectaGasolina(10)
            self._motor.temperatura(12)
        else: 
            self._motor.inyectaGasolina(3)
            self._motor.temperatura(7)
        self._estado = "EnMovimiento"

    def desAcelerar (self, tipo ):

        if tipo == "brusca":
            self._seguridad.activar()
        else:
            pass
            
class Motor:
    def __init__(self, cilindros, tipo = 'gasolina', nivelGasolina = 46000, temperatura = 0 ): #tipo es un parametro ya definido, se le llama default keyword, se entiende comoo un parametro por defecto.
        self.cilindros = cilindros
        self.tipo = tipo 
        self.nivelGasolina = nivelGasolina
        self.estadoTemperatura = temperatura

    def inyectaGasolina(self, cantidadGasolina):
        self.nivelGasolina = self.nivelGasolina - cantidadGasolina
    
    def temperatura (self, grados ):
        self.estadoTemperatura = self.estadoTemperatura + grados

    def informacion (self): #Esta funcion es temporal, solo para revisar que todo esta funcionanndo :v xd 
        print("\n")
        print(f"nivelGasolina = {self.nivelGasolina} y temperatura = {self.estadoTemperatura}")
        print("\n")

class AirBag:

    def __init__(self, estado = "optimo"):
        self.estado = estado

    def activar (self):
        print("SISTEMA DE SEGURAD DE CHOQUES ACTIVADO")
        self.estado = "inhalitado"

if __name__ == "__main__":

    car1 = Automovil("AAFF","toyota", "rojo")
    car1._motor.informacion() 
    car1.acelerar("lenta")
    car1._motor.informacion()
    car1.desAcelerar("brusca")
    
print("\n\n")
