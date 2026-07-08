<?php
class Conexion {
    public static function conectar() {
        // Configuración compatible con Docker y local
        $host = getenv('MYSQL_HOST') ?: "localhost";
        $db = getenv('MYSQL_DATABASE') ?: "storeonline";
        $user = getenv('MYSQL_USER') ?: "root";
        $pass = getenv('MYSQL_PASSWORD') ?: "";

        try {
            $conexion = new PDO(
                "mysql:host=$host;dbname=$db;charset=utf8mb4",
                $user,
                $pass,
                [PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION]
            );
            return $conexion;
        } catch (PDOException $e) {
            die("Error de Conexión: " . $e->getMessage());
        }
    }
}
?>