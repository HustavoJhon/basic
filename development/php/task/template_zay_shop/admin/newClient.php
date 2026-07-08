<?php
session_start();

if (!isset($_SESSION['usuario'])) {
    header("Location: ../login.php");
    exit();
}

require_once "../Data/dataUser.php";

$dataUser = new DataUser();
$mensaje = '';

if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $usuario = trim($_POST['usuario'] ?? '');
    $contrasena = trim($_POST['contrasena'] ?? '');
    $confirmacion = trim($_POST['confirmacion'] ?? '');
    
    // Validación
    if (empty($usuario) || empty($contrasena) || empty($confirmacion)) {
        $mensaje = '<div class="alert alert-danger">Todos los campos son requeridos</div>';
    } elseif ($contrasena !== $confirmacion) {
        $mensaje = '<div class="alert alert-danger">Las contraseñas no coinciden</div>';
    } elseif (strlen($contrasena) < 6) {
        $mensaje = '<div class="alert alert-danger">La contraseña debe tener al menos 6 caracteres</div>';
    } else {
        if ($dataUser->insertUser($usuario, $contrasena)) {
            header("Location: clients.php?success=1");
            exit();
        } else {
            $mensaje = '<div class="alert alert-danger">Error al crear el usuario (posiblemente duplicado)</div>';
        }
    }
}

include "../View/includes/head.php";
include "../View/includes/menu.php";
?>

<div class="container py-5">
    <div class="row justify-content-center">
        <div class="col-md-6">
            <div class="card shadow">
                <div class="card-body">
                    <h2 class="text-center text-success mb-4">Crear Nuevo Usuario</h2>

                    <?php echo $mensaje; ?>

                    <form method="POST">
                        <div class="mb-3">
                            <label class="form-label">Usuario</label>
                            <input type="text" name="usuario" class="form-control" placeholder="nombre_usuario" required>
                        </div>

                        <div class="mb-3">
                            <label class="form-label">Contraseña</label>
                            <input type="password" name="contrasena" class="form-control" placeholder="Mínimo 6 caracteres" required>
                        </div>

                        <div class="mb-3">
                            <label class="form-label">Confirmar Contraseña</label>
                            <input type="password" name="confirmacion" class="form-control" placeholder="Repite la contraseña" required>
                        </div>

                        <div class="d-grid gap-2">
                            <button type="submit" class="btn btn-success btn-lg">Crear Usuario</button>
                            <a href="clients.php" class="btn btn-secondary">Cancelar</a>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </div>
</div>

<?php include "../View/includes/footer.php"; ?>
