1. Tabla "Usuarios":
    - id_usuario (clave primaria)
    - nombre
    - apellido
    - correo electrónico
    - contraseña
    - fecha de registro
2. Tabla "Perfiles":
    - id_perfil (clave primaria)
    - id_usuario (clave foránea a la tabla "Usuarios")
    - nombre de usuario
    - imagen de perfil
    - descripción de perfil
Tabla "Juegos":
    - id_juego (clave primaria)
    - título
    - descripción
    - desarrollador
    - editor
    - género
    - fecha de lanzamiento
    - precio
Tabla "Plataformas":
    - id_plataforma (clave primaria)
    - nombre
    - fabricante
    - año de lanzamiento
Tabla "Juegos_Plataformas":
    - id_juego_plataforma (clave primaria)
    - id_juego (clave foránea a la tabla "Juegos")
    - id_plataforma (clave foránea a la tabla "Plataformas")
    - fecha de lanzamiento en la plataforma
    - precio en la plataforma
Tabla "Clasificaciones":
    - id_clasificación (clave primaria)
    - nombre
    - descripción
Tabla "Juegos_Clasificaciones":
    - id_juego_clasificación (clave primaria)
    - id_juego (clave foránea a la tabla "Juegos")
    - id_clasificación (clave foránea a la tabla "Clasificaciones")
Tabla "Comentarios":
    - id_comentario (clave primaria)
    - id_usuario (clave foránea a la tabla "Usuarios")
    - id_juego (clave foránea a la tabla "Juegos")
    - comentario
    - fecha de publicación
Tabla "Valoraciones":
    - id_valoración (clave primaria)
    - id_usuario (clave foránea a la tabla "Usuarios")
    - id_juego (clave foránea a la tabla "Juegos")
    - valoración (puntuación del 1 al 5)
    - fecha de valoración
Tabla "Compras":
    - id_compra (clave primaria)
    - id_usuario (clave foránea a la tabla "Usuarios")
    - id_juego (clave foránea a la tabla "Juegos")
    - fecha de compra
    - precio de compra

#### Las relaciones entre estas tablas serían las siguientes:

- Un usuario puede tener muchos perfiles (relación 1 a muchos entre la tabla "Usuarios" y la tabla "Perfiles").
- Un juego puede estar disponible en muchas plataformas, y una plataforma puede tener muchos juegos (relación muchos a muchos entre la tabla "Juegos" y la tabla "Plataformas", resuelta con la tabla intermedia "Juegos_Plataformas").
- Un juego puede tener muchas clasificaciones, y una clasificación puede aplicarse a muchos juegos (relación muchos a muchos entre la tabla "Juegos" y la tabla "Clasificaciones", resuelta con la tabla intermedia "Juegos_Clasificaciones").
- Un usuario puede hacer muchos comentarios y valoraciones en muchos juegos (relación muchos a muchos entre la tabla "Usuarios" y la tabla "Juegos", resuelta con las tablas intermedias "Comentarios" y "Valoraciones").
- Un usuario puede comprar muchos juegos, y un juego puede ser comprado por muchos usuarios (relación muchos a muchos entre la tabla "Usuarios" y la tabla "Juegos", resuelta con la tabla intermedia