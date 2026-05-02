<?php
require_once __DIR__ . "/conection.php";

class DataProduct {
    
    /**
     * Obtiene todos los productos
     * @return array
     */
    public function getAll() {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM productos ORDER BY id DESC";
            $stmt = $conexion->prepare($sql);
            $stmt->execute();
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
        } catch (PDOException $e) {
            error_log("Error al obtener productos: " . $e->getMessage());
            return [];
        }
    }
    
    /**
     * Obtiene un producto por ID
     * @param int $id
     * @return array|false
     */
    public function getById($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM productos WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return $stmt->fetch(PDO::FETCH_ASSOC);
        } catch (PDOException $e) {
            error_log("Error al obtener producto: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Crea un nuevo producto
     * @param string $nombre
     * @param string $descripcion
     * @param float $precio
     * @param int $stock
     * @param string $imagen
     * @return bool
     */
    public function create($nombre, $descripcion, $precio, $stock, $imagen = null) {
        try {
            $conexion = Conexion::conectar();
            $sql = "INSERT INTO productos (nombre, descripcion, precio, stock, imagen) 
                    VALUES (?, ?, ?, ?, ?)";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$nombre, $descripcion, $precio, $stock, $imagen]);
            return true;
        } catch (PDOException $e) {
            error_log("Error al crear producto: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Actualiza un producto
     * @param int $id
     * @param string $nombre
     * @param string $descripcion
     * @param float $precio
     * @param int $stock
     * @param string $imagen
     * @return bool
     */
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
        } catch (PDOException $e) {
            error_log("Error al actualizar producto: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Elimina un producto
     * @param int $id
     * @return bool
     */
    public function delete($id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "DELETE FROM productos WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$id]);
            return true;
        } catch (PDOException $e) {
            error_log("Error al eliminar producto: " . $e->getMessage());
            return false;
        }
    }
}
?>