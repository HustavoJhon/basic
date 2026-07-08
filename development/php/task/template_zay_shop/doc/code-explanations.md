# Informe Tecnico: Explicacion del Codigo Fuente

## Introduccion

El presente informe detalla el funcionamiento interno de los archivos PHP mas importantes del sistema Template Zay Shop. Se explican las clases, metodos, flujos de ejecucion y dependencias de cada componente clave del sistema.

---

## 1. Capa de Datos (Data/)

### 1.1 Archivo: Data/conection.php

**Proposito**: Establecer la conexion con la base de datos MySQL utilizando PDO (PHP Data Objects).

**Analisis del codigo**:

```php
<?php
class Conexion {
    public static function conectar() {
        // Configuracion compatible con Docker y local
        $host = getenv('MYSQL_HOST') ?: "localhost";
        $db = getenv('MYSQL_DATABASE') ?: "storeonline";
        $user = getenv('MYSQL_USER') ?: "root";
        $pass = getenv('MYSQL_PASSWORD') ?: "";
```

- **Lineas 1-8**: Se define la clase `Conexion` con un metodo estatico `conectar()`. Las variables de entorno se leen con `getenv()`, lo que permite que el codigo funcione tanto en Docker (donde Docker inyecta las variables) como en un entorno local tradicional. Si no existen las variables, usa valores por defecto.

```php
        try {
            $conexion = new PDO(
                "mysql:host=$host;dbname=$db;charset=utf8mb4",
                $user,
                $pass,
                [PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION]
            );
            return $conexion;
```

- **Lineas 10-16**: Se instancia un objeto `PDO` con los parametros de conexion. Se especifica `utf8mb4` como juego de caracteres para soporte completo de UTF-8. La opcion `PDO::ERRMODE_EXCEPTION` hace que cualquier error en la base de datos lance una excepcion, lo que facilita el manejo de errores.

```php
        } catch (PDOException $e) {
            die("Error de Conexion: " . $e->getMessage());
        }
    }
}
```

- **Lineas 18-20**: Si la conexion falla, se captura la excepcion `PDOException` y se termina el script con un mensaje de error.

**Dependencias**: Ninguna. Es la clase base de acceso a datos.

**Flujo de ejecucion**: Cuando cualquier clase de la capa `Data/` necesita acceder a la base de datos, llama a `Conexion::conectar()` y recibe un objeto PDO activo.

---

### 1.2 Archivo: Data/dataUser.php

**Proposito**: Manejar todas las operaciones CRUD (Create, Read, Update, Delete) relacionadas con los usuarios en la tabla `clientes`.

**Analisis del codigo**:

```php
<?php
require_once __DIR__ . "/conection.php";
require_once __DIR__ . "/../Model/user.php";

class DataUser {
```

- **Lineas 2-3**: Se incluyen dos archivos esenciales: `conection.php` para la conexion a BD y `user.php` para el modelo de usuario.

#### Metodo: insertUser($usuario, $pass)

```php
    public function insertUser($usuario, $pass) {
        try {
            $conexion = Conexion::conectar();
            $passHash = password_hash($pass, PASSWORD_BCRYPT);
            $sql = "INSERT INTO clientes (usuario, contrasena, estado) VALUES (?, ?, 1)";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario, $passHash]);
            return true;
```

- **Lineas 14-23**: Inserta un nuevo usuario. La contrasena se hashea con `password_hash()` usando el algoritmo `PASSWORD_BCRYPT` (bcrypt), que es una practica segura. Se usa un `prepared statement` con marcadores `?` para evitar inyeccion SQL. El estado por defecto es `1` (activo).

#### Metodo: validate($usuario, $pass)

```php
    public function validate($usuario, $pass) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM clientes WHERE usuario = ? AND estado = 1";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario]);
            $usuario_bd = $stmt->fetch(PDO::FETCH_ASSOC);
            
            if ($usuario_bd && password_verify($pass, $usuario_bd['contrasena'])) {
                return $usuario_bd;
            }
            return false;
```

- **Lineas 38-53**: Valida las credenciales de un usuario. Busca al usuario por su nombre y que este activo (`estado = 1`). Luego usa `password_verify()` para comparar la contrasena en texto plano con el hash almacenado en la base de datos. Si es correcto, retorna los datos del usuario; si no, retorna `false`.

#### Metodo: getAll()

```php
    public function getAll() {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT id, usuario, estado FROM clientes";
            $stmt = $conexion->prepare($sql);
            $stmt->execute();
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
```

- **Lineas 64-70**: Obtiene todos los usuarios de la base de datos, pero solo selecciona los campos `id`, `usuario` y `estado` (no se trae las contrasenas por seguridad).

#### Metodo: getById($id)

```php
    public function getById($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT id, usuario, estado FROM clientes WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return $stmt->fetch(PDO::FETCH_ASSOC);
```

- **Lineas 82-88**: Obtiene un usuario especifico por su ID.

#### Metodo: update($id, $usuario, $estado)

```php
    public function update($id, $usuario, $estado) {
        try {
            $conexion = Conexion::conectar();
            $sql = "UPDATE clientes SET usuario = ?, estado = ? WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario, $estado, $id]);
            return true;
```

- **Lineas 102-108**: Actualiza el nombre de usuario y el estado de un usuario especifico.

#### Metodo: updatePassword($id, $pass)

```php
    public function updatePassword($id, $pass) {
        try {
            $conexion = Conexion::conectar();
            $passHash = password_hash($pass, PASSWORD_BCRYPT);
            $sql = "UPDATE clientes SET contrasena = ? WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$passHash, $id]);
            return true;
```

- **Lineas 121-128**: Actualiza la contrasena de un usuario, hasheandola nuevamente con bcrypt.

#### Metodo: delete($id)

```php
    public function delete($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "UPDATE clientes SET estado = 0 WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return true;
```

- **Lineas 140-146**: "Elimina" al usuario cambiando su estado a `0` (inactivo). Esto es un borrado logico, no fisico.

**Dependencias**: `conection.php`, `Model/user.php`

---

### 1.3 Archivo: Data/dataProduct.php

**Proposito**: Manejar las operaciones CRUD de los productos en la tabla `productos`.

**Analisis del codigo**:

```php
<?php
require_once __DIR__ . "/conection.php";

class DataProduct {
```

- **Lineas 1-4**: Incluye la conexion y define la clase.

#### Metodo: getAll()

```php
    public function getAll() {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM productos ORDER BY id DESC";
            $stmt = $conexion->prepare($sql);
            $stmt->execute();
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
```

- **Lineas 10-16**: Obtiene todos los productos ordenados por ID descendente (los mas nuevos primero).

#### Metodo: getById($id)

```php
    public function getById($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM productos WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return $stmt->fetch(PDO::FETCH_ASSOC);
```

- **Lineas 28-34**: Obtiene un producto especifico por su ID.

#### Metodo: create($nombre, $descripcion, $precio, $stock, $imagen)

```php
    public function create($nombre, $descripcion, $precio, $stock, $imagen = null) {
        try {
            $conexion = Conexion::conectar();
            $sql = "INSERT INTO productos (nombre, descripcion, precio, stock, imagen) 
                    VALUES (?, ?, ?, ?, ?)";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$nombre, $descripcion, $precio, $stock, $imagen]);
            return true;
```

- **Lineas 50-57**: Crea un nuevo producto. Todos los campos se pasan como parametros al `prepared statement` para evitar SQL injection.

#### Metodo: update($id, $nombre, $descripcion, $precio, $stock, $imagen)

```php
    public function update($id, $nombre, $descripcion, $precio, $stock, $imagen = null) {
        try {
            $conexion = Conexion::conectar();
            if ($imagen) {
                $sql = "UPDATE productos SET nombre = ?, descripcion = ?, precio = ?, stock = ?, imagen = ? WHERE id = ?";
                $stmt = $conexion->prepare($sql);
                $stmt->execute([$nombre, $descripcion, $precio, $stock, $imagen, $id]);
            } else {
                $sql = "UPDATE productos SET nombre = ?, descripcion = ?, precio = ?, stock = ? WHERE id = ?";
                $stmt = $conexion->prepare($sql);
                $stmt->execute([$nombre, $descripcion, $precio, $stock, $id]);
            }
            return true;
```

- **Lineas 74-86**: Actualiza un producto. Si se proporciona una nueva imagen, la actualiza; si no, mantiene la imagen anterior.

#### Metodo: delete($id)

```php
    public function delete($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "DELETE FROM productos WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return true;
```

- **Lineas 100-106**: Elimina un producto fisicamente de la base de datos.

**Dependencias**: `conection.php`

---

### 1.4 Archivo: Data/dataCart.php

**Proposito**: Gestionar los pedidos y el carrito de compras, utilizando transacciones de base de datos.

**Analisis del codigo**:

```php
<?php
require_once __DIR__ . "/conection.php";

class DataCart {
```

- **Lineas 1-4**: Incluye la conexion y define la clase.

#### Metodo: savePedido($usuario_id, $items)

```php
    public function savePedido($usuario_id, $items) {
        try {
            $conexion = Conexion::conectar();
            $conexion->beginTransaction();
```

- **Lineas 12-17**: Inicia una transaccion con `$conexion->beginTransaction()`. Esto garantiza que todas las operaciones de la base de datos se completen o ninguna se aplique (atomicidad).

```php
            $total = 0;
            foreach ($items as $item) {
                $total += $item['cantidad'] * $item['precio'];
            }
```

- **Lineas 19-22**: Calcula el total del pedido sumando el subtotal de cada item (cantidad * precio).

```php
            $sql = "INSERT INTO pedidos (usuario_id, fecha, total, estado) 
                    VALUES (?, NOW(), ?, 'pendiente')";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario_id, $total]);
            $pedido_id = $conexion->lastInsertId();
```

- **Lineas 24-28**: Inserta el pedido principal en la tabla `pedidos`. Usa `NOW()` para la fecha actual y `lastInsertId()` para obtener el ID del pedido recien creado.

```php
            $sql_detalle = "INSERT INTO detalle_pedido (pedido_id, producto_id, cantidad, precio_unitario) 
                           VALUES (?, ?, ?, ?)";
            $stmt_detalle = $conexion->prepare($sql_detalle);
            
            foreach ($items as $item) {
                $stmt_detalle->execute([
                    $pedido_id,
                    $item['id_producto'],
                    $item['cantidad'],
                    $item['precio']
                ]);
            }
```

- **Lineas 32-44**: Inserta cada item del carrito en la tabla `detalle_pedido`, asociandolos con el ID del pedido.

```php
            $conexion->commit();
            return $pedido_id;
        } catch (PDOException $e) {
            if ($conexion->inTransaction()) {
                $conexion->rollBack();
            }
            error_log("Error al guardar pedido: " . $e->getMessage());
            return false;
        }
    }
```

- **Lineas 46-54**: Si todo sale bien, hace `commit()` para aplicar los cambios. Si hay un error, hace `rollBack()` para deshacer todas las operaciones y registra el error.

#### Metodo: getPedido($pedido_id)

```php
    public function getPedido($pedido_id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM pedidos WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$pedido_id]);
            $pedido = $stmt->fetch(PDO::FETCH_ASSOC);
```

- **Lineas 62-70**: Obtiene la informacion principal de un pedido.

```php
            $sql_detalle = "SELECT dp.*, p.nombre FROM detalle_pedido dp
                           JOIN productos p ON p.id = dp.producto_id
                           WHERE dp.pedido_id = ?";
            $stmt_detalle = $conexion->prepare($sql_detalle);
            $stmt_detalle->execute([$pedido_id]);
            $detalles = $stmt_detalle->fetchAll(PDO::FETCH_ASSOC);
            
            $pedido['detalles'] = $detalles;
            return $pedido;
```

- **Lineas 76-85**: Obtiene los detalles del pedido y los une (`JOIN`) con la tabla `productos` para obtener el nombre del producto. Agrega los detalles como un sub-array al pedido principal.

#### Metodo: getPedidosByUsuario($usuario_id)

```php
    public function getPedidosByUsuario($usuario_id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM pedidos WHERE usuario_id = ? ORDER BY fecha DESC";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario_id]);
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
```

- **Lineas 97-103**: Obtiene todos los pedidos de un usuario especifico, ordenados por fecha descendente.

**Dependencias**: `conection.php`

---

## 2. Capa de Modelo (Model/)

### 2.1 Archivo: Model/user.php

**Proposito**: Definir la estructura de datos (entidad) para un usuario.

**Analisis del codigo**:

```php
<?PHP
    class usuarios{
        // Atributos
        public $id;
        public $usuario;
        public $contrasena;
```

- **Lineas 2-6**: Define la clase `usuarios` con tres atributos publicos: `id`, `usuario` y `contrasena`.

```php
        // Recuperacion de atributos
        function get_id(){
            return $this->id;
        }
        function get_usuario(){
            return $this->usuario;
        }
        function get_contrasena(){
            return $this->contrasena;
        }
```

- **Lineas 8-16**: Metodos "getter" para obtener los valores de los atributos.

```php
        // Asignacion de atributos    
        function set_id($id){
            $this->id = $id;
        }
        function set_usuario($usuario){
            $this->usuario = $usuario;
        }
        function set_contrasena($contrasena){
            $this->contrasena = $contrasena;
        }
    }
```

- **Lineas 17-26**: Metodos "setter" para asignar valores a los atributos.

**Nota**: Esta clase sigue el patron de "Plain Old PHP Object" (POPO). En la implementacion actual del sistema, esta clase se incluye pero los datos se manejan principalmente como arrays asociativos en la capa `Data/`. Sin embargo, esta clase proporciona una estructura formal para representar a un usuario.

**Dependencias**: Ninguna.

---

## 3. Capa de Controlador (Controller/)

### 3.1 Archivo: Controller/userController.php

**Proposito**: Manejar el proceso de autenticacion de usuarios (login).

**Analisis del codigo**:

```php
<?php
session_start();

require_once __DIR__ . "/../Data/dataUser.php";
```

- **Lineas 1-4**: Inicia la sesion PHP y carga la clase `DataUser` que contiene la logica de validacion.

```php
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['usuario']) && isset($_POST['contrasena'])) {
    $usuario = trim($_POST['usuario']);
    $contrasena = trim($_POST['contrasena']);
```

- **Lineas 6-10**: Verifica que la peticion sea POST y que existan los campos `usuario` y `contrasena`. Luego, usa `trim()` para eliminar espacios en blanco al inicio y final.

```php
    if (empty($usuario) || empty($contrasena)) {
        header("Location: ../login.php?error=1");
        exit();
    }
```

- **Lineas 12-15**: Si alguno de los campos esta vacio, redirige al login con un parametro de error.

```php
    $dataUser = new DataUser();
    $usuario_bd = $dataUser->validate($usuario, $contrasena);
```

- **Lineas 18-19**: Instancia `DataUser` y llama al metodo `validate()` para verificar las credenciales.

```php
    if ($usuario_bd) {
        $_SESSION['usuario_id'] = $usuario_bd['id'];
        $_SESSION['usuario'] = $usuario_bd['usuario'];
        $_SESSION['estado'] = $usuario_bd['estado'];
        
        header("Location: ../index.php?login=success");
        exit();
```

- **Lineas 24-31**: Si la validacion es exitosa, guarda los datos del usuario en la sesion (`$_SESSION`) y redirige a la pagina principal con un mensaje de exito.

```php
    } else {
        error_log("Login fallido para usuario: $usuario");
        header("Location: ../login.php?error=1");
        exit();
    }
} else {
    header("Location: ../login.php");
    exit();
}
```

- **Lineas 33-42**: Si la validacion falla, registra un error en el log del servidor y redirige al login con error. Si alguien accede directamente al archivo sin un POST, tambien lo redirige.

**Dependencias**: `Data/dataUser.php`

**Flujo de ejecucion**:
1. Usuario envia formulario de login (POST).
2. `userController.php` recibe los datos.
3. Llama a `DataUser::validate()`.
4. Si es correcto, crea la sesion y redirige a `index.php`.
5. Si es incorrecto, redirige a `login.php` con error.

---

## 4. Archivos Publicos (Raiz)

### 4.1 Archivo: index.php

**Proposito**: Pagina principal de la tienda.

**Analisis del codigo**:

```php
<?php
include "View/includes/head.php";
include "View/includes/menu.php";
include "View/includes/header.php";
include "View/includes/body.php";
include "View/includes/footer.php";
?>
```

- **Lineas 1-6**: Solo incluye los archivos de la vista para armar la pagina. No tiene logica de negocio. Es una pagina puramente visual.

**Dependencias**: `View/includes/head.php`, `menu.php`, `header.php`, `body.php`, `footer.php`

---

### 4.2 Archivo: login.php

**Proposito**: Mostrar el formulario de inicio de sesion y manejar mensajes de error.

**Analisis del codigo**:

```php
<?php
session_start();
include "View/includes/header.php";
include "View/includes/head.php";
?>
```

- **Lineas 1-4**: Inicia sesion y carga los archivos de cabecera HTML.

```php
<form action="Controller/userController.php" method="POST">
    <div class="mb-3">
        <label class="form-label">Usuario</label>
        <input type="text" name="usuario" class="form-control" placeholder="Ingrese su usuario" required>
    </div>
    <div class="mb-3">
        <label class="form-label">Contrasena</label>
        <input type="password" name="contrasena" class="form-control" placeholder="Ingrese su contrasena" required>
    </div>
    <div class="d-grid">
        <button type="submit" class="btn btn-success btn-lg">Ingresar</button>
    </div>
</form>
```

- **Lineas 16-31**: Formulario HTML que envia los datos a `Controller/userController.php` via POST.

```php
<?php if (isset($_GET['error'])): ?>
    <div class="alert alert-danger mt-3 text-center">
        Usuario o contrasena incorrectos
    </div>
<?php endif; ?>
```

- **Lineas 36-40**: Muestra un mensaje de error si el parametro `error` esta presente en la URL (lo que indica que el login fallo).

**Dependencias**: `View/includes/header.php`, `head.php`, `footer.php`, `Controller/userController.php` (destino del formulario)

---

### 4.3 Archivo: logout.php

**Proposito**: Cerrar la sesion del usuario.

**Analisis del codigo**:

```php
<?php
session_start();
session_destroy();
header("Location: index.php?logout=success");
exit();
?>
```

- **Lineas 1-5**: Inicia la sesion, la destruye completamente con `session_destroy()` y redirige a la pagina principal.

**Dependencias**: Ninguna.

---

### 4.4 Archivo: shop.php

**Proposito**: Mostrar el catalogo de productos.

**Analisis del codigo**:

```php
<?php
session_start();
include "View/includes/head.php";
include "View/includes/menu.php";
include "View/includes/header.php";
include "View/includes/shop.php";
include "View/includes/footer.php";
?>
```

- **Lineas 1-7**: Inicia sesion y carga las vistas. La logica real de mostrar productos esta en `View/includes/shop.php`.

**Dependencias**: `View/includes/head.php`, `menu.php`, `header.php`, `shop.php`, `footer.php`

---

### 4.5 Archivo: cart.php

