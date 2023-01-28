# PRACTICE EXCERCISES 1

#Promedio de duracion
otros_cursos_min = 2.5
otros_cursos_max = 7
otros_cursos_promendio = 4
dalto_cursos = 1.5

#Duracion de crudos
crudo_promedio = 5
crudo_dalto = 3.5

# Diferencias de duracion
diferencia_con_min = 100 - dalto_cursos / otros_cursos_min * 100
diferencia_con_max = 100 - dalto_cursos * 1000 // otros_cursos_max / 100
diferencia_con_promedio = 100 - dalto_cursos / otros_cursos_promendio * 100

#Calculamos el porcentaje de tiempo vacio removido
timepo_vacio_promedio = 100 - otros_cursos_promendio * 1000 // crudo_promedio / 10
tiempo_vacio_dalto = 100 - dalto_cursos * 1000 // crudo_dalto / 10

#Mostramos las diferencias de duracion (ejercicio A)
print('El curso de dalto dura:\n')
print(f'- un {diferencia_con_min}% menos que el mas rapido')
print(f'- un {diferencia_con_max}% menos que el mas lento')
print(f'- un {diferencia_con_promedio}% menos que el promedio')
print("--------------------------")

#Mostramos la cantidad de espacios vacios que se remueven (ejercicio B)
print(f'Un curso promedio elimina un {timepo_vacio_promedio}% de tiempo vacio')
print(f'Este curso elimino el {tiempo_vacio_dalto}% de timepo vacio')
print("--------------------------")

#Mostrando diferencias si los cursos duraran 10 horas
print(f'Ver 10 horas de este curso equivale a ver {otros_cursos_promendio * 100 // dalto_cursos / 10} horas de otros cursos')
print(f'Ver 10 horas de otros cursos equivale a ver {dalto_cursos * 100 // otros_cursos_promendio / 10} horas de este cursos')
print("--------------------------")

