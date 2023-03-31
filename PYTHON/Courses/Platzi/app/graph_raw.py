import seaborn as sns
import pandas as pd 
import matplotlib.pyplot as plt 


# seleccionar una fila
df = pd.read_csv('./app/data.csv')
data = df[df["Country/Territory"]=="Bolivia"].iloc[:,5:13]
sns.barplot(data=data)
plt.rcParams["figure.figsize"] = (4,3)
plt.xticks(rotation=45);

plt.show()