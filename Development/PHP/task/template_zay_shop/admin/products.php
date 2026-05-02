<?php
session_start();

// Verificar autenticación
if (!isset($_SESSION['usuario'])) {
    header("Location: ../login.php");
    exit();
}

require_once "../Data/dataProduct.php";

$dataProduct = new DataProduct();
$productos = $dataProduct->getAll();
$mensaje = '';

// Eliminar producto
if (isset($_GET['delete'])) {
    $id = intval($_GET['delete']);
    if ($dataProduct->delete($id)) {
        $mensaje = '<div class="alert alert-success">Producto eliminado correctamente</div>';
        $productos = $dataProduct->getAll();
    } else {
        $mensaje = '<div class="alert alert-danger">Error al eliminar el producto</div>';
    }
}

include "../View/includes/head.php";
include "../View/includes/menu.php";
?>

<div class="container py-5">
    <div class="row mb-4">
        <div class="col-md-8">
            <h1>Gestión de Productos</h1>
        </div>
        <div class="col-md-4 text-end">
            <a href="newProduct.php" class="btn btn-success btn-lg">
                <i class="fas fa-plus"></i> Nuevo Producto
            </a>
        </div>
    </div>

    <?php echo $mensaje; ?>

    <div class="table-responsive">
        <table class="table table-striped table-hover">
            <thead class="table-dark">
                <tr>
                    <th>ID</th>
                    <th>Nombre</th>
                    <th>Precio</th>
                    <th>Stock</th>
                    <th>Descripción</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <?php if (!empty($productos)): ?>
                    <?php foreach ($productos as $producto): ?>
                    <tr>
                        <td><?php echo $producto['id']; ?></td>
                        <td><?php echo htmlspecialchars($producto['nombre']); ?></td>
                        <td>$<?php echo number_format($producto['precio'], 2); ?></td>
                        <td>
                            <span class="badge <?php echo $producto['stock'] > 0 ? 'bg-success' : 'bg-danger'; ?>">
                                <?php echo $producto['stock']; ?>
                            </span>
                        </td>
                        <td><?php echo substr(htmlspecialchars($producto['descripcion']), 0, 30) . '...'; ?></td>
                        <td>
                            <a href="updateProduct.php?id=<?php echo $producto['id']; ?>" class="btn btn-sm btn-warning">
                                <i class="fas fa-edit"></i> Editar
                            </a>
                            <a href="?delete=<?php echo $producto['id']; ?>" class="btn btn-sm btn-danger" onclick="return confirm('¿Está seguro?')">
                                <i class="fas fa-trash"></i> Eliminar
                            </a>
                        </td>
                    </tr>
                    <?php endforeach; ?>
                <?php else: ?>
                    <tr>
                        <td colspan="6" class="text-center text-muted">No hay productos</td>
                    </tr>
                <?php endif; ?>
            </tbody>
        </table>
    </div>
</div>

<?php include "../View/includes/footer.php"; ?>
