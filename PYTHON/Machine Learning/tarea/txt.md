## CASO DE USO: Predicción de Ventas de Helados 

Disponemos de los datos históricos de las ventas de 36 heladerías ubicadas en España, así como de la localización de las mismas, sus horarios y algunas características propias como la posibilidad de consumo de productos en la propia tienda o la posibilidad de adquisición de productos Premium. Además de esto, se han extraído datos de fuentes externas acerca del calendario laboral y las temperaturas registradas diariamente en los últimos años. 

A partir de los datos cada heladería, combinados con los datos obtenidos de fuentes externas, se busca predecir las ventas en una fecha o rango de fechas futuro (como se puede ver en el Notebook de Zeppelin). Para realizar esto, es necesario construir un modelo predictivo que permita, a partir de las variables de entrada, determinar el valor de la variable objetivo (la cantidad de ventas diaria en euros). 

Mediante el uso del algoritmo de regresión GBT (árboles potenciados por gradiente), es posible modelar el comportamiento de las ventas en base a una serie de reglas. Este algoritmo, basado en el uso de árboles de decisión, genera una secuencia lineal de árboles que le permiten realizar un buen ajuste sobre los datos de entrada y tratar tanto variables cuantitativas como cualitativas, sin necesidad de normalizar los datos. 

El uso de Apache Spark, con la librería MlLib, permite realizar la construcción del modelo predictivo y la realización de predicciones en un clúster de hadoop, donde se encuentran disponibles las fuentes de información citadas.

Tras realizar pruebas con diferentes modelos de Machine Learning de regresión (como la regresión lineal, arboles de decisión o Random Forest), se ha determinado que GBT es el algoritmo que ofrece los mejores resultados tras analizar las métricas de RMSE y el coeficiente R2, siguiendo un procedimiento de validación cruzada de los modelos creados.

## MODELO DEL CASO DE USO:
Ver modelo del caso de uso

## ARQUITECTURA DEL MODELO 
En el caso de estudio que presentamos, hacemos uso de la herramienta Pentaho Data Integration para procesar datos de diferentes fuentes de datos. Esta información interna y externa incluye datos históricos sobre las ventas de las heladerías, así como datos acerca del calendario laboral, la temperatura, horarios de apertura etc. 

Aunque la aplicación de demostración no contempla el procesamiento de datos de APIs o el uso de Wescraping, estas tecnologías se han implementado en otros proyectos y pueden ser integradas con Pentaho Data Integration. Tras la carga y procesamiento de datos, se ha generado un archivo de datos almacenado en un clúster de Hadoop (HDFS). 

Se ha utilizado Apache Spark para la explotación de los datos mediante el uso de Spark SQL y Spark MlLib. Se ha construído un modelo predictivo para determinar las ventas de las heladerías partiendo de la fecha y las variables de entorno que podrían condicionar las mismas. Se ha utilizado el algoritmo GBT que ha demostrado ofrecer los resultados más adecuados en la evaluación de diferentes modelos de regresión. 

Se ha utilizado la herramienta Apache Zeppelin para la visualización de los datos y la realización dinámica de predicciones por parte de los usuarios, utiizando los modelos predictivos generados. Las interfaces de usuario de Zeppelin pueden ser construídas de forma sencilla mediante el uso de formularios dinamicos de Zeppelin o bien de forma más personalizable mediante el uso de HTML, CSS, Javascript y Angular. 

La configuración realizada en el clúster permite la utilización de otras herramientas de visualización como Apache Superset para la realización de cuadros de mandos, así como la conexión entre el clúster y herramientas externas de Business Intelligence como pueden ser Tableau, PowerBI o Pentaho.

**PASO 1:** Arquitectura Propuesta Distribución de Hadoop Hortonworks (HDP) open source para el despliegue de un clúster de procesamiento y almacenamiento distribuido y escalable. Incluye herramientas para almacenar y procesar cualquier tipo de fuente de datos: • Spark con Python, R, SQL o Scala para Machine Learning • Hive para data warehousing y SQL, Kafka para adquisición real time, Kylin y Druid para OLAP • Zeppelin y Superset para visualización de datos Ver Arquitectura propuesta 

**PASO 2:** Preparación y carga de datos Se dispone de un archivo CSV generado a partir de un proceso ETL con Pentaho Data Integration (PDI) que ha sido almacenado en un clúster de Hadoop. Los datos del archivo pueden ser procesados con Apache Spark a través del uso de objetos DataFrame para realizar la analítica y predicciones de ventas futuras, mediante la aplicación de algoritmos de Machine Learning. PASO 3: Analítica tradicional Una vez leídos los datos de las ventas, es posible analizar los mismos utilizando Spark SQL y realizar representaciones gráficas que permitan interpretar su estructura y contenido. Teniendo en cuenta la ubicación de la heladería, las fechas de venta y otros parámetros, es posible realizar consultas sobre los datos históricos para entender el comportamiento pasado de las ventas que se han realizado, las cuales varían ampliamente dependiendo de estas características. Ver Analítica Tradicional-Imagen Ver Analítica Tradicional-Datos Ver histórico de Ventas por Mes 

PASO 4: Machine Learning. Algoritmo de regresión GBT para la predicción de ventas Se construirá un modelo de regresión de Machine Learning a partir de los datos de las ventas de las heladerías, generadas en en base a las diferentes variables de entorno. Se utilizará la librería Spark MlLib (Machine Learning) para la construcción de los modelos necesarios para predecir las ventas. Tras analizar los resultados obtenidos con múltiples algoritmos de regresión (regresión lineal, árboles de decisión, Random Forest y GBT), se ha determinado que el algoritmo GBT es el que ofrece mejores resultados en las predicciones y, por tanto, se ha seleccionado para realizar las predicciones. 

**PASO 5:** Creación del modelo Ver Archivo de Codigo Fuente PASO 6: Evaluación del Modelo Una vez construido el modelo, es posible evaluarlo siguiendo la técnica de la validación cruzada. Para ello se realizan predicciones sobre los datos de test y se comparan los valores predecidos con los valores reales. Por último, se calcula el RMSE y el coeficiente r2 como métricas de evaluación de la calidad de los resultados. Para conseguir una predicción más ajustada, se ha construido un modelo de regresión específico para cada heladería y se han calculado las métricas de evaluación de las predicciones realizadas. A continuación, se muestran los resultados obtenidos para cada una de las heladerías. Es posible comprobar las heladerías cuyas predicciones son más fiables (las que presentan un valor más cercano a 1 en el coeficiente r2). Ver archivo de evaluación del modelo

**PASO 7:** Ejecución del modelo. Generación de Predicciones de ventas (utilizando regresión)

Una vez construido el modelo de regresión para la predicción de ventas, es posible realizar predicciones para una fecha concreta o un rango de fechas para una heladería específica (teniendo en cuenta las diferentes variables de entrada que fueron utilizadas para construir el modelo).

En esta aplicación de demostración, se consulta el calendario para cada día a predecir para comprobar si es laborable, sábado, domingo o festivo y se pondera la temperatura media según el mes del año para conseguir predicciones fiables en rangos de fechas. En lo referente a las horas de apertura, consumo in situ y venta de productos Premium, se asume que estas características se mantienen a lo largo del periodo seleccionado en la predicción.
