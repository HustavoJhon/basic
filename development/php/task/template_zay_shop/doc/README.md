# Documentación del Proyecto Template Zay Shop

## Indice de Documentación

Bienvenido a la documentación completa del sistema **Template Zay Shop**. Este proyecto es un sistema de e-commerce desarrollado en PHP puro con arquitectura MVC, contenerizado con Docker.

### Secciones de la Documentación

1. **[Arquitectura General](architecture.md)** - Estructura del proyecto, patron MVC, y flujo de datos
2. **[Capa de Datos (Data/)](data-layer.md)** - Conexion a BD y clases de acceso a datos
3. **[Capa de Modelo (Model/)](model-layer.md)** - Entidades y objetos de dominio
4. **[Capa de Controlador (Controller/)](controller-layer.md)** - Logica de control y manejo de peticiones
5. **[Capa de Vista (View/)](views.md)** - Templates, layouts e includes
6. **[Archivos Publicos](public-files.md)** - Paginas principales (index, login, shop, cart, etc.)
7. **[Configuracion Docker](docker.md)** - Dockerfile, docker-compose.yml y servicios
8. **[Base de Datos](database.md)** - Esquema, tablas y relaciones
9. **[Informe de Codigo](code-explanations.md)** - Explicacion detallada de cada archivo PHP importante

---

## Descripcion Rapida

| Caracteristica | Tecnologia |
|---------------|-------------|
| Backend | PHP 8.2 |
| Servidor Web | Apache (en Docker) |
| Base de Datos | MySQL 8.0 |
| Frontend | Bootstrap 5, HTML5, CSS3, JavaScript |
| Arquitectura | MVC (Modelo - Vista - Controlador) |
| Contenedores | Docker + Docker Compose |
| Autenticacion | Sesiones PHP + bcrypt |

---

## Como Leer Esta Documentacion

Cada archivo de documentacion contiene:
- **Proposito**: Que hace el archivo
- **Codigo comentado**: Explicacion linea por linea
- **Dependencias**: Que archivos incluye o requiere
- **Flujo de ejecucion**: Como se ejecuta y que retorna

---

## Credenciales de Acceso

```
URL: http://localhost:8080
Usuario: admin
Contrasena: 123456
```

---

## Estructura de Carpetas Documentada

```
template_zay_shop/
├── Data/                    # Documentado en data-layer.md
│   ├── conection.php
│   ├── dataCart.php
│   ├── dataProduct.php
│   └── dataUser.php
├── Model/                   # Documentado en model-layer.md
│   └── user.php
├── Controller/              # Documentado en controller-layer.md
│   └── userController.php
├── View/                    # Documentado en views.md
│   ├── includes/
│   │   ├── head.php
│   │   ├── menu.php
│   │   ├── header.php
│   │   ├── footer.php
│   │   ├── body.php
│   │   ├── shop.php
│   │   ├── shop-single.php
│   │   ├── about.php
│   │   └── contact.php
│   ├── css/
│   ├── js/
│   └── img/
├── admin/                   # Panel de administracion
├── *.php (public files)     # Documentado en public-files.md
├── Dockerfile               # Documentado en docker.md
├── docker-compose.yml       # Documentado en docker.md
└── DataBase/               # Documentado en database.md
    └── storeonline.sql
```

---

**Nota**: Esta documentacion NO modifica el codigo original. Solo explica su funcionamiento.
