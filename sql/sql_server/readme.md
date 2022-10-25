<div align="center">

```css
  █████████     ██████    █████           █████████  ██████████ ███████████   █████   █████ ██████████ ███████████  
 ███░░░░░███  ███░░░░███ ░░███           ███░░░░░███░░███░░░░░█░░███░░░░░███ ░░███   ░░███ ░░███░░░░░█░░███░░░░░███ 
░███    ░░░  ███    ░░███ ░███          ░███    ░░░  ░███  █ ░  ░███    ░███  ░███    ░███  ░███  █ ░  ░███    ░███ 
░░█████████ ░███     ░███ ░███          ░░█████████  ░██████    ░██████████   ░███    ░███  ░██████    ░██████████  
 ░░░░░░░░███░███   ██░███ ░███           ░░░░░░░░███ ░███░░█    ░███░░░░░███  ░░███   ███   ░███░░█    ░███░░░░░███ 
 ███    ░███░░███ ░░████  ░███      █    ███    ░███ ░███ ░   █ ░███    ░███   ░░░█████░    ░███ ░   █ ░███    ░███ 
░░█████████  ░░░██████░██ ███████████   ░░█████████  ██████████ █████   █████    ░░███      ██████████ █████   █████
 ░░░░░░░░░     ░░░░░░ ░░ ░░░░░░░░░░░     ░░░░░░░░░  ░░░░░░░░░░ ░░░░░   ░░░░░      ░░░      ░░░░░░░░░░ ░░░░░   ░░░░░ 
                                                                                                                         
```

</div>

# Tipos de datos (Transact-SQL)
---

## Tipos de datos

> Los tipos de datos son los valores que pueden tener cada columna: datos enteros, datos de caracteres, datos monetarios, datos de fecha y hora , cadenas bvinarisa, etc. 

### **Categorías de tipos de datos**

los tipos de datos de SQL Server se organizan en las siguientes categorias:

* Numéricos exactos
* Cadenas de caracteres Unicode
* Numéricos aproximados
* Cadenas binarias
* Fecha y hora
*Cadenas de caracteres
* Otros tipos de datos

### **Numéricos exactos**

| Tipos de datos | Intervalo | Storage |
| -- | -- | -- |
| money | De -922.337.203.685.477,5808 a 922.337.203.685.477,5807   | 8 bytes |
| bigint | De -2^63 (-9.223.372.036.854.775.808) a 2^63-1 (9.223.372.036.854.775.807) | 8 bytes |
| smallmoney | De - 214.748,3648 a 214.748,3647 | 4 bytes |
| int | De -2^31 (-2.147.483.648) a 2^31-1 (2.147.483.647) | 4 bytes |
| smallint | De -2^15 (-32.768) a 2^15-1 (32.767) | 2 bytes |
| tinyint	| De 0 a 255 | 1 byte |

### **Numéricos aproximados**

| Tipos de datos | Intervalo | Storage |
| -- | -- | -- |
| float | De - 1,79E+308 a -2,23E-308, 0 y de 2,23E-308 a 1,79E+308 | Depende del valor de n |
| real | De - 3,40E + 38 a -1,18E - 38, 0 y de 1,18E - 38 a 3,40E + 38 | 4 bytes |

### **Fecha y hora**

| Tipos de datos de SQL Server | IntervaloEl formato del literal de cadena predeterminado se pasó al cliente de nivel inferior | StorageSQLCLIENT de nivel inferior |
| -- | -- | -- |
| time | hh:mm:ss[.nnnnnnn]	 | Cadena o SqString |
| date | hh:mm:ss[.nnnnnnn]	 | Cadena o SqString |
| datetime2	| AAAA-MM-DD hh:mm:ss[.nnnnnnn]	 | Cadena o SqString |
| datetimeoffset | AAAA-MM-DD hh:mm:ss[.nnnnnnn] [+/-]hh:mm	| Cadena o SqString |

### **CADENAS DE CARACTERES**

| Tipos de datos | Intervalo | 
| -- | -- | -- |
| char | char [ ( n ) ] Datos de cadena de tamaño fijo. n define el tamaño de la cadena en bytes y debe ser un valor entre 1 y 8000. Para juegos de caracteres de codificación de byte único, como el latino, el tamaño de almacenamiento es n bytes y el número de caracteres que se pueden almacenar también es n. Para los juegos de caracteres de codificación multibyte, el tamaño de almacenamiento sigue siendo n bytes, pero el número de caracteres que se pueden almacenar puede ser menor que n. El sinónimo ISO para char es character |
| varchar | varchar [ ( n | max ) ] Datos de cadena de tamaño variable. Utilice n para definir el tamaño de la cadena en bytes, que puede ser un valor comprendido entre 1 y 8000, o bien use max para indicar un tamaño de restricción de columna hasta un almacenamiento máximo de 2^31-1 bytes (2 GB). Para juegos de caracteres de codificación de byte único, como el latino, el tamaño de almacenamiento es n bytes + 2 bytes y el número de caracteres que se pueden almacenar también es n. Para los juegos de caracteres de codificación multibyte, el tamaño de almacenamiento sigue siendo n bytes + 2 bytes, pero el número de caracteres que se pueden almacenar puede ser menor que n. Los sinónimos ISO para varchar son charvarying o charactervarying |
|text, ntext, image | Tipos de datos de longitud fija y variable para almacenar valores de gran tamaño con datos de caracteres y binarios Unicode y no Unicode. Los datos Unicode utilizan el juego de caracteres UNICODE UCS-2.
Los tipos de datos ntext, text e image se quitarán en una versión futura de SQL Server. Evite su uso en nuevos trabajos de desarrollo y piense en modificar las aplicaciones que los usan actualmente.
ntext: datos Unicode de longitud variable con una longitud máxima de cadena de 2^30 – 1 (1.073.741.823) bytes. El tamaño de almacenamiento, en bytes, es dos veces la longitud de cadena especificada. El sinónimo en ISO de ntext es national text.
text: datos no Unicode de longitud variable en la página de códigos del servidor y con una longitud máxima de cadena de 2^31-1 (2.147.483.647). Cuando la página de códigos del servidor utiliza caracteres de doble byte, el almacenamiento sigue siendo de 2.147.483.647 bytes. Dependiendo de la cadena de caracteres, el espacio de almacenamiento puede ser inferior a 2.147.483.647 bytes.
image: datos binarios de longitud variable desde 0 hasta 2^31-1 (2.147.483.647) bytes |