# METODOS DE CADENAS
# LIST METHDS

#funcion   // metodo
# dir('...') != hola.lower()

cadena1 = "Hola soy Dalto"
cadena2 = "bienvenido"

#convierte a mayusculas
resultado = cadena1.upper()

#convierte a minuscula
resultado=cadena1.lower()

#primera letra en mayuscula
resultado=cadena1.capitalize()


#buscamos una cadena en otra cadena si no hay coincidencias devuelve -1
busqueda_find = cadena1.find("a")

#buscamos una cadena en otra cadena, si no hay coinsidencias lanza una excepcion
busqueda_index = cadena1.index("s")

#si es numerico(un string con numeros), devolvemos True, sino devolvemos False
es_numer_ = cadena1.isnumeric()

#si es alfanumerico devolvemos True, sino devolvemos False
#los caracters alfanumericos son de la a-z y los espacios son especiales
es_alfnumerico = cadena1.isalpha()

#contamos conincidencias de una cadeana, dentro de otra cadena, devuelve la cantidad de coincidencias
contar_coincidencias = cadena1.count("i")

#contamoscuantos caracteres tiene una cadena
#len es una funcion
contar_caracteres = len(cadena1)

#verificamos si una cadena empieza con otra cadena dada, si es asi devulve True
empieza_con = cadena1.startswith("H")

#verificamos si una cadena termina con otra cadena dada, si es asi devulve True
termina_con = cadena1.endswith("o")

#si el calor 1, se encuetra en la cadena original, remplaza el calor q de la misma, por el valor 2
cadena_nueva = cadena1.replace("Hola","Hi")

#separa cadenas con la cadena que le pasemos y nos devuelve una lista
cadena_separada = cadena1.split(",") #separala por comas
cadena.

print(resultado)