<?php
session_start();

if (!isset($_SESSION['usuario'])) {
    header("Location: ../login.php");
    exit();
}

require_once "../Data/dataUser.php";

$dataUser = new DataUser();
$usuarios = $dataUser->getAll();
$mensaje = '';

// Eliminar usuario
if (isset($_GET['delete'])) {
    $id = intval($_GET['delete']);
    if ($dataUser->delete($id)) {
        $mensaje = '<div class="alert alert-success">Usuario eliminado correctamente</div>';
        $usuarios = $dataUser->getAll();
    } else {
        $mensaje = '<div class="alert alert-danger">Error al eliminar el usuario</div>';
    }
}

include "../View/includes/head.php";
include "../View/includes/menu.php";
?>

<div class="container py-5">
    <div class="row mb-4">
        <div class="col-md-8">
            <h1>Gestión de Usuarios</h1>
        </div>
        <div class="col-md-4 text-end">
            <a href="newClient.php" class="btn btn-success btn-lg">
                <i class="fas fa-plus"></i> Nuevo Usuario
            </a>
        </div>
    </div>

    <?php echo $mensaje; ?>

    <div class="table-responsive">
        <table class="table table-striped table-hover">
            <thead class="table-dark">
                <tr>
                    <th>ID</th>
                    <th>Usuario</th>
                    <th>Estado</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <?php if (!empty($usuarios)): ?>
                    <?php foreach ($usuarios as $usuario): ?>
                    <tr>
                        <td><?php echo $usuario['id']; ?></td>
                        <td><?php echo htmlspecialchars($usuario['usuario']); ?></td>
                        <td>
                            <span class="badge <?php echo $usuario['estado'] == 1 ? 'bg-success' : 'bg-danger'; ?>">
                                <?php echo $usuario['estado'] == 1 ? 'Activo' : 'Inactivo'; ?>
                            </span>
                        </td>
                        <td>
                            <a href="updateClient.php?id=<?php echo $usuario['id']; ?>" class="btn btn-sm btn-warning">
                                <i class="fas fa-edit"></i> Editar
                            </a>
                            <a href="?delete=<?php echo $usuario['id']; ?>" class="btn btn-sm btn-danger" onclick="return confirm('¿Está seguro?')">
                                <i class="fas fa-trash"></i> Eliminar
                            </a>
                        </td>
                    </tr>
                    <?php endforeach; ?>
                <?php else: ?>
                    <tr>
                        <td colspan="4" class="text-center text-muted">No hay usuarios</td>
                    </tr>
                <?php endif; ?>
            </tbody>
        </table>
    </div>
</div>

<?php include "../View/includes/footer.php"; ?>
