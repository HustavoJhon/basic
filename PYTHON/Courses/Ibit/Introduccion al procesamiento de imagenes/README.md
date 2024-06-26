# Procesamiento de imagenes con Python

> Link video class [here](https://transcripts.gotomeeting.com/#/s/9846b9a9524cf06a4a00bd07b5ab8cfe62a1ba925ced164caf490913b6f8dadb)

> Docente: Ing Oblitas Aristondo Franzua Le Rennard

## Etapas del procesamiento de imagenes

1. Captura
    - Diseño de la propiedades de la captura. Tipo de camara, distancia al objeto, mega pixeles, etc.
2. Pre-procesamiento
    - Reducir el entorno que no es de interes para el problema. Fondo, ruido, etc.
3. Segmentacion
    - Reconocer y extraer cada uno de los objetos presentes en la imagen.
4. Extraccion de caracteristicas
    - Seleccionar y extraer "caracteristicas" apropiadas para la identificacion de los objetos deseados.
5. Identificacion de objetos
    - Utilizar un modelo de toma de decision para decidir a que categoria pertenece cada objeto.
6. **Clasificador**
7. **Entrenamiento**

### Captura

**Paso 1:** Se necesita un sensor de imagenes y la posibilidad de digitalizar la señal producida por el sensor.
- Binaria, Escala de grises, Microscopica, "Industrial", Satelite, Sintetica, Medica

### Pre-Procesamiento:

**Paso 2:** Mejorar la imagen de forma que aumenten las probabilidades de exito en los procesos posteriores.
- *Binarizacion* > cambiar de una imagen de color a blanco y negro
- *Adaptativa Tresh-mean* > mejorar la iluminacion
- *Eliminacion de ruidos* >
- *Suavizado* > filtro que se tendria que usar al final
- *Realce* > filtro para ver las facciones de la cara
- *Umbralizacion:* Metodo Otsu* > pasar las imagenes de color a blanco y negro
- *Umbralizacion: Transformacion de espacios vectorial RGB a HSV:* no se trabaja en RGB, por que sino se tardaria en poner el color minimo-maximo por lo que es mejor en HSV

### Segmentacion

**Paso 3:** Dividir la imagen en sus partes sustituyentes u objetos.
- *Erosion* > limpia el ruido y lo hace mas pequeño
- *Dilatacion* > di la imagen tiene ruido ese ruido crese

