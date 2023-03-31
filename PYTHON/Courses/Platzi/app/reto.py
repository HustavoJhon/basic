import csv
import functools

def read_csv(path):
   # Tu código aquí 👇
    with open(path) as csvfile:
        reader = csv.reader(csvfile, delimiter=',')
        data = list()
        for line in reader:
         data.append(int(line[1]))
    total = functools.reduce(lambda x,y: x+y, data)
    return total

response = read_csv('./app/data.csv')
print(response)
