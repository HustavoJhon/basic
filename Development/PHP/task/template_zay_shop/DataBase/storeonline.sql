CREATE DATABASE IF NOT EXISTS storeonline;
USE storeonline;

-- ========================
-- TABLA CLIENTES (LOGIN)
-- ========================
CREATE TABLE clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(100),
    apellidos VARCHAR(100),
    email VARCHAR(150),
    usuario VARCHAR(50) UNIQUE,
    contrasena VARCHAR(255),
    telefono VARCHAR(20),
    direccion VARCHAR(255),
    estado TINYINT(1) DEFAULT 1,
    fecha_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- USUARIO DE PRUEBA (password: 123456 - hash bcrypt)
INSERT INTO clientes (nombre, apellidos, email, usuario, contrasena, estado)
VALUES (
    'Admin',
    'Principal',
    'admin@store.com',
    'admin',
    '$2y$10$wH8Q0PqQxgX6k6YyYg4T5e6l9Z0g7zZ8e5d1Yw0lXq7mR5Q8vQZbK',
    1
);

-- ========================
-- PRODUCTOS
-- ========================
CREATE TABLE productos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255),
    descripcion TEXT,
    precio DECIMAL(10,2),
    imagen VARCHAR(255),
    stock INT DEFAULT 0,
    activo TINYINT(1) DEFAULT 1
);

INSERT INTO productos (nombre, descripcion, precio, imagen, stock)
VALUES
('Zapatilla Nike', 'Zapatilla deportiva de alta calidad con tecnología de amortiguación', 91.00, 'shop_01.jpg', 10),
('Zapatilla Umbro', 'Zapatilla urbana cómoda para el día a día', 85.00, 'shop_02.jpg', 15),
('Zapatilla Reebok', 'Zapatilla running con soporte para maratones', 78.00, 'shop_03.jpg', 8);

-- ========================
-- PEDIDOS
-- ========================
CREATE TABLE pedidos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT,
    fecha TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    total DECIMAL(10,2) DEFAULT 0,
    estado VARCHAR(50) DEFAULT 'pendiente',
    FOREIGN KEY (usuario_id) REFERENCES clientes(id)
);

-- ========================
-- DETALLE PEDIDO
-- ========================
CREATE TABLE detalle_pedido (
    id INT AUTO_INCREMENT PRIMARY KEY,
    pedido_id INT,
    producto_id INT,
    cantidad INT,
    precio_unitario DECIMAL(10,2),
    FOREIGN KEY (pedido_id) REFERENCES pedidos(id),
    FOREIGN KEY (producto_id) REFERENCES productos(id)
);

-- ========================
-- CARRITO (Opcional - usamos sesión)
-- ========================
CREATE TABLE carrito (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT,
    producto_id INT,
    cantidad INT,
    fecha_agregado TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_id) REFERENCES clientes(id),
    FOREIGN KEY (producto_id) REFERENCES productos(id)
);
