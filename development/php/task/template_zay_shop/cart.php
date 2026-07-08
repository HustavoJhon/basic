<?php
session_start();
require_once "Data/conection.php";
require_once "Data/dataProduct.php";
require_once "Data/dataCart.php";

// Verificar autenticación
if (!isset($_SESSION['usuario_id'])) {
    header("Location: login.php");
    exit();
}

$dataProduct = new DataProduct();
$dataCart = new DataCart();
$mensaje = '';

// Inicializar carrito en sesión
if (!isset($_SESSION['carrito'])) {
    $_SESSION['carrito'] = [];
}

// Agregar producto al carrito
if (isset($_GET['add'])) {
    $producto_id = intval($_GET['add']);
    $producto = $dataProduct->getById($producto_id);
    
    if ($producto && $producto['stock'] > 0) {
        // Verificar si ya existe en el carrito
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
        $mensaje = '<div class="alert alert-success alert-dismissible fade show" role="alert">
                    Producto agregado al carrito
                    <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
                   </div>';
    }
}

// Eliminar producto del carrito
if (isset($_GET['remove'])) {
    $producto_id = intval($_GET['remove']);
    $_SESSION['carrito'] = array_filter($_SESSION['carrito'], function($item) use ($producto_id) {
        return $item['id_producto'] != $producto_id;
    });
    $_SESSION['carrito'] = array_values($_SESSION['carrito']); // Reindexar
    $mensaje = '<div class="alert alert-warning alert-dismissible fade show" role="alert">
                Producto removido del carrito
                <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
               </div>';
}

// Vaciar carrito
if (isset($_GET['clear'])) {
    $_SESSION['carrito'] = [];
    $mensaje = '<div class="alert alert-info alert-dismissible fade show" role="alert">
                Carrito vaciado
                <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
               </div>';
}

// Procesar compra
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['comprar'])) {
    if (empty($_SESSION['carrito'])) {
        $mensaje = '<div class="alert alert-danger">El carrito está vacío</div>';
    } else {
        $pedido_id = $dataCart->savePedido($_SESSION['usuario_id'], $_SESSION['carrito']);
        
        if ($pedido_id) {
            $_SESSION['carrito'] = [];
            header("Location: confirmation.php?pedido=" . $pedido_id);
            exit();
        } else {
            $mensaje = '<div class="alert alert-danger">Error al procesar la compra</div>';
        }
    }
}

// Calcular total
$total = 0;
foreach ($_SESSION['carrito'] as $item) {
    $total += $item['cantidad'] * $item['precio'];
}

include "View/includes/head.php";
include "View/includes/menu.php";
include "View/includes/header.php";
?>

<div class="container py-5">
    <div class="row">
        <div class="col-md-12">
            <h1 class="mb-4">Carrito de Compras</h1>
            <?php echo $mensaje; ?>
        </div>
    </div>

    <div class="row">
        <div class="col-md-8">
            <?php if (!empty($_SESSION['carrito'])): ?>
                <div class="table-responsive">
                    <table class="table table-striped table-hover">
                        <thead class="table-dark">
                            <tr>
                                <th>Producto</th>
                                <th>Precio Unitario</th>
                                <th>Cantidad</th>
                                <th>Subtotal</th>
                                <th>Acciones</th>
                            </tr>
                        </thead>
                        <tbody>
                            <?php foreach ($_SESSION['carrito'] as $item): ?>
                            <tr>
                                <td><?php echo htmlspecialchars($item['nombre']); ?></td>
                                <td>$<?php echo number_format($item['precio'], 2); ?></td>
                                <td><?php echo $item['cantidad']; ?></td>
                                <td><strong>$<?php echo number_format($item['cantidad'] * $item['precio'], 2); ?></strong></td>
                                <td>
                                    <a href="?remove=<?php echo $item['id_producto']; ?>" class="btn btn-sm btn-danger">
                                        <i class="fas fa-trash"></i> Eliminar
                                    </a>
                                </td>
                            </tr>
                            <?php endforeach; ?>
                        </tbody>
                    </table>
                </div>
            <?php else: ?>
                <div class="alert alert-info">
                    <h5>Tu carrito está vacío</h5>
                    <p><a href="shop.php" class="btn btn-primary">Ir a la tienda</a></p>
                </div>
            <?php endif; ?>
        </div>

        <div class="col-md-4">
            <div class="card shadow">
                <div class="card-body">
                    <h5 class="card-title">Resumen del Pedido</h5>
                    <hr>
                    
                    <div class="d-flex justify-content-between mb-2">
                        <span>Subtotal:</span>
                        <strong>$<?php echo number_format($total, 2); ?></strong>
                    </div>
                    
                    <div class="d-flex justify-content-between mb-3">
                        <span>Envío:</span>
                        <strong>GRATIS</strong>
                    </div>
                    
                    <hr>
                    
                    <div class="d-flex justify-content-between mb-4">
                        <span class="fs-5">Total:</span>
                        <strong class="fs-5 text-success">$<?php echo number_format($total, 2); ?></strong>
                    </div>

                    <?php if (!empty($_SESSION['carrito'])): ?>
                        <form method="POST">
                            <button type="submit" name="comprar" class="btn btn-success btn-lg w-100 mb-2">
                                <i class="fas fa-credit-card"></i> Confirmar Compra
                            </button>
                        </form>
                        <a href="?clear" class="btn btn-secondary w-100" onclick="return confirm('¿Vaciar el carrito?')">
                            <i class="fas fa-trash"></i> Vaciar Carrito
                        </a>
                    <?php endif; ?>
                    
                    <a href="shop.php" class="btn btn-outline-primary w-100 mt-2">
                        <i class="fas fa-arrow-left"></i> Continuar Comprando
                    </a>
                </div>
            </div>
        </div>
    </div>
</div>

<?php include "View/includes/footer.php"; ?>

