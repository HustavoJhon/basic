# Comandos para GNU/Linux 

## Informacion del sistema

1. `arch`: mostrar la arquitectura de la máquina (1).
2. `uname -m`: mostrar la arquitectura de la máquina (2).
3. `uname -r`: mostrar la versión del kernel usado.
4. `dmidecode -q`: mostrar los componentes (hardware) del sistema.
5. `hdparm -i /dev/hda`: mostrar las características de un disco duro.
6. `hdparm -tT /dev/sda`: realizar prueba de lectura en un disco duro.
7. `cat /proc/cpuinfo`: mostrar información de la CPU.
8. `cat /proc/interrupts`: mostrar las interrupciones.
9. `cat /proc/meminfo`: verificar el uso de memoria.
10. `cat /proc/swaps`: mostrar ficheros swap.
11. `cat /proc/version`: mostrar la versión del kernel.
12. `cat /proc/net/dev`: mostrar adaptadores de red y estadísticas.
13. `cat /proc/mounts`: mostrar el sistema de ficheros montado.
14. `lspci -tv`: mostrar los dispositivos PCI.
15. `lsusb -tv`: mostrar los dispositivos USB.
16. `date`: mostrar la fecha del sistema.
17. `cal 2011`: mostrar el almanaque de 2011.
18. `cal 07 2011`: mostrar el almanaque para el mes julio de 2011.
19. `date 041217002011.00`: colocar (declarar, ajustar) fecha y hora.
20. `clock -w`: guardar los cambios de fecha en la BIOS.

## Apagar (Reiniciar Sistema o Cerrar Sesion)
1.shutdown -h now: apagar el sistema (1).
2.init 0: apagar el sistema (2).
3.telinit 0: apagar el sistema (3).
4.halt: apagar el sistema (4).
5.shutdown -h hours:minutes &: apagado planificado del sistema.
6.shutdown -c: cancelar un apagado planificado del sistema.
7.shutdown -r now: reiniciar (1).
8.reboot: reiniciar (2).
9.logout: cerrar sesión.

## Archivos y Directorios

1.cd /home: entrar en el directorio “home”.
2.cd ..: retroceder un nivel.
3.cd ../..: retroceder 2 niveles.
4.cd: ir al directorio raíz.
5.cd ~user1: ir al directorio user1.
6.cd -: ir (regresar) al directorio anterior.
7.pwd: mostrar el camino del directorio de trabajo.
8.ls: ver los ficheros de un directorio.
9.ls -F: ver los ficheros de un directorio.
10.ls -l: mostrar los detalles de ficheros y carpetas de un directorio.
11.ls -a: mostrar los ficheros ocultos.
12.ls *[0-9]*: mostrar los ficheros y carpetas que contienen números.
13.tree: mostrar los ficheros y carpetas en forma de árbol comenzando por la raíz.
(1)
14.lstree: mostrar los ficheros y carpetas en forma de árbol comenzando por la
raíz.(2)
15.mkdir dir1: crear una carpeta o directorio con nombre ‘dir1′.
16.mkdir dir1 dir2: crear dos carpetas o directorios simultáneamente (Crear dos
directorios a la vez).
17.mkdir -p /tmp/dir1/dir2: crear un árbol de directorios.18.rm -f file1: borrar el fichero llamado ‘file1′.
19.rmdir dir1: borrar la carpeta llamada ‘dir1′.
20.rm -rf dir1: eliminar una carpeta llamada ‘dir1′ con su contenido de forma
recursiva. (Si lo borro recursivo estoy diciendo que es con su contenido).
21.rm -rf dir1 dir2: borrar dos carpetas (directorios) con su contenido de forma
recursiva.
22.mv dir1 new_dir: renombrar o mover un fichero o carpeta (directorio).
23.cp file1: copiar un fichero.
24.cp file1 file2: copiar dos ficheros al unísono.
25.cp dir /* .: copiar todos los ficheros de un directorio dentro del directorio de
trabajo actual.
26.cp -a /tmp/dir1 .: copiar un directorio dentro del directorio actual de trabajo.
27.cp -a dir1: copiar un directorio.
28.cp -a dir1 dir2: copiar dos directorio al unísono.
29.ln -s file1 lnk1: crear un enlace simbólico al fichero o directorio.
30.ln file1 lnk1: crear un enlace físico al fichero o directorio.
31.touch -t 0712250000 file1: modificar el tiempo real (tiempo de creación) de
un fichero o directorio.

32.file file1: salida (volcado en pantalla) del tipo mime de un fichero texto.
33.iconv -l: listas de cifrados conocidos.
34.iconv -f fromEncoding -t toEncoding inputFile > outputFile: crea una
nueva forma del fichero de entrada asumiendo que está codificado en
fromEncoding y convirtiéndolo a ToEncoding.
35.find . -maxdepth 1 -name *.jpg -print -exec convert ”{}” -resize 80×60
“thumbs/{}” \;: agrupar ficheros redimensionados en el directorio actual y
enviarlos a directorios en vistas de miniaturas (requiere convertir desde
ImagemagicK).

## Encontrar archivos
1.find / -name file1: buscar fichero y directorio a partir de la raíz del sistema.
2.find / -user user1: buscar ficheros y directorios pertenecientes al usuario
‘user1′.
3.find /home/user1 -name \*.bin: buscar ficheros con extensión ‘. bin’ dentro
del directorio ‘/ home/user1′.
4.find /usr/bin -type f -atime +100: buscar ficheros binarios no usados en los
últimos 100 días.
5.find /usr/bin -type f -mtime -10: buscar ficheros creados o cambiados dentro
de los últimos 10 días.
6.find / -name \*.rpm -exec chmod 755 ‘{}’ \;: buscar ficheros con extensión
‘.rpm’ y modificar permisos.
7.find / -xdev -name \*.rpm: Buscar ficheros con extensión ‘.rpm’ ignorando los
dispositivos removibles como cdrom, pen-drive, etc.…
8.locate \*.ps: encuentra ficheros con extensión ‘.ps’ ejecutados primeramente
con el command ‘updatedb’.
9.whereis halt: mostrar la ubicación de un fichero binario, de ayuda o fuente. En
este caso pregunta dónde está el comando ‘halt’.
10.which halt: mostrar la senda completa (el camino completo) a un binario /ejecutable.


