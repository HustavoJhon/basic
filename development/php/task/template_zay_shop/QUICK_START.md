# Inicio Rápido - 5 Minutos

## Paso 1: Iniciar Docker

```bash
docker-compose up -d --build
```

Espera 30 segundos mientras se inician los servicios.

## Paso 2: Acceder a la aplicación

Abre tu navegador en: **http://localhost:8080**

## Paso 3: Iniciar sesión

**Usuario:** `admin`
**Contraseña:** `123456`

## ✅ Listo para usar

### Rutas principales:

| Ruta | Descripción |
|------|-------------|
| `http://localhost:8080/` | Página inicio |
| `http://localhost:8080/login.php` | Iniciar sesión |
| `http://localhost:8080/shop.php` | Catálogo de productos |
| `http://localhost:8080/cart.php` | Carrito de compras |
| `http://localhost:8080/admin/` | Panel de administración |

### En el Admin puedes:

1. **Gestionar Productos**
   - Crear productos
   - Editar productos
   - Eliminar productos
   - Ver stock disponible

2. **Gestionar Usuarios**
   - Crear usuarios
   - Editar usuarios
   - Cambiar contraseña
   - Desactivar usuarios

### Flujo de Compra:

1. Inicia sesión
2. Ve a "Catálogo" (shop.php)
3. Agrega productos al carrito
4. Revisa el carrito
5. Confirma la compra
6. Recibe confirmación con detalles del pedido

## Detener la aplicación

```bash
docker-compose down
```

## Problemas Comunes

### Puerto 8080 ya en uso

Cambia el puerto en `docker-compose.yml`:
```yaml
ports:
  - "8081:80"  # Usa 8081 en lugar de 8080
```

### BD no se conecta

Espera 30 segundos y recarga la página. MySQL necesita tiempo para iniciar.

### Limpiar todo

```bash
docker-compose down -v
docker-compose up -d --build
```

## Próximos Pasos

1. **Crear más usuarios** desde el admin
2. **Crear más productos** desde el admin
3. **Hacer compras de prueba**
4. **Revisar confirmaciones de pedidos**

¡Listo! 🚀
