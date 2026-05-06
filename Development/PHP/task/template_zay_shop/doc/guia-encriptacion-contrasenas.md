# Guía: Encriptación de Contraseñas en PHP

## 1. ¿Cómo funcionaba la encriptación original?

El sistema usaba **bcrypt**, que es el algoritmo de hashing recomendado por PHP para contraseñas. Funcionaba en dos partes:

### Al registrar un usuario (insertUser)

```php
$passHash = password_hash($pass, PASSWORD_BCRYPT);
```

- `password_hash()` toma la contraseña en texto plano (ej: `123456`)
- Genera un **hash irreversible** con un **salt aleatorio** integrado
- El resultado se ve así: `$2y$10$wH8Q0PqQxgX6k6YyYg4T5e6l9Z0g7zZ8e5d1Yw0lXq7mR5Q8vQZbK`
- Cada vez que hasheás `123456`, obtenés un hash **distinto** por el salt

### Al validar el login (validate)

```php
if ($usuario_bd && password_verify($pass, $usuario_bd['contrasena'])) {
    return $usuario_bd;
}
```

- `password_verify()` compara la contraseña que ingresó el usuario con el hash guardado
- Extrae el salt del hash y hace la comparación de forma segura

### Al actualizar contraseña (updatePassword)

```php
$passHash = password_hash($pass, PASSWORD_BCRYPT);
```

- Mismo principio: hashea la nueva contraseña antes de guardarla

### Resumen visual del flujo original

```
┌─────────────────┐
│  Usuario escribe │
│  "123456"        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     password_hash()     ┌──────────────────────┐
│  Texto plano:   │ ──────────────────────► │  Hash bcrypt:        │
│  "123456"       │                         │  $2y$10$wH8Q0Pq...  │
└─────────────────┘                         └──────────┬───────────┘
                                                       │
                                                       ▼
                                               ┌──────────────────────┐
                                               │  Se guarda en la BD  │
                                               └──────────────────────┘
```

**Importante:** bcrypt es **irreversible**. No podés "desencriptar" un hash. Por eso el login usa `password_verify()` que compara sin necesidad de revertir.

---

## 2. ¿Qué se cambió para desactivar la encriptación?

Se modificaron **3 funciones** en `Data/dataUser.php`:

### Cambio 1: insertUser - Ya no hashea

| Antes | Después |
|-------|---------|
| `$passHash = password_hash($pass, PASSWORD_BCRYPT);` | Contraseña directa |
| `$stmt->execute([$usuario, $passHash]);` | `$stmt->execute([$usuario, $pass]);` |

### Cambio 2: validate - Compara en texto plano

| Antes | Después |
|-------|---------|
| Query solo por usuario, luego `password_verify()` | Query filtra por usuario **Y** contraseña directo |
| `WHERE usuario = ?` + `password_verify($pass, ...)` | `WHERE usuario = ? AND contrasena = ?` |

### Cambio 3: updatePassword - Ya no hashea

| Antes | Después |
|-------|---------|
| `$passHash = password_hash($pass, PASSWORD_BCRYPT);` | Contraseña directa |
| `$stmt->execute([$passHash, $id]);` | `$stmt->execute([$pass, $id]);` |

---

## 3. ¿Cómo volver a la encriptación original?

Si en algún momento querés reactivar bcrypt, hacé estos cambios:

### Paso 1: Restaurar `Data/dataUser.php`

**Función `insertUser`** (línea ~13):
```php
public function insertUser($usuario, $pass) {
    try {
        $conexion = Conexion::conectar();
        $passHash = password_hash($pass, PASSWORD_BCRYPT);
        $sql = "INSERT INTO clientes (usuario, contrasena, estado) VALUES (?, ?, 1)";
        $stmt = $conexion->prepare($sql);
        $stmt->execute([$usuario, $passHash]);
        return true;
    } catch (PDOException $e) {
        error_log("Error al insertar usuario: " . $e->getMessage());
        return false;
    }
}
```

**Función `validate`** (línea ~38):
```php
public function validate($usuario, $pass) {
    try {
        $conexion = Conexion::conectar();
        $sql = "SELECT * FROM clientes WHERE usuario = ? AND estado = 1";
        $stmt = $conexion->prepare($sql);
        $stmt->execute([$usuario]);
        $usuario_bd = $stmt->fetch(PDO::FETCH_ASSOC);
        if ($usuario_bd && password_verify($pass, $usuario_bd['contrasena'])) {
            return $usuario_bd;
        }
        return false;
    } catch (PDOException $e) {
        error_log("Error al validar usuario: " . $e->getMessage());
        return false;
    }
}
```

**Función `updatePassword`** (línea ~121):
```php
public function updatePassword($id, $pass) {
    try {
        $conexion = Conexion::conectar();
        $passHash = password_hash($pass, PASSWORD_BCRYPT);
        $sql = "UPDATE clientes SET contrasena = ? WHERE id = ?";
        $stmt = $conexion->prepare($sql);
        $stmt->execute([$passHash, $id]);
        return true;
    } catch (PDOException $e) {
        error_log("Error al actualizar contraseña: " . $e->getMessage());
        return false;
    }
}
```

### Paso 2: Regenerar los hashes en la base de datos

Las contraseñas que están en texto plano **ya no sirven**. Necesitás generar un nuevo hash bcrypt. Podés hacerlo desde PHP:

```php
// Desde una terminal o script
<?php
echo password_hash('123456', PASSWORD_BCRYPT);
?>
```

Ese hash lo ponés en el SQL:

```sql
UPDATE clientes SET contrasena = '$2y$10$el_hash_generado' WHERE usuario = 'admin';
```

### Paso 3: Reconstruir los contenedores

```bash
docker-compose down -v
docker-compose up --build -d
```

---

## 4. Comparación de seguridad

| Aspecto | Con bcrypt | Sin encriptación |
|---------|-----------|------------------|
| Contraseña visible en la BD | No (hash) | Sí (texto plano) |
| Si filtran la BD | Seguro | Comprometida |
| Login más lento | Sí (~100ms) | Instantáneo |
| Apto para producción | Sí | No |
