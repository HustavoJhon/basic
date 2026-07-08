<?php
require_once __DIR__ . "/../../Data/dataProduct.php";

$dataProduct = new DataProduct();
$productos = $dataProduct->getAll();
?>

<div class="container py-5">
    <div class="row">
        <div class="col-lg-12">
            <h1 class="h2 pb-4">Nuestros Productos</h1>
        </div>
    </div>

    <div class="row">
        <?php if (!empty($productos)): ?>
            <?php foreach ($productos as $producto): ?>
            <div class="col-md-4 mb-4">
                <div class="card product-wap shadow-sm">
                    <div class="card rounded-0 position-relative overflow-hidden" style="height: 250px; background-color: #f5f5f5;">
                        <img class="card-img rounded-0 img-fluid w-100 h-100" src="View/img/shop_01.jpg" alt="<?php echo htmlspecialchars($producto['nombre']); ?>" style="object-fit: cover;">
                        <div class="card-img-overlay rounded-0 product-overlay d-flex align-items-center justify-content-center">
                            <ul class="list-unstyled">
                                <li>
                                    <?php if ($producto['stock'] > 0): ?>
                                        <a class="btn btn-success text-white" href="cart.php?add=<?php echo $producto['id']; ?>">
                                            <i class="fas fa-cart-plus"></i> Agregar
                                        </a>
                                    <?php else: ?>
                                        <button class="btn btn-secondary text-white" disabled>
                                            <i class="fas fa-ban"></i> Sin Stock
                                        </button>
                                    <?php endif; ?>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div class="card-body">
                        <h5 class="card-title"><?php echo htmlspecialchars($producto['nombre']); ?></h5>
                        <p class="card-text text-muted small">
                            <?php echo substr(htmlspecialchars($producto['descripcion']), 0, 60) . '...'; ?>
                        </p>
                        <div class="d-flex justify-content-between align-items-center mt-3">
                            <span class="h5 text-success mb-0">$<?php echo number_format($producto['precio'], 2); ?></span>
                            <span class="badge <?php echo $producto['stock'] > 0 ? 'bg-success' : 'bg-danger'; ?>">
                                <?php echo $producto['stock'] > 0 ? 'En stock' : 'Sin stock'; ?>
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            <?php endforeach; ?>
        <?php else: ?>
            <div class="col-md-12">
                <div class="alert alert-info text-center py-5">
                    <h4>No hay productos disponibles</h4>
                    <p>Por favor, intenta más tarde.</p>
                </div>
            </div>
        <?php endif; ?>
    </div>
</div>
