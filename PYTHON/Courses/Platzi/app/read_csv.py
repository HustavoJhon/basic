#----- modulo para leer el acrhivo csv --------
import csv

# funcion abrir archivo
def read_csv(path):
  with open(path, 'r') as csvfile:
    reader = csv.reader(csvfile, delimiter=',')

    #nombre de las columnas se encuentra en la primera fila
    header = next(reader)
    
    data = []
    for row in reader:
      iterable =zip(header, row) # une los valores de la listas en tuplas
      country_dict = {key:value for key, value in iterable}
      data.append(country_dict)

    return data

# correr archivo como script desde la terminal
if __name__ == '__main__':
    data = read_csv('./app/data.csv')
    print(data[0])