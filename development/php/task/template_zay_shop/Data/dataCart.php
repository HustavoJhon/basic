<?php
require_once __DIR__ . "/conection.php";

class DataCart {
    
    /**
     * Guarda un pedido en la BD
     * @param int $usuario_id
     * @param array $items Carrito con [id_producto, cantidad, precio]
     * @return int|false ID del pedido o false si falla
     */
    public function savePedido($usuario_id, $items) {
        try {
            $conexion = Conexion::conectar();
            
            // Iniciar transacción
            $conexion->beginTransaction();
            
            $total = 0;
            foreach ($items as $item) {
                $total += $item['cantidad'] * $item['precio'];
            }
            
            // Insertar pedido
            $sql = "INSERT INTO pedidos (usuario_id, fecha, total, estado) 
                    VALUES (?, NOW(), ?, 'pendiente')";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario_id, $total]);
            
            $pedido_id = $conexion->lastInsertId();
            
            // Insertar detalles del pedido
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
    
    /**
     * Obtiene un pedido completo
     * @param int $pedido_id
     * @return array|false
     */
    public function getPedido($pedido_id) {
        try {
            $conexion = Conexion::conectar();
            
            // Obtener pedido
            $sql = "SELECT * FROM pedidos WHERE id = ?";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$pedido_id]);
            $pedido = $stmt->fetch(PDO::FETCH_ASSOC);
            
            if (!$pedido) {
                return false;
            }
            
            // Obtener detalles
            $sql_detalle = "SELECT dp.*, p.nombre FROM detalle_pedido dp
                           JOIN productos p ON p.id = dp.producto_id
                           WHERE dp.pedido_id = ?";
            $stmt_detalle = $conexion->prepare($sql_detalle);
            $stmt_detalle->execute([$pedido_id]);
            $detalles = $stmt_detalle->fetchAll(PDO::FETCH_ASSOC);
            
            $pedido['detalles'] = $detalles;
            return $pedido;
        } catch (PDOException $e) {
            error_log("Error al obtener pedido: " . $e->getMessage());
            return false;
        }
    }
    
    /**
     * Obtiene todos los pedidos de un usuario
     * @param int $usuario_id
     * @return array
     */
    public function getPedidosByUsuario($usuario_id) {
        try {
            $conexion = Conexion::conectar();
            $sql = "SELECT * FROM pedidos WHERE usuario_id = ? ORDER BY fecha DESC";
            $stmt = $conexion->prepare($sql);
            $stmt->execute([$usuario_id]);
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
        } catch (PDOException $e) {
            error_log("Error al obtener pedidos: " . $e->getMessage());
            return [];
        }
    }
}
?>