<?php
session_start();

require_once __DIR__ . "/../Data/dataUser.php";

// Verificar que se recibieron los datos correctos
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['usuario']) && isset($_POST['contrasena'])) {
    
    $usuario = trim($_POST['usuario']);
    $contrasena = trim($_POST['contrasena']);
    
    // Validación básica
    if (empty($usuario) || empty($contrasena)) {
        header("Location: ../login.php?error=1");
        exit();
    }
    
    // Instanciar la clase de datos
    $dataUser = new DataUser();
    
    // Validar usuario
    $usuario_bd = $dataUser->validate($usuario, $contrasena);
    
    if ($usuario_bd) {
        // Crear sesión
        $_SESSION['usuario_id'] = $usuario_bd['id'];
        $_SESSION['usuario'] = $usuario_bd['usuario'];
        $_SESSION['estado'] = $usuario_bd['estado'];
        
        // Redirigir al inicio
        header("Location: ../index.php?login=success");
        exit();
    } else {
        // Error de autenticación - log para debug
        error_log("Login fallido para usuario: $usuario");
        header("Location: ../login.php?error=1");
        exit();
    }
} else {
    // Acceso directo sin POST
    header("Location: ../login.php");
    exit();
}
?>