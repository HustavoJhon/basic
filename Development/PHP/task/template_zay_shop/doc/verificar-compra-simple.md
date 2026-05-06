# Verificar Compra Exitosa

## Paso 1: Entrar a la base de datos

```bash
docker exec -it zay_shop_mysql mysql -u usuario -ppassword storeonline
```

## Paso 2: Ver el stock (si bajó, la compra funcionó)

```sql
SELECT id, nombre, stock FROM productos;
```

## Paso 3: Ver tu pedido registrado

```sql
SELECT * FROM pedidos;
```

## Paso 4: Ver qué compraste

```sql
SELECT * FROM detalle_pedido;
```

## Paso 5: Ver todo junto (usuario + productos + fecha)

```sql
SELECT c.usuario, pr.nombre, dp.cantidad, dp.precio_unitario, p.fecha FROM pedidos p JOIN clientes c ON p.usuario_id = c.id JOIN detalle_pedido dp ON dp.pedido_id = p.id JOIN productos pr ON dp.producto_id = pr.id;
```

## Salir

```sql
EXIT;
```

---

## Todo de una (sin entrar al contenedor)

```bash
docker exec -it zay_shop_mysql mysql -u usuario -ppassword storeonline -e "SELECT c.usuario, pr.nombre, dp.cantidad, dp.precio_unitario, p.fecha FROM pedidos p JOIN clientes c ON p.usuario_id = c.id JOIN detalle_pedido dp ON dp.pedido_id = p.id JOIN productos pr ON dp.producto_id = pr.id;"
```

```bash
docker exec -it zay_shop_mysql mysql -u usuario -ppassword storeonline -e "SELECT id, nombre, stock FROM productos;"
```
