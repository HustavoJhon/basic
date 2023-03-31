#--------- Graficas en Python -----------

import matplotlib.pyplot as plt

# funcion para grafico de barra
def generate_bar_chart(labels, values):
  fig, ax = plt.subplots()
  ax.bar(labels, values)
  plt.show()

# funcion para pie chart
def generate_pie_chart(labels, values):
  fig, ax = plt.subplots()
  ax.pie(values, labels=labels)
  ax.axis('equal')
  plt.show()

# ejecutar archivo como script desde la terminal
if __name__ == '__main__':
  labels = ['a', 'b', 'c']
  values = [20, 50, 10]
  
  generate_bar_chart(labels, values)
  generate_pie_chart(labels, values)