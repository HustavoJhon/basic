# Ramas y Fusion de Cambios: branch, merge, switch y checkout

**Resumen**

El uso de ramas en Git permite trabajar en un entorno aislado sin interferir con otros, facilitando la organización y el control del proyecto. Aprender a crear, gestionar y fusionar ramas optimiza la colaboración y ayuda a mantener la limpieza en el historial de cambios.

## ¿Por qué son útiles las ramas en Git?
Las ramas son una herramienta que permite trabajar en tareas específicas sin alterar la rama principal. Entre sus ventajas se encuentran:

- Aislamiento de cambios individuales.
- Posibilidad de desechar una rama sin afectar la principal.
- Organización de actividades múltiples en diferentes ramas.

## ¿Cómo verificar la rama actual?
Para saber en qué rama estás trabajando, ejecuta:

```git 
git branch
```
El asterisco (*) indica la rama activa. Inicialmente, suele ser `main`, pero al crear más ramas, la lista crecerá, permitiéndote ver todas las disponibles y cuál es la actual.

## ¿Cómo crear una nueva rama en Git?
La creación de ramas permite desarrollar sin riesgo en paralelo. Para crear y moverte a una nueva rama, usa:
```git
git checkout -b 
```
Por ejemplo, `git checkout -b Amin` crea y mueve a la rama Amin. Puedes verificar que estás en esta rama ejecutando `git branch`.

## ¿Cómo agregar y confirmar cambios en una rama?
Dentro de una nueva rama, los archivos se editan y confirman sin que impacten otras ramas. Sigue estos pasos para agregar y confirmar:

1. Crea o edita un archivo.
2. Añádelo con:
```git
git add .
```
3. Confirma el cambio:
```git
git commit -m "mensaje de confirmación"
```
Los cambios ahora son parte de la rama en la que trabajas y no afectan la principal.

## ¿Cómo fusionar cambios de una rama secundaria a la principal?
Para unificar el trabajo en la rama principal:

`. Cambia a la rama principal:
```git
git switch main
```
_Nota:_ Puedes usar también `git checkout main`.

2. Fusiona la rama secundaria:
```git
git merge 
```
Git indicará que el proceso fue exitoso y actualizará el contenido en la rama `main` con los cambios de la rama secundaria.

## ¿Por qué es importante eliminar ramas que ya no se usan?
Una vez fusionada una rama, es buena práctica eliminarla para evitar desorden. Hazlo con:
```git
git branch -d 
```
Eliminar ramas que ya cumplieron su propósito previene conflictos y mantiene el entorno de trabajo limpio y organizado.

**Lecturas recomendadas**

- [Git - git-branch Documentation](https://git-scm.com/docs/git-branch)

- [Git - git-merge Documentation](https://git-scm.com/docs/git-merge)

- [Git - git-switch Documentation](https://git-scm.com/docs/git-switch)

- [Git - git-checkout Documentation](https://git-scm.com/docs/git-checkout )