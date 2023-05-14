import matplotlib.pyplot as plt
import numpy as np
from pyspark.sql.functions import col
from pyspark.sql import SparkSession

spark = SparkSession.builder.appName("Analisis").getOrCreate()
df = spark.read.format("csv").option("header", "true").load("product_sales_weather_view.csv")

# Calcular el histograma
hist, bin_edges = np.histogram(df.select(col("daily_sales_count").cast("float")).rdd.flatMap(lambda x: x).collect(), bins=10)
# Graficar el histograma
plt.hist(df.select(col("daily_sales_count")).rdd.flatMap(lambda x: x).collect(), bins=10)
plt.xlabel("Valor")
plt.xticks(bin_edges)
plt.ylabel("Frecuencia")
plt.show()