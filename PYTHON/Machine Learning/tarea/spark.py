#pip install pyspark==3.2.0 
from pyspark.sql import SparkSession

spark = SparkSession.builder.appName("Analisis").getOrCreate()

df = spark.read.format("csv").option("header","true").load("./main_product_sales_weather_view.csv")

df.show()

