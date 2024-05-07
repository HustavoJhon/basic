1. `diskpart`

2. listado de de discos `list disk`

3. si tiene en el la columan Gpt el * es que es gpt

4. win + x > disk management

5. eliminar disco con la particion de linux

6. seleccionamos el disco para el boot `sel disk 0`

7. seleccionarmos la particion efi `sel part 1`

8. asignar una letra `assign letter z:`

9. salimos de diskpart con `exit`

10. seleccionamos el boot `bcdboot c:\windows /s z: /f all`

11. listamos todos los firmware `bcdedit.exe /enum firmware`

12. copiamos el identifier incluido con llaves

13. eliminar con el comando en cmd con administrador `bcdedit.exe /delete {id}`
