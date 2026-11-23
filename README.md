<a id="readme-top"></a>

<div align="center">

<br />
<img src="./assets/ghost.svg" alt="Logo de basic" width="80" height="80" />
<h3>basic</h3>
<p>
  Vault de Markdown para aprendizaje y conocimiento personal,
  con una TUI propia para indexar, explorar, repasar y practicar código.
</p>

[![stars](https://img.shields.io/github/stars/hustavojhon/basic?style=for-the-badge&logo=github&label=Stars&color=F5C518)](https://github.com/hustavojhon/basic/stargazers)
[![CI](https://img.shields.io/github/actions/workflow/status/hustavojhon/basic/ci.yml?branch=main&style=for-the-badge&label=CI)](https://github.com/hustavojhon/basic/actions/workflows/ci.yml)

</div>

---

## Tabla de contenidos

- [Acerca del proyecto](#acerca-del-proyecto)
- [Inicio rápido](#inicio-rápido)
- [Qué incluye](#qué-incluye)
- [Estructura](#estructura)
- [Documentación](#documentación)
- [Tecnologías](#tecnologías)
- [Contenido por sección](#contenido-por-sección)
- [Recursos de aprendizaje](#recursos-de-aprendizaje)
- [Contacto](#contacto)

---

## Acerca del proyecto

Este repositorio es dos cosas a la vez:

1. **Un vault de Markdown** (compatible con Obsidian) donde guardo apuntes de
   arquitectura, desarrollo, Linux, herramientas y aprendizaje en general.
2. **BASIC**, una aplicación de terminal escrita en Rust que indexa ese vault,
   ofrece búsqueda, repaso espaciado, Pomodoro, estadísticas y un laboratorio
   de código (Code Lab) con ejercicios reales de C#.

El principio central es simple: **Markdown es la fuente de verdad; SQLite es
solo un índice y un registro de estado**. La aplicación nunca reescribe una
nota existente y la base de datos puede reconstruirse escaneando el vault.

---

## Inicio rápido

```bash
cd tui
cargo build --release
./target/release/basic config init
./target/release/basic index
./target/release/basic tui
```

La guía completa de comandos, atajos, configuración y arquitectura está en
[`tui/README.md`](tui/README.md). Para verificar el entorno sin abrir la
interfaz: `basic doctor --strict`.

---

## Qué incluye

### BASIC TUI (F2–F22)

- Interfaz Ratatui con tema Gruvbox, adaptable a terminales estrechas.
- Indexado incremental con FTS5 e insensible a acentos.
- Explorador, búsqueda difusa y de contenido, visor Markdown y creación
  segura de notas.
- Learning Mode, Pomodoro con persistencia y estadísticas con heatmap.
- Repaso tipo SM-2 para notas y conceptos.

### Code Lab

- Catálogo declarativo de lecciones y ejercicios de C# con semilla idempotente.
- Editor de código en la TUI, borradores y ejecución síncrona con el SDK local
  en un workspace temporal fuera del vault.
- Casos de prueba visibles y ocultos, timeouts y solución solo bajo petición.
- Dificultad, progreso de lecciones, intentos y conceptos con repaso SM-2.
- Notas enlazadas en el vault con navegación inversa al origen.
- Objetivos diarios configurables e informes Markdown/JSON.

Los detalles de cada fase están en [`docs/design/`](docs/design/).

---

## Estructura

```text
basic/
├── tui/                   # aplicación BASIC (Rust, Ratatui, SQLite)
│   ├── src/               # dominio, app, infra, ui y code_lab
│   ├── migrations/        # esquema SQLite versionado (v1–v5)
│   ├── tests/             # pruebas de extremo a extremo del CLI
│   └── README.md          # guía de uso y arquitectura
├── docs/design/           # auditorías y decisiones de cada fase
├── arquitecture/          # patrones y principios de arquitectura
├── development/           # lenguajes y frameworks
│   ├── api/               # diseño y consumo de APIs
│   ├── java/              # fundamentos de Java
│   ├── javascript/        # JavaScript moderno (ES6+)
│   └── rocketbot/         # automatización con Rocketbot
├── learning/              # tecnologías emergentes y especializadas
│   ├── blockchain/        # fundamentos de blockchain
│   ├── latex/             # composición tipográfica
│   └── resources/         # recursos de aprendizaje
├── tools/                 # herramientas de desarrollo
│   ├── docker/            # contenedores y Docker Compose
│   ├── git/               # control de versiones
│   └── github/            # workflows y GitHub Actions
├── linux/                 # administración Linux
│   ├── command/           # comandos esenciales
│   ├── security/          # seguridad en sistemas
│   ├── termux/            # Termux en Android
│   └── vim/               # editor Vim/Neovim
├── resources/             # material de apoyo
├── assets/                # imágenes del repositorio
└── .github/workflows/     # CI: fmt, clippy, tests y build release
```

---

## Documentación

| Documento | Contenido |
|---|---|
| [`tui/README.md`](tui/README.md) | Instalación, comandos, atajos, configuración y arquitectura |
| [`docs/design/F0-audit.md`](docs/design/F0-audit.md) | Auditoría inicial y arquitectura aprobada |
| [`docs/design/F12-audit.md`](docs/design/F12-audit.md) | Estabilización de F2–F11 y hoja de ruta |
| [`docs/design/`](docs/design/) | Diseño de las fases F13–F22 |

---

## Tecnologías

<div align="left">

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white&labelColor=DEA584)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=flat-square&logo=sqlite&logoColor=white&labelColor=003B57)
![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=flat-square&logo=javascript&logoColor=black&labelColor=F7DF1E)
![Java](https://img.shields.io/badge/Java-ED8B00?style=flat-square&logo=openjdk&logoColor=white&labelColor=ED8B00)
![Go](https://img.shields.io/badge/Go-00ADD8?style=flat-square&logo=go&logoColor=white&labelColor=00ADD8)
![C#](https://img.shields.io/badge/C%23-239120?style=flat-square&logo=csharp&logoColor=white&labelColor=239120)
![Bash](https://img.shields.io/badge/Bash-4EAA25?style=flat-square&logo=gnu-bash&logoColor=white&labelColor=4EAA25)

</div>

<div align="left">

![Ratatui](https://img.shields.io/badge/Ratatui-7B3FE4?style=flat-square&logo=ratatui&logoColor=white&labelColor=7B3FE4)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white&labelColor=2496ED)
![Git](https://img.shields.io/badge/Git-F05032?style=flat-square&logo=git&logoColor=white&labelColor=F05032)
![Vim](https://img.shields.io/badge/Vim-019733?style=flat-square&logo=vim&logoColor=white&labelColor=019733)
![Neovim](https://img.shields.io/badge/Neovim-57A64A?style=flat-square&logo=neovim&logoColor=white&labelColor=57A64A)
![Obsidian](https://img.shields.io/badge/Obsidian-7C3AED?style=flat-square&logo=obsidian&logoColor=white&labelColor=7C3AED)

</div>

---

## Contenido por sección

### Arquitecture

Patrones de diseño, principios SOLID, arquitectura hexagonal y mejores
prácticas para estructurar código escalable.

### Development

| Tema | Descripción |
|---|---|
| **API** | REST, GraphQL, consumo de servicios RESTful |
| **JAVA** | Fundamentos, POO, Spring Boot |
| **JAVASCRIPT** | Desde cero hasta conceptos avanzados (ES6+) |
| **ROCKETBOT** | Automatización de tareas y procesos |

### Learning

- **Blockchain**: contratos inteligentes, Ethereum, wallets.
- **LaTeX**: documentación técnica profesional.
- **Recursos**: enlaces útiles para seguir aprendiendo.

### Tools

- **Docker**: contenedores, Compose, networking.
- **Git**: versionado, branching, estrategias de merge.
- **GitHub**: Actions, Codespaces, colaboración.

### Linux

- **Comandos**: los esenciales del día a día.
- **Seguridad**: hardening, permisos, SSH.
- **Termux**: Linux en Android.
- **Vim/Neovim**: productividad en el editor.

---

## Recursos de aprendizaje

- [Project Based Learning](https://github.com/practical-tutorials/project-based-learning) — aprende construyendo proyectos reales.
- [Coddy.tech](https://coddy.tech/) — cursos interactivos.
- [Guía de entrevistas de programación](https://github.com/DevCaress/guia-entrevistas-de-programacion) — preparación técnica.
- [Programación en Java](https://www.politecnicodecolombia.edu.co/diplomados-virtuales-gratis/escuela-de-informacion/programacion-en-java.html) — diplomados virtuales gratis.

---

## Contacto

[hustavojhon](https://github.com/hustavojhon)

<div align="center">

**[⬆ Volver arriba](#readme-top)**

</div>
