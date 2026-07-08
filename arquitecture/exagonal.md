# Arquitectura Hexagonal en .NET

## ¿Qué es la Arquitectura Hexagonal?
La **Arquitectura Hexagonal**, también conocida como **Clean Architecture**, es un patrón de diseño de software que busca desacoplar la lógica de negocio de los detalles de implementación. Promueve la independencia de los frameworks y la facilidad para realizar pruebas.

## Principios Clave
- **Independencia de la Tecnología**: La lógica de negocio no depende de frameworks, bases de datos u otros detalles externos.
- **Facilidad de Pruebas**: Permite probar la lógica de negocio sin depender de interfaces externas.
- **Separación de Responsabilidades**: Divide la aplicación en capas, cada una con responsabilidades específicas.

## Capas de la Arquitectura Hexagonal

### 1. Dominio
- **Descripción**: El núcleo de la aplicación que contiene las reglas de negocio, entidades y lógica central.
- **Características**:
  - Sin dependencias de otras capas.
  - Contiene entidades y objetos de valor.

### 2. Aplicación
- **Descripción**: Define las reglas y casos de uso específicos de la aplicación, gestionando la interacción entre el dominio y la infraestructura.
- **Características**:
  - Contiene servicios de aplicación que coordinan las operaciones.
  - Define interfaces que deben ser implementadas por la infraestructura.

### 3. Infraestructura
- **Descripción**: Proporciona implementaciones concretas para las interfaces definidas en las capas de dominio y aplicación.
- **Características**:
  - Incluye componentes como repositorios y servicios externos.
  - Maneja las interacciones con tecnologías externas.

### 4. API
- **Descripción**: Capa que expone los endpoints para que los clientes (aplicaciones web, móviles, etc.) interactúen con la aplicación.
- **Características**:
  - Conecta las interfaces externas con el núcleo de la aplicación.
  - Utiliza métodos HTTP para gestionar solicitudes (GET, POST, PUT, DELETE).

## Ejemplo de Estructura de Directorios en .NET
```plaintext
src/
├── MyApp.Domain/
│   ├── Entities/
│   │   └── User.cs
│   ├── ValueObjects/
│   │   └── Email.cs
│   └── Interfaces/
│       └── IUserRepository.cs
├── MyApp.Application/
│   ├── Services/
│   │   └── UserService.cs
│   ├── UseCases/
│   │   └── RegisterUser.cs
│   └── DTOs/
│       └── UserDto.cs
├── MyApp.Infrastructure/
│   ├── Repositories/
│   │   └── UserRepository.cs
│   └── ExternalServices/
│       └── EmailService.cs
└── MyApp.API/
    ├── Controllers/
    │   └── UserController.cs
    └── Middlewares/
        └── ErrorHandlingMiddleware.cs
