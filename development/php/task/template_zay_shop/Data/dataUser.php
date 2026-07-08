<?php
require_once __DIR__ . "/conection.php";
require_once __DIR__ . "/../Model/user.php";

class DataUser {
    
    /**
     * Registra un nuevo usuario en la base de datos
     * @param string $usuario
     * @param string $pass Contraseña en texto plano (será hasheada)
     * @return bool
     */
    public function insertUser($usuario, $pass) {
        try {
            $conexion = Conexion::conectar();
            
            // Contraseña en texto plano
            $sql = "INSERT INTO clientes (usuario, contrasena, estado) VALUES (?, ?, 1)";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario, $pass]);
            
            return true;
        } catch (PDOException $e) {
            error_log("Error al insertar usuario: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Valida usuario y contraseña
     * @param string $usuario
     * @param string $pass
     * @return bool
     */
    public function validate($usuario, $pass) {
        try {
            $conexion = Conexion::conectar();
            
            // Query con prepared statement
            $sql = "SELECT * FROM clientes WHERE usuario = ? AND contrasena = ? AND estado = 1";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario, $pass]);
            
            $usuario_bd = $stmt->fetch(PDO::FETCH_ASSOC);
            
            if ($usuario_bd) {
                return $usuario_bd;
            }
            
            return false;
        } catch (PDOException $e) {
            error_log("Error al validar usuario: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Obtiene todos los usuarios
     * @return array
     */
    public function getAll() {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT id, usuario, estado FROM clientes";
            $stmt = $conexion->prepare($sql);
            $stmt->execute();
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
        } catch (PDOException $e) {
            error_log("Error al obtener usuarios: " . $e->getMessage());
            return [];
        }
    }
    
    /**
     * Obtiene un usuario por ID
     * @param int $id
     * @return array|false
     */
    public function getById($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT id, usuario, estado FROM clientes WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return $stmt->fetch(PDO::FETCH_ASSOC);
        } catch (PDOException $e) {
            error_log("Error al obtener usuario: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Actualiza usuario
     * @param int $id
     * @param string $usuario
     * @param int $estado
     * @return bool
     */
    public function update($id, $usuario, $estado) {
        try {
            $conexion = Conexion::conectar();
            $sql = "UPDATE clientes SET usuario = ?, estado = ? WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario, $estado, $id]);
            return true;
        } catch (PDOException $e) {
            error_log("Error al actualizar usuario: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Actualiza contraseña
     * @param int $id
     * @param string $pass
     * @return bool
     */
    public function updatePassword($id, $pass) {
        try {
            $conexion = Conexion::conectar();
            $sql = "UPDATE clientes SET contrasena = ? WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$pass, $id]);
            return true;
        } catch (PDOException $e) {
            error_log("Error al actualizar contraseña: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Elimina un usuario (desactiva)
     * @param int $id
     * @return bool
     */
    public function delete($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "UPDATE clientes SET estado = 0 WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return true;
        } catch (PDOException $e) {
            error_log("Error al eliminar usuario: " . $e->getMessage());
            return false;
        }
    }
}
?>
