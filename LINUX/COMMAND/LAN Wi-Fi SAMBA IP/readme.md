# show password wifi

```bash
netsh wlan show profiles
netsh wlan show profile "name_red" key=clear
```

## Trabajo con la RED (LAN Y Wifi

|command|description|
|--|--|
|ifconfig eth0|mostrar la configuracion de una tarjeta de red Ethernet|
|ifup eth0|activar una interfaz 'eth0'|
|ifdown eth0|desabilitar una interface 'eth0'|
|ifconfig eth0 192.168.1.1 netmask 255.255.255.0:|configurar una dirección IP.|
|ifconfig eth0 promisc: |configurar ‘eth0′en modo común para obtener los paquetes (sniffing).
|dhclient eth0: |activar la interface ‘eth0′ en modo dhcp.|
|route -n: |mostrar mesa de recorrido.|
|route add -net 0/0 gw IP_Gateway: |configurar entrada predeterminada.|
|route add -net 192.168.0.0 netmask 255.255.0.0 gw 192.168.1.1: |configurar ruta estática para buscar la red ’192.168.0.0/16′.|
|route del 0/0 gw IP_gateway: |eliminar la ruta estática.|
|echo “1” > /proc/sys/net/ipv4/ip_forward: |activar el recorrido ip.|
|hostname: |mostrar el nombre del host del sistema.|
|host www.example.com: |buscar el nombre del host para resolver el nombre a una dirección ip(1).|
|nslookup www.example.com: |buscar el nombre del host para resolver el nombre a una direccióm ip y viceversa(2).|
|ip link show: |mostar el estado de enlace de todas las interfaces.|
|mii-tool eth0: |mostar el estado de enlace de ‘eth0′.|
|ethtool eth0: |mostrar las estadísticas de tarjeta de red ‘eth0′.|
|netstat -tup: |mostrar todas las conexiones de red activas y sus PID.|
|netstat -tupl: |mostrar todos los servicios de escucha de red en el sistema y sus PID.|
|tcpdump tcp port 80: |mostrar todo el tráfico HTTP.|
|iwlist scan: |mostrar las redes inalámbricas.|
|iwconfig eth1: |mostrar la configuración de una tarjeta de red inalámbrica.|
|whois www.example.com: |buscar en base de datos Whois.|

## Redes de Microsoft Windows (SAMBA)

|comando|descripcion|
|--|--|
|nbtscan ip_addr: |resolucion de nombre de red bios.|
|nmblookup -A ip_addr: |resolucion de nombre de red bios|
|smbclient -L ip_addr/hostname: |mostrar acciones remotadas de un host en windows|
