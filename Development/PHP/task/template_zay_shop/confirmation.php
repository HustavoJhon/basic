<?php
session_start();
require_once "Data/conection.php";
require_once "Data/dataCart.php";

// Verificar autenticación
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

if (!$pedido) {
    header("Location: shop.php");
    exit();
}

include "View/includes/head.php";
include "View/includes/menu.php";
include "View/includes/header.php";
?>

<div class="container py-5">
    <div class="row justify-content-center">
        <div class="col-md-8">
            <!-- Alerta de éxito -->
            <div class="alert alert-success alert-dismissible fade show" role="alert">
                <h4 class="alert-heading">¡Compra Confirmada!</h4>
                <p>Tu pedido ha sido procesado exitosamente. A continuación encontrarás los detalles de tu compra.</p>
                <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
            </div>

            <!-- Detalles del pedido -->
            <div class="card shadow mb-4">
                <div class="card-header bg-success text-white">
                    <h5 class="mb-0">Detalles del Pedido #<?php echo $pedido['id']; ?></h5>
                </div>
                <div class="card-body">
                    <div class="row mb-3">
                        <div class="col-md-6">
                            <p><strong>Fecha:</strong> <?php echo date('d/m/Y H:i', strtotime($pedido['fecha'])); ?></p>
                            <p><strong>Estado:</strong> 
                                <span class="badge bg-info">
                                    <?php echo ucfirst($pedido['estado']); ?>
                                </span>
                            </p>
                        </div>
                        <div class="col-md-6">
                            <p><strong>Usuario:</strong> <?php echo htmlspecialchars($_SESSION['usuario']); ?></p>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Productos comprados -->
            <div class="card shadow mb-4">
                <div class="card-header bg-info text-white">
                    <h5 class="mb-0">Productos Comprados</h5>
                </div>
                <div class="card-body">
                    <div class="table-responsive">
                        <table class="table table-striped">
                            <thead>
                                <tr>
                                    <th>Producto</th>
                                    <th>Cantidad</th>
                                    <th>Precio Unitario</th>
                                    <th>Subtotal</th>
                                </tr>
                            </thead>
                            <tbody>
                                <?php foreach ($pedido['detalles'] as $detalle): ?>
                                <tr>
                                    <td><?php echo htmlspecialchars($detalle['nombre']); ?></td>
                                    <td><?php echo $detalle['cantidad']; ?></td>
                                    <td>$<?php echo number_format($detalle['precio_unitario'], 2); ?></td>
                                    <td><strong>$<?php echo number_format($detalle['cantidad'] * $detalle['precio_unitario'], 2); ?></strong></td>
                                </tr>
                                <?php endforeach; ?>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>

            <!-- Resumen de pago -->
            <div class="card shadow mb-4">
                <div class="card-header bg-primary text-white">
                    <h5 class="mb-0">Resumen de Pago</h5>
                </div>
                <div class="card-body">
                    <div class="d-flex justify-content-between mb-2">
                        <span>Subtotal:</span>
                        <strong>$<?php echo number_format($pedido['total'], 2); ?></strong>
                    </div>
                    <div class="d-flex justify-content-between mb-2">
                        <span>Envío:</span>
                        <strong>GRATIS</strong>
                    </div>
                    <hr>
                    <div class="d-flex justify-content-between fs-5">
                        <span><strong>Total a Pagar:</strong></span>
                        <strong class="text-success">$<?php echo number_format($pedido['total'], 2); ?></strong>
                    </div>
                </div>
            </div>

            <!-- Acciones -->
            <div class="d-grid gap-2">
                <a href="shop.php" class="btn btn-primary btn-lg">
                    <i class="fas fa-shopping-bag"></i> Continuar Comprando
                </a>
                <a href="index.php" class="btn btn-secondary btn-lg">
                    <i class="fas fa-home"></i> Ir al Inicio
                </a>
            </div>
        </div>
    </div>
</div>

<?php include "View/includes/footer.php"; ?>