**Proposito**: Gestionar el carrito de compras (agregar, eliminar, vaciar) y procesar la compra.

**Analisis del codigo**:

```php
<?php
session_start();
require_once "Data/conection.php";
require_once "Data/dataProduct.php";
require_once "Data/dataCart.php";

if (!isset($_SESSION['usuario_id'])) {
    header("Location: login.php");
    exit();
}
```

- **Lineas 1-10**: Inicia sesion, carga las clases de datos y verifica que el usuario este autenticado. Si no, lo redirige al login.

```php
if (!isset($_SESSION['carrito'])) {
    $_SESSION['carrito'] = [];
}
```

- **Lineas 17-19**: Inicializa el carrito en la sesion si no existe.

```php
if (isset($_GET['add'])) {
    $producto_id = intval($_GET['add']);
    $producto = $dataProduct->getById($producto_id);
    
    if ($producto && $producto['stock'] > 0) {
        $encontrado = false;
        foreach ($_SESSION['carrito'] as &$item) {
            if ($item['id_producto'] == $producto_id) {
                $item['cantidad']++;
                $encontrado = true;
                break;
            }
        }
        if (!$encontrado) {
            $_SESSION['carrito'][] = [
                'id_producto' => $producto_id,
                'nombre' => $producto['nombre'],
                'precio' => $producto['precio'],
                'cantidad' => 1
            ];
        }
    }
}
```

- **Lineas 23-45**: Si se pasa el parametro `add` en la URL, busca el producto y lo agrega al carrito en la sesion. Si ya existe, aumenta la cantidad; si no, lo agrega como nuevo item.

```php
if (isset($_GET['remove'])) {
    $producto_id = intval($_GET['remove']);
    $_SESSION['carrito'] = array_filter($_SESSION['carrito'], function($item) use ($producto_id) {
        return $item['id_producto'] != $producto_id;
    });
    $_SESSION['carrito'] = array_values($_SESSION['carrito']);
}
```

- **Lineas 54-59**: Elimina un producto del carrito usando `array_filter()`.

```php
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['comprar'])) {
    if (empty($_SESSION['carrito'])) {
        $mensaje = '<div class="alert alert-danger">El carrito esta vacio</div>';
    } else {
        $pedido_id = $dataCart->savePedido($_SESSION['usuario_id'], $_SESSION['carrito']);
        if ($pedido_id) {
            $_SESSION['carrito'] = [];
            header("Location: confirmation.php?pedido=" . $pedido_id);
            exit();
        }
    }
}
```

- **Lineas 76-89**: Si se presiona el boton "Confirmar Compra", llama a `savePedido()` para guardar el pedido en la base de datos. Si tiene exito, vacia el carrito y redirige a la pagina de confirmacion.

**Dependencias**: `Data/conection.php`, `dataProduct.php`, `dataCart.php`, `View/includes/head.php`, `menu.php`, `header.php`, `footer.php`

---

### 4.6 Archivo: confirmation.php

**Proposito**: Mostrar los detalles de un pedido confirmado.

**Analisis del codigo**:

```php
<?php
session_start();
require_once "Data/conection.php";
require_once "Data/dataCart.php";

if (!isset($_SESSION['usuario_id'])) {
    header("Location: login.php");
    exit();
}

$pedido_id = intval($_GET['pedido'] ?? 0);
if ($pedido_id <= 0) {
    header("Location: shop.php");
    exit();
}

$dataCart = new DataCart();
$pedido = $dataCart->getPedido($pedido_id);
```

- **Lineas 1-19**: Inicia sesion, verifica autenticacion y obtiene los detalles del pedido usando `getPedido()`.

**Dependencias**: `Data/conection.php`, `dataCart.php`, `View/includes/head.php`, `menu.php`, `header.php`, `footer.php`

---

## 5. Capa de Vista (View/includes/)

### 5.1 Archivo: View/includes/head.php

