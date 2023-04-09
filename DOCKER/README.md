# Curso de Fundamentos de Docker 

> Docker te permite construir,
distribuir y ejecutar cualquier
aplicación en cualquier lado.

### Problemas del desarrollo de software profesional

|Construir|Distribuir|Ejecutar|
|--|--|--|
|Entorno de desarrollo|Divergencia de repositorios|Dependencias de aplicación
|Dependencias|Divergencia de artefactos|Compatibilidad del entorno productivo
|Entorno de ejecucion|Versionado|Disponibilidad de servicios externos
|Equivalencia con entorno |Equivalencia con entorno productivo|Recursos de hardware
|Servicios externos|Servicios externos

### Virtualización

> ... versión virtual de algún recurso
tecnológico, como (...) hardware,
un sistema operativo, un
dispositivo de almacenamiento o
(...) recurso de red.

Permite atacar en simultáneo
los tres problemas del
desarrollo de software
profesional.

#### Problemas de las VMsVirtualización

**Peso**

_En el orden de los GBs. Repiten archivos en común.
Inicio lento._

**Costo de administración**

_Necesita mantenimiento igual que cualquier otra
computadora._

**Múltiples de formatos**

_VDI, VMDK, VHD, raw, etc._

### Containerización 

> El empleo de contenedores para
construir y desplegar software. 

- Flexibles
- Libianos
- Portables
- Bajo acoplamiento
- Escalabres
- Seguros 

> A diferencia de una máquina virtual, que es una abstracción del hardware y emula toda una computadora (o servidor), un contenedor es una abstracción del software y éste puede empaquetar el código, el runtime necesario y las dependencias de una aplicación


### Qué es y cómo funciona Docker

<img src="doc/img/arquitectura.png">

## Contenedores

```shell
docker run hello-world
```

**Que es un contenedor?**

- Es una agrupación de procesos.

- Es una entidad lógica, no tiene el limite estricto de las máquinas virtuales, emulación del sistema operativo simulado por otra más abajo.

- Ejecuta sus procesos de forma nativa.

- Los procesos que se ejecutan adentro de los contenedores ven su universo como el contenedor lo define, no pueden ver mas allá del contenedor, a pesar de estar corriendo en una maquina más grande.

- No tienen forma de consumir más recursos que los que se les permite. Si esta restringido en memoria ram por ejemplo, es la única que pueden usar.

- A fines prácticos los podemos imaginar cómo maquinas virtuales, pero NO lo son. Máquinas virtuales livianas.

- Docker corre de forma nativa solo en Linux.

- Sector del disco: Cuando un contenedor es ejecutado, el daemon de docker le dice, a partir de acá para arriba este disco es tuyo, pero no puedes subir mas arriba.

- Docker hace que los procesos adentro de un contenedor este aislados del resto del sistema, no le permite ver más allá.

- Cada contenedor tiene un ID único, también tiene un nombre.