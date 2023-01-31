#! Mad Libs 

#Concadenar cadenas de carcteres.
# SUpongamos que queremos crear una cadena que diga:
# Aprende a programar con _____.add()

# organizacion = "freeCodeCamp"
# print("Aprende a programar con " + organizacion)
# print("Aprende a programar con {}".format(organizacion))
# print(f"Aprende a programar con {organizacion}")

adj = input("Adjetivo: ")
vervo1 = input("Vervo: ")
vervo2 = input("Vervo: ")
sustantivo_plural = input("Sustantivo (plural): ")

mablib = f"!Programar es tan {adj} Siempre me emociona porque {vervo1} problemas. Aprened a {vervo2} con migo {sustantivo_plural}" 

print(mablib)