**Proposito**: Contiene las etiquetas `<head>` de HTML con las referencias a CSS y fuentes.

**Analisis del codigo**:

```html
<head>
    <title>Zay Shop eCommerce HTML CSS Template</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    
    <link rel="apple-touch-icon" href="View/img/apple-icon.png">
    <link rel="shortcut icon" type="image/x-icon" href="View/img/favicon.ico">
    
    <link rel="stylesheet" href="View/css/bootstrap.min.css">
    <link rel="stylesheet" href="View/css/templatemo.css">
    <link rel="stylesheet" href="View/css/custom.css">
    
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@100;200;300;400;500;700;900&display=swap">
    <link rel="stylesheet" href="View/css/fontawesome.min.css">
</head>
```

- Define el juego de caracteres UTF-8, la configuracion de la ventana de visualizacion para dispositivos moviles, iconos de la pagina, hojas de estilo de Bootstrap, TemplateMo y FontAwesome, y fuentes de Google Fonts.

---

### 5.2 Archivo: View/includes/menu.php

**Proposito**: Barra de navegacion superior con informacion de contacto y enlaces a redes sociales.

**Analisis del codigo**:

- Muestra el correo electronico, telefono y enlaces a Facebook, Instagram, Twitter y LinkedIn.
- Es una barra de navegacion oscura que solo se muestra en pantallas grandes (`d-none d-lg-block`).

---

### 5.3 Archivo: View/includes/header.php

**Proposito**: Barra de navegacion principal con el logo y el menu de la tienda.

**Analisis del codigo**:

- Contiene el logo "Zay".
- Enlaces de navegacion: Home, About, Shop, Contact.
- Iconos de busqueda, carrito de compras y usuario.

---

### 5.4 Archivo: View/includes/footer.php

**Proposito**: Pie de pagina con informacion de contacto, enlaces rapidos y formulario de suscripcion.

**Analisis del codigo**:

- Tres columnas: informacion de la empresa, productos y mas informacion.
- Enlaces a redes sociales.
- Formulario de suscripcion (no funcional, solo visual).
- Copyright.

---

### 5.5 Archivo: View/includes/shop.php

**Proposito**: Mostrar la cuadricula de productos obtenidos de la base de datos.

**Analisis del codigo**:

```php
<?php
require_once __DIR__ . "/../../Data/dataProduct.php";

$dataProduct = new DataProduct();
$productos = $dataProduct->getAll();
?>
```

- **Lineas 2-5**: Carga la clase `DataProduct` y obtiene todos los productos.

```php
<?php foreach ($productos as $producto): ?>
    <div class="col-md-4 mb-4">
        <div class="card product-wap shadow-sm">
            <div class="card rounded-0 position-relative overflow-hidden" style="height: 250px; background-color: #f5f5f5;">
                <img class="card-img rounded-0 img-fluid w-100 h-100" src="View/img/shop_01.jpg" alt="<?php echo htmlspecialchars($producto['nombre']); ?>" style="object-fit: cover;">
```

- **Lineas 22-26**: Itera sobre cada producto y muestra su imagen, nombre, descripcion, precio y estado de stock.

```php
<?php if ($producto['stock'] > 0): ?>
    <a class="btn btn-success text-white" href="cart.php?add=<?php echo $producto['id']; ?>">
        <i class="fas fa-cart-plus"></i> Agregar
    </a>
<?php else: ?>
    <button class="btn btn-secondary text-white" disabled>
        <i class="fas fa-ban"></i> Sin Stock
    </button>
<?php endif; ?>
```

- **Lineas 29-38**: Si hay stock, muestra un boton para agregar al carrito; si no, muestra un boton deshabilitado.

**Dependencias**: `Data/dataProduct.php`

---

## 6. Configuracion Docker

### 6.1 Archivo: Dockerfile

**Proposito**: Definir la imagen Docker para el servidor PHP/Apache.

**Analisis del codigo**:

