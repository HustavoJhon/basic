# Game 
para correr el juego debes seguir las siguientes instrucciones en la terminal

```
cd game
python3 main.py
```

**Arbol de librerias instaladas con pip**
```py
pip freeze
```

### page

1. [gitignore.io](https://gitignore.io)
2. [pyppi.org](https://pypi.org)


## Ambientes virtuales

> encapsulan  cada proyecto y no lo dejan de forma compartida ya que por lo general se necesita diferentes versiones de modulos para nuestro priyecto y al momento de instalar una version desinstala otra y eso genera comfligtos

```python
# verificar donde esta python y pip
which python
which pip
```

```python
# for linux and wsl
apt install -y python3-venv
```

```python
# Poner cada proyecto en su propio ambiente, entrar en cada carpeta.
python3 -m venv env
# Windows
py -m venv venv
```

```python
# Activar el entorno virtual
source env/bin/activate
# windows 
.\env\Scripts\activate
# Desactivar entorno virtual
deactivate
```

## requerements.txt

```python
# generar dependencia y versiones de modulos que necesita para crear requirements.txt
pip freeze > requiremens.txt
# instalar dependencias y versinode de modulos con pip que esta dentro de requirements.txt
pip install -r requirements.txt
```

# App Project
```sh
git clone
cd app
python3 -m venv env 
source env/bin/activate
pip3 install -r requirements.txt
python3 main.py
```
