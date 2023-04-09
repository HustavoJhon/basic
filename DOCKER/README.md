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