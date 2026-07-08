# Configuration

1. Download git 

2. Configuration
- global, local, system
> local estara como prioridad 

```
git config --system 
git config --local 
git config --global

#example for name 
git config --global user.name "username"
git config --global user.email "username@gmail"

# view all config 
git config --list

# view global list 
git config --global --list 

git config --local --list #solo si tenemos un repo

# config editor 
git config --global core.editor "code --wait" #vscode --wait(close editor for save commit)
git config --global core.editor nvim #neovim

# color 
git config --global color.ui true

# crlf - salto de linea /r /n 
git config --global autocrlf true (only for windows)
git config --global autocrlf input (only for unix/mac)
```

