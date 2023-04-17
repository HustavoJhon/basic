# Curso de Administración de Servidores Linux: Manejo de Recursos

## ¿Cómo es el arranque del sistema?

<img src="doc/img1.png">

|Inicio del sistema|Bios|
|--|--|
|Fase 1. bootloader|MBR: Master Boot Record|
|Fase 2. bootloader|LILO o GRUB|
|Kernel (nucleo)|Linux|
|Inicilaiazion|Init|
|Boot Splash|Usplash o Splashy|
|Ambiente grafico|Gnome, KDE, XFCE|
|Usuario|Espacio del usuario|

**Firmware del sistema**.

Hace un inventario y chequeo de todos
los dispositivos conectados al sistema al
momento del arranque (discos, memorias,
etc.).

Este a su vez crea interfaces para que
el software del sistema operativo pueda
usar estos dispositivos.

**Boot Loader**

Separa el firmware del sistema y el
arranque del sistema operativo. Es un paso
previo a que se ejecute el sistema
operativo.
Es útil para correr el SO en forma de rescate
o con parámetros extra.

**GRUB: the GRand Unified Boot loader**

Es el boot loader por defecto en la mayoría
de distribuciones Linux.
Actualmente contamos con dos versiones:.

● GRUB Legacy

● GRUB 2

**Lecturas recomendadas**

[oracle](https://docs.oracle.com/cd/E50691_01/html/E50101/gnchj.html)

[Intel Server](https://www.intel.com/content/www/us/en/support/articles/000033003/server-products.html)

