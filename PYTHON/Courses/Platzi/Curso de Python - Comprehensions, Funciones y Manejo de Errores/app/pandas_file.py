import pandas
filename = './app/data.csv'
data = pandas.read_csv(filename, header=0)

print(data.shape)
print(data.head(10))