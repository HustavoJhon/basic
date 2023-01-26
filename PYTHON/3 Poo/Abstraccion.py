#! Abstracccion

#* Enfcarnos en la informacion relevante
#* Separar la informacion central de los detalles secuandarios 
#* Podemos utilizar variables y metodos (privados o publicos)

#? Example 1
class Lavadora:
    def __init__(self):
        pass 

    def lavar(self, temperatura='caliente'):
        self._llenar_tanque_de_agua(temperatura)
        self._anadir_jabon()
        self._lavar()
        self._centrifugar()

    def _llenar_tanque_de_agua(self, temperatura):
        print(f'llenando el tanque de ague {temperatura}')
    
    def _anadir_jabon(self):
        print('Anadiendo jabon')
    
    def _lavar(self):
        print('lavando la ropa')

    def _centrifugar(self):
        print('centrifigando la ropa')
    
if __name__ == '__main__':
    lavadora = Lavadora()
    lavadora.lavar()
