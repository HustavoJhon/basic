<?php
session_start();

// Verificar si el usuario está logeado
if (!isset($_SESSION['usuario'])) {
    header("Location: ../login.php");
    exit();
}

// Aquí se podría verificar si es admin, pero por ahora cualquier usuario logeado puede entrar
include "../View/includes/head.php";
include "../View/includes/menu.php";
?>

<div class="container py-5">
    <div class="row mb-4">
        <div class="col-md-12">
            <h1 class="mb-4">Panel de Administración</h1>
            <p class="text-muted">Bienvenido, <strong><?php echo htmlspecialchars($_SESSION['usuario']); ?></strong></p>
        </div>
    </div>

    <div class="row">
        <!-- Gestión de Productos -->
        <div class="col-md-6 mb-4">
            <div class="card shadow">
                <div class="card-body text-center">
                    <h5 class="card-title text-success mb-3">
                        <i class="fas fa-box"></i> Gestión de Productos
                    </h5>
                    <p class="card-text">Crear, editar y eliminar productos de la tienda</p>
                    <a href="products.php" class="btn btn-success">Ir a Productos</a>
                </div>
            </div>
        </div>

        <!-- Gestión de Usuarios -->
        <div class="col-md-6 mb-4">
            <div class="card shadow">
                <div class="card-body text-center">
                    <h5 class="card-title text-info mb-3">
                        <i class="fas fa-users"></i> Gestión de Usuarios
                    </h5>
                    <p class="card-text">Ver, editar y eliminar usuarios del sistema</p>
                    <a href="clients.php" class="btn btn-info">Ir a Usuarios</a>
                </div>
            </div>
        </div>
    </div>
</div>

<?php include "../View/includes/footer.php"; ?>
