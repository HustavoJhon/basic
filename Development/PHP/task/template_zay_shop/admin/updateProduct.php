<?php
session_start();

if (!isset($_SESSION['usuario'])) {
    header("Location: ../login.php");
    exit();
}

require_once "../Data/dataProduct.php";

$dataProduct = new DataProduct();
$mensaje = '';
$producto = null;

// Obtener el ID del producto
$id = intval($_GET['id'] ?? 0);
if ($id <= 0) {
    header("Location: products.php");
    exit();
}

$producto = $dataProduct->getById($id);
if (!$producto) {
    header("Location: products.php?error=1");
    exit();
}

// Procesar el formulario
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $nombre = trim($_POST['nombre'] ?? '');
    $descripcion = trim($_POST['descripcion'] ?? '');
    $precio = floatval($_POST['precio'] ?? 0);
    $stock = intval($_POST['stock'] ?? 0);
    
    if (empty($nombre) || empty($descripcion) || $precio <= 0 || $stock < 0) {
        $mensaje = '<div class="alert alert-danger">Todos los campos son requeridos y deben ser válidos</div>';
    } else {
        if ($dataProduct->update($id, $nombre, $descripcion, $precio, $stock)) {
            header("Location: products.php?success=1");
            exit();
        } else {
            $mensaje = '<div class="alert alert-danger">Error al actualizar el producto</div>';
        }
    }
}

include "../View/includes/head.php";
include "../View/includes/menu.php";
?>

<div class="container py-5">
    <div class="row justify-content-center">
        <div class="col-md-8">
            <div class="card shadow">
                <div class="card-body">
                    <h2 class="text-center text-warning mb-4">Editar Producto</h2>

                    <?php echo $mensaje; ?>

                    <form method="POST">
                        <div class="mb-3">
                            <label class="form-label">Nombre del Producto</label>
                            <input type="text" name="nombre" class="form-control" value="<?php echo htmlspecialchars($producto['nombre']); ?>" required>
                        </div>

                        <div class="mb-3">
                            <label class="form-label">Descripción</label>
                            <textarea name="descripcion" class="form-control" rows="4" required><?php echo htmlspecialchars($producto['descripcion']); ?></textarea>
                        </div>

                        <div class="row">
                            <div class="col-md-6 mb-3">
                                <label class="form-label">Precio ($)</label>
                                <input type="number" name="precio" class="form-control" step="0.01" value="<?php echo $producto['precio']; ?>" required>
                            </div>

                            <div class="col-md-6 mb-3">
                                <label class="form-label">Stock</label>
                                <input type="number" name="stock" class="form-control" value="<?php echo $producto['stock']; ?>" required>
                            </div>
                        </div>

                        <div class="d-grid gap-2">
                            <button type="submit" class="btn btn-warning btn-lg">Actualizar Producto</button>
                            <a href="products.php" class="btn btn-secondary">Cancelar</a>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </div>
</div>

<?php include "../View/includes/footer.php"; ?>
