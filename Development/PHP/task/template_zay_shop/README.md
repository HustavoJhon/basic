# Template Zay Shop - Sistema MVC con Login y Carrito

Sistema de e-commerce desarrollado en **PHP puro** con arquitectura **MVC**, implementado con **Docker** para fácil despliegue.

## Características Implementadas

**Sistema de Autenticación**
- Login/Logout funcional
- Contraseñas hasheadas con bcrypt
- Validación de usuarios activos
- Sesiones seguras

**CRUD de Usuarios**
- Crear, leer, actualizar y eliminar usuarios
- Gestión de estado (activo/inactivo)
- Cambio de contraseña

**CRUD de Productos**
- Crear, editar y eliminar productos
- Control de stock
- Panel de administración

**Carrito de Compras**
- Agregar/remover productos
- Persistencia en base de datos
- Cálculo automático de totales
- Confirmación de pedidos

## Estructura del Proyecto

```
template_zay_shop/
├── admin/                    # Panel de administración
│   ├── index.php            # Menú admin
│   ├── products.php         # Gestión de productos
│   ├── newProduct.php       # Crear producto
│   ├── updateProduct.php    # Editar producto
│   ├── clients.php          # Gestión de usuarios
│   ├── newClient.php        # Crear usuario
│   └── updateClient.php     # Editar usuario
├── Data/                     # Capa de Datos
│   ├── conection.php        # Conexión a BD (PDO)
│   ├── dataUser.php         # CRUD de usuarios
│   ├── dataProduct.php      # CRUD de productos
│   └── dataCart.php         # Gestión de pedidos
├── Model/                    # Modelos (Entidades)
│   └── user.php
├── View/                     # Vistas (Presentación)
│   ├── includes/
│   │   ├── head.php
│   │   ├── menu.php
│   │   ├── header.php
│   │   ├── footer.php
│   │   └── shop.php
│   ├── css/
│   ├── js/
│   └── img/
├── DataBase/                 # SQL
│   └── storeonline.sql
├── Dockerfile               # Configuración Docker
├── docker-compose.yml       # Orquestación de servicios
└── Archivos públicos:
    ├── index.php           # Página inicio
    ├── login.php           # Login
    ├── logout.php          # Logout
    ├── shop.php            # Catálogo de productos
    ├── cart.php            # Carrito
    └── confirmation.php    # Confirmación de compra
```

## Requisitos

- Docker
- Docker Compose

## Instalación y Ejecución

### 1. Clonar o descargar el proyecto

```bash
cd template_zay_shop
```

### 2. Construir y ejecutar con Docker

```bash
docker-compose up -d --build
```

Esto inicia:
- **MySQL** en puerto `3307`
- **PHP/Apache** en puerto `8080`

### 3. Acceder a la aplicación

- **Sitio web:** http://localhost:8080
- **Admin:** http://localhost:8080/admin/

### Credenciales de Prueba

```
Usuario: admin
Contraseña: 123456
```

## Tecnologías Utilizadas

| Componente | Tecnología | Versión |
|-----------|-----------|---------|
| **Servidor Web** | Apache | 2.4 |
| **Backend** | PHP | 8.2 |
| **Base de Datos** | MySQL | 8.0 |
| **Frontend** | Bootstrap | 5.x |
| **Autenticación** | bcrypt | - |

## Flujo de la Aplicación

### 1. **Inicio de Sesión**
```
login.php → Controller/userController.php → Data/dataUser.php → BD
```

### 2. **Carrito de Compras**
```
shop.php (listar productos) → cart.php (agregar) → BD (pedidos)
```

### 3. **Admin - CRUD**
```
admin/ (formularios) → Controller → Data/ → BD
```

## Base de Datos

**Usuario de BD:**
```
Host: mysql
Usuario: usuario
Contraseña: password
Base: storeonline
```

**Tabla de Usuarios:** `clientes`
- `id`, `usuario`, `contrasena` (bcrypt), `estado`, `email`, `telefono`, `direccion`

**Tabla de Productos:** `productos`
- `id`, `nombre`, `descripcion`, `precio`, `stock`, `imagen`

**Tabla de Pedidos:** `pedidos`
- `id`, `usuario_id`, `fecha`, `total`, `estado`

**Tabla de Detalles:** `detalle_pedido`
- `id`, `pedido_id`, `producto_id`, `cantidad`, `precio_unitario`

## Funcionalidades Principales

###  Autenticación
- Login con validación de usuario/contraseña
- Logout que destruye la sesión
- Contraseñas protegidas con hash bcrypt
- Verificación de estado activo del usuario

### 🛒 Carrito
- Agregar productos al carrito (sesión + BD)
- Remover productos
- Vaciar carrito
- Confirmar compra (guarda en BD)
- Historial de pedidos

### 👥 Administración de Usuarios
- Crear usuarios con contraseña hasheada
- Editar usuario (cambiar nombre, estado)
- Cambiar contraseña
- Desactivar usuarios
- Listar todos los usuarios

### Administración de Productos
- Crear productos con nombre, descripción, precio, stock
- Editar productos
- Eliminar productos
- Control de inventario

## Seguridad Implementada

**Prepared Statements** - Protección contra SQL Injection
**Password Hashing** - bcrypt para contraseñas
**Validación de Entrada** - htmlspecialchars() para XSS
**Sesiones Seguras** - Control de autenticación
**Separación MVC** - Lógica separada de presentación

## Notas de Desarrollo

### Acceder a la BD desde fuera del contenedor

```bash
mysql -h localhost -u usuario -p -D storeonline
```

Contraseña: `password`

### Ver logs del contenedor

```bash
docker-compose logs -f php
docker-compose logs -f mysql
```

### Detener los contenedores

```bash
docker-compose down
```

### Reiniciar desde cero

```bash
docker-compose down -v  # Elimina BD
docker-compose up -d --build  # Recrea todo
```

## Mejoras Futuras

- [ ] Autenticación con JWT
- [ ] API REST
- [ ] Paginación en listados
- [ ] Búsqueda de productos
- [ ] Reportes de ventas
- [ ] Sistema de permisos por rol
- [ ] Integración de pagos
- [ ] Envío de emails

## Estructura MVC Explicada

### **Model (Modelos)**
`Model/user.php` - Define las entidades del sistema

### **View (Vistas)**
`View/includes/*.php` - Templates HTML/PHP para presentación

### **Controller (Controladores)**
`Controller/userController.php` - Lógica de autenticación

### **Data (Capa de Datos)**
`Data/dataUser.php`, `Data/dataProduct.php` - Comunicación con BD

## Créditos

Desarrollado con PHP puro siguiendo estándares MVC y buenas prácticas de seguridad.
