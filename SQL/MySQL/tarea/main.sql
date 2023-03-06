CREATE TABLE Usuarios (
    id_usuario INT NOT NULL AUTO_INCREMENT,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    correo_electronico VARCHAR(100) NOT NULL,
    contrasena VARCHAR(100) NOT NULL,
    fecha_registro DATE NOT NULL,
    PRIMARY KEY (id_usuario)
);

CREATE TABLE Perfiles (
    id_perfil INT NOT NULL AUTO_INCREMENT,
    id_usuario INT NOT NULL,
    nombre_usuario VARCHAR(50) NOT NULL,
    imagen_perfil VARCHAR(255),
    descripcion_perfil TEXT,
    PRIMARY KEY (id_perfil),
    FOREIGN KEY (id_usuario) REFERENCES Usuarios(id_usuario)
);

CREATE TABLE Juegos (
    id_juego INT NOT NULL AUTO_INCREMENT,
    titulo VARCHAR(100) NOT NULL,
    descripcion TEXT NOT NULL,
    desarrollador VARCHAR(100) NOT NULL,
    editor VARCHAR(100) NOT NULL,
    genero VARCHAR(50) NOT NULL,
    fecha_lanzamiento DATE NOT NULL,
    precio DECIMAL(10, 2) NOT NULL,
    PRIMARY KEY (id_juego)
);

CREATE TABLE Plataformas (
    id_plataforma INT NOT NULL AUTO_INCREMENT,
    nombre VARCHAR(50) NOT NULL,
    fabricante VARCHAR(100) NOT NULL,
    anio_lanzamiento INT NOT NULL,
    PRIMARY KEY (id_plataforma)
);

CREATE TABLE Juegos_Plataformas (
    id_juego_plataforma INT NOT NULL AUTO_INCREMENT,
    id_juego INT NOT NULL,
    id_plataforma INT NOT NULL,
    fecha_lanzamiento DATE NOT NULL,
    precio DECIMAL(10, 2) NOT NULL,
    PRIMARY KEY (id_juego_plataforma),
    FOREIGN KEY (id_juego) REFERENCES Juegos(id_juego),
    FOREIGN KEY (id_plataforma) REFERENCES Plataformas(id_plataforma)
);

CREATE TABLE Clasificaciones (
    id_clasificacion INT NOT NULL AUTO_INCREMENT,
    nombre VARCHAR(50) NOT NULL,
    descripcion TEXT,
    PRIMARY KEY (id_clasificacion)
);

CREATE TABLE Juegos_Clasificaciones (
    id_juego_clasificacion INT NOT NULL AUTO_INCREMENT,
    id_juego INT NOT NULL,
    id_clasificacion INT NOT NULL,
    PRIMARY KEY (id_juego_clasificacion),
    FOREIGN KEY (id_juego) REFERENCES Juegos(id_juego),
    FOREIGN KEY (id_clasificacion) REFERENCES Clasificaciones(id_clasificacion)
);

CREATE TABLE Comentarios (
    id_comentario INT NOT NULL AUTO_INCREMENT,
    id_usuario INT NOT NULL,
    id_juego INT NOT NULL,
    comentario TEXT NOT NULL,
    fecha_publicacion DATE NOT NULL,
    PRIMARY KEY (id_comentario),
    FOREIGN KEY (id_usuario) REFERENCES Usuarios(id_usuario),
    FOREIGN KEY (id_juego) REFERENCES Juegos(id_juego)
);

CREATE TABLE Valoraciones (
    id_valoracion INT NOT NULL AUTO_INCREMENT,
    id_usuario INT NOT NULL,
    id_juego INT NOT NULL,
    valoracion INT NOT NULL,
    fecha_valoracion DATE NOT NULL,
    PRIMARY KEY (id_valoracion),
    FOREIGN KEY (id_usuario) REFERENCES Usuarios(id_usuario),
    FOREIGN KEY (id_juego) REFERENCES Juegos(id_juego)
);

CREATE TABLE Compras (
    id_compra INT NOT NULL AUTO_INCREMENT,
    id_usuario INT NOT NULL,
    id_juego INT NOT NULL,
    fecha_compra DATE NOT NULL,
    precio DECIMAL(10, 2) NOT NULL,
    PRIMARY KEY (id_compra),
    FOREIGN KEY (id_usuario) REFERENCES Usuarios(id_usuario),
    FOREIGN KEY (id_juego) REFERENCES Juegos(id_juego)
);
