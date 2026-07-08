<?php
session_start();

if (!isset($_SESSION['usuario'])) {
    header("Location: ../login.php");
    exit();
}

require_once "../Data/dataUser.php";

$dataUser = new DataUser();
$mensaje = '';
$usuario = null;

// Obtener el ID del usuario
$id = intval($_GET['id'] ?? 0);
if ($id <= 0) {
    header("Location: clients.php");
    exit();
}

$usuario = $dataUser->getById($id);
if (!$usuario) {
    header("Location: clients.php?error=1");
    exit();
}

// Procesar el formulario
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $nuevo_usuario = trim($_POST['usuario'] ?? '');
    $nuevo_estado = intval($_POST['estado'] ?? 0);
    $nueva_contrasena = trim($_POST['contrasena'] ?? '');
    
    if (empty($nuevo_usuario)) {
        $mensaje = '<div class="alert alert-danger">El usuario es requerido</div>';
    } else {
        // Si se proporciona contraseña, actualizarla
        if (!empty($nueva_contrasena)) {
            if (strlen($nueva_contrasena) < 6) {
                $mensaje = '<div class="alert alert-danger">La contraseña debe tener al menos 6 caracteres</div>';
            } else {
                $dataUser->updatePassword($id, $nueva_contrasena);
            }
        }
        
        // Siempre actualizar usuario y estado si no hay error
        if (empty($mensaje) && $dataUser->update($id, $nuevo_usuario, $nuevo_estado)) {
            header("Location: clients.php?success=1");
            exit();
        } elseif (empty($mensaje)) {
            $mensaje = '<div class="alert alert-danger">Error al actualizar el usuario</div>';
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
                    <h2 class="text-center text-warning mb-4">Editar Usuario</h2>

                    <?php echo $mensaje; ?>

                    <form method="POST">
                        <div class="mb-3">
                            <label class="form-label">Usuario</label>
                            <input type="text" name="usuario" class="form-control" value="<?php echo htmlspecialchars($usuario['usuario']); ?>" required>
                        </div>

                        <div class="mb-3">
                            <label class="form-label">Nueva Contraseña (opcional)</label>
                            <input type="password" name="contrasena" class="form-control" placeholder="Dejar vacío para no cambiar">
                            <small class="text-muted">Mínimo 6 caracteres si desea cambiar</small>
                        </div>

                        <div class="mb-3">
                            <label class="form-label">Estado</label>
                            <select name="estado" class="form-control">
                                <option value="1" <?php echo $usuario['estado'] == 1 ? 'selected' : ''; ?>>Activo</option>
                                <option value="0" <?php echo $usuario['estado'] == 0 ? 'selected' : ''; ?>>Inactivo</option>
                            </select>
                        </div>

                        <div class="d-grid gap-2">
                            <button type="submit" class="btn btn-warning btn-lg">Actualizar Usuario</button>
                            <a href="clients.php" class="btn btn-secondary">Cancelar</a>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </div>
</div>

<?php include "../View/includes/footer.php"; ?>