```dockerfile
FROM php:8.2-apache
```

- **Linea 1**: Usa la imagen oficial de PHP 8.2 con Apache preconfigurado.

```dockerfile
RUN docker-php-ext-install pdo pdo_mysql mysqli
```

- **Linea 4**: Instala las extensiones de PHP necesarias para conectarse a MySQL (`pdo`, `pdo_mysql`, `mysqli`).

```dockerfile
RUN a2enmod rewrite
```

- **Linea 7**: Habilita el modulo `rewrite` de Apache (necesario para URL amigables, aunque en este proyecto no se usa activamente).

```dockerfile
COPY . /var/www/html/
```

- **Linea 13**: Copia todos los archivos del proyecto al directorio raiz de Apache en el contenedor.

```dockerfile
RUN chown -R www-data:www-data /var/www/html
```

- **Linea 16**: Asigna la propiedad de los archivos al usuario `www-data` (el usuario con el que corre Apache).

```dockerfile
EXPOSE 80
CMD ["apache2-foreground"]
```

- **Lineas 19-20**: Expone el puerto 80 y define el comando para iniciar Apache en primer plano.

---

### 6.2 Archivo: docker-compose.yml

**Proposito**: Orquestar los servicios de MySQL y PHP/Apache.

**Analisis del codigo**:

```yaml
services:
  mysql:
    image: mysql:8.0
    container_name: zay_shop_mysql
    environment:
      MYSQL_ROOT_PASSWORD: root
      MYSQL_DATABASE: storeonline
      MYSQL_USER: usuario
      MYSQL_PASSWORD: password
```

- **Lineas 5-12**: Define el servicio `mysql` usando la imagen oficial de MySQL 8.0. Configura las variables de entorno para la base de datos, el usuario y la contrasena.

```yaml
    ports:
      - "3307:3306"
    volumes:
      - mysql_data:/var/lib/mysql
      - ./DataBase/storeonline.sql:/docker-entrypoint-initdb.d/storeonline.sql
```

- **Lineas 13-17**: Mapea el puerto 3307 del host al 3306 del contenedor. Crea un volumen para persistencia de datos y monta el archivo SQL para inicializar la base de datos la primera vez.

```yaml
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      timeout: 5s
      retries: 10
```

- **Lineas 20-23**: Configura una verificacion de salud para asegurar que MySQL este listo antes de iniciar el servicio PHP.

```yaml
  php:
    build: .
    container_name: zay_shop_php
    depends_on:
      mysql:
        condition: service_healthy
```

- **Lineas 26-31**: Define el servicio `php` que se construye desde el `Dockerfile` en la raiz. Depende del servicio `mysql` y espera a que este este saludable.

```yaml
    environment:
      MYSQL_HOST: mysql
      MYSQL_USER: usuario
      MYSQL_PASSWORD: password
      MYSQL_DATABASE: storeonline
    ports:
      - "8080:80"
    volumes:
      - .:/var/www/html
```

- **Lineas 32-40**: Pasa las variables de entorno a PHP para la conexion a BD. Mapea el puerto 8080 del host al 80 del contenedor y monta el directorio actual para desarrollo en vivo.

```yaml
volumes:
  mysql_data:

networks:
  zay_network:
    driver: bridge
```

- **Lineas 44-48**: Define el volumen persistente para MySQL y una red personalizada para que los contenedores se comuniquen.

---

## Conclusion

El sistema Template Zay Shop implementa una arquitectura MVC basica en PHP puro, con una clara separacion de responsabilidades:

1. **Model**: Define la estructura de datos (`Model/user.php`).
2. **View**: Presentacion visual (`View/includes/`).
3. **Controller**: Logica de control (`Controller/userController.php`).
4. **Data**: Acceso a datos con PDO y prepared statements (`Data/`).

La configuracion con Docker permite un despliegue consistente y portable, mientras que el uso de `password_hash()` y `password_verify()` garantiza la seguridad en la autenticacion.
