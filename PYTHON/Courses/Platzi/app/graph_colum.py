import utils
import readcsv
import charts

def run():
  data = readcsv.readCsv('./app/data.csv')
  labels, values = utils.populationTotal(data)
  charts.generatePieChart(labels, values)

if __name__ == '__main__':
  run()

# función dentro de utils.py
def populationTotal(data):
  percentages = []
  countries = []
  for i in data:
    #countries.append(i['Country/Territory']) 
    countries.append(i['CCA3'])
    percentages.append(float(i['World Population Percentage']))
  return countries, percentages