# Tablas IP (CORTAFUEGOS)   

1.iptables -t filter -L: mostrar todas las cadenas de la tabla de filtro.
2.iptables -t nat -L: mostrar todas las cadenas de la tabla nat.
3.iptables -t filter -F: limpiar todas las reglas de la tabla de filtro.
4.iptables -t nat -F: limpiar todas las reglas de la tabla nat.
5.iptables -t filter -X: borrar cualquier cadena creada por el usuario.
6.iptables -t filter -A INPUT -p tcp –dport telnet -j ACCEPT: permitir las
conexiones telnet para entar.
7.iptables -t filter -A OUTPUT -p tcp –dport http -j DROP: bloquear las
conexiones HTTP para salir.
8.iptables -t filter -A FORWARD -p tcp –dport pop3 -j ACCEPT: permitir las
conexiones POP a una cadena delantera.
9.iptables -t filter -A INPUT -j LOG –log-prefix “DROP INPUT”: registrando
una cadena de entrada.
10.iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE: configurar un
PAT (Puerto de traducción de dirección) en eth0, ocultando los paquetes de salida
forzada.
11.iptables -t nat -A PREROUTING -d 192.168.0.1 -p tcp -m tcp –dport 22 -j
DNAT –to-destination 10.0.0.2:22: redireccionar los paquetes diriguidos de un
host a otro.
