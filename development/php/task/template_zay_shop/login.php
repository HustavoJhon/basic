<?php
session_start();
include "View/includes/header.php";
include "View/includes/head.php";
?>

<div class="container py-5">
    <div class="row justify-content-center">

        <div class="col-md-6 col-lg-5">
            <div class="card shadow product-wap">

                <div class="card-body">
                    <h2 class="text-center text-success mb-4">Iniciar Sesión</h2>

                    <form action="Controller/userController.php" method="POST">

                        <div class="mb-3">
                            <label class="form-label">Usuario</label>
                            <input type="text" name="usuario" class="form-control" placeholder="Ingrese su usuario" required>
                        </div>

                        <div class="mb-3">
                            <label class="form-label">Contraseña</label>
                            <input type="password" name="contrasena" class="form-control" placeholder="Ingrese su contraseña" required>
                        </div>

                        <div class="d-grid">
                            <button type="submit" class="btn btn-success btn-lg">
                                Ingresar
                            </button>
                        </div>

                    </form>

                    <?php if (isset($_GET['error'])): ?>
                        <div class="alert alert-danger mt-3 text-center">
                            Usuario o contraseña incorrectos
                        </div>
                    <?php endif; ?>

                </div>

            </div>
        </div>

    </div>
</div>

<?php
include "View/includes/footer.php";
?>