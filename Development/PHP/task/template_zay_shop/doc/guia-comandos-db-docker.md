# Guía: Consultar la Base de Datos por Docker

## 1. Acceder a la consola de MySQL

### Opción A: Entrar directamente al contenedor MySQL

```bash
docker exec -it zay_shop_mysql mysql -u usuario -ppassword storeonline
```

### Opción B: Entrar como root

```bash
docker exec -it zay_shop_mysql mysql -u root -proot storeonline
```

> Después de ejecutar cualquiera de los dos comandos, vas a estar dentro del prompt de MySQL. Ahí podés correr las queries de abajo.

### Opción C: Ejecutar una query sin entrar al contenedor

```bash
docker exec -it zay_shop_mysql mysql -u usuario -ppassword storeonline -e "SELECT * FROM clientes;"
```

---

## 2. Comandos útiles una vez dentro de MySQL

### Ver todas las tablas

```sql
SHOW TABLES;
```

### Ver la estructura de una tabla

```sql
DESCRIBE clientes;
DESCRIBE productos;
DESCRIBE pedidos;
DESCRIBE detalle_pedido;
```

---

## 3. Verificar compras y pedidos

### Ver todos los clientes registrados

```sql
SELECT id, usuario, nombre, email, estado, fecha_registro FROM clientes;
```

### Ver todos los productos y su stock

```sql
SELECT id, nombre, precio, stock, activo FROM productos;
```

### Ver todos los pedidos

```sql
SELECT * FROM pedidos ORDER BY fecha DESC;
```

### Ver el detalle de cada pedido (qué productos compró cada usuario)

```sql
SELECT
    p.id AS pedido_id,
    c.usuario,
    c.nombre,
    pr.nombre AS producto,
    dp.cantidad,
    dp.precio_unitario,
    (dp.cantidad * dp.precio_unitario) AS subtotal,
    p.estado,
    p.fecha
FROM pedidos p
JOIN clientes c ON p.usuario_id = c.id
JOIN detalle_pedido dp ON dp.pedido_id = p.id
JOIN productos pr ON dp.producto_id = pr.id
ORDER BY p.fecha DESC;
```

### Ver el total de cada pedido

```sql
SELECT
    p.id AS pedido_id,
    c.usuario,
    COUNT(dp.producto_id) AS cantidad_productos,
    SUM(dp.cantidad * dp.precio_unitario) AS total,
    p.estado,
    p.fecha
FROM pedidos p
JOIN clientes c ON p.usuario_id = c.id
JOIN detalle_pedido dp ON dp.pedido_id = p.id
GROUP BY p.id, c.usuario, p.estado, p.fecha
ORDER BY p.fecha DESC;
```

### Ver si el stock cambió después de una compra

```sql
SELECT id, nombre, stock FROM productos ORDER BY id;
```

> Si hiciste una compra, el stock del producto comprado debería haber bajado.

### Ver compras de un usuario específico

```sql
SELECT * FROM pedidos WHERE usuario_id = 1;
```

---

## 4. Flujo completo para verificar una compra

**Paso 1:** Hacé una compra en la web → http://localhost:8080

**Paso 2:** Entrá a la base de datos:
```bash
docker exec -it zay_shop_mysql mysql -u usuario -ppassword storeonline
```

**Paso 3:** Verificá que el pedido se creó:
```sql
SELECT * FROM pedidos ORDER BY fecha DESC LIMIT 1;
```

**Paso 4:** Verificá los productos del pedido:
```sql
SELECT * FROM detalle_pedido WHERE pedido_id = (SELECT MAX(id) FROM pedidos);
```

**Paso 5:** Verificá que el stock bajó:
```sql
SELECT nombre, stock FROM productos;
```

**Paso 6:** Salir de MySQL:
```sql
EXIT;
```

---

## 5. Resetear la base de datos (si querés empezar de cero)

```bash
docker-compose down -v
docker-compose up --build -d
```

> Esto borra todo el volumen de MySQL y recrea la base desde el archivo `storeonline.sql`.
