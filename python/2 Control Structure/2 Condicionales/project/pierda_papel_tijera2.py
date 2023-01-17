user_option = input('Piedra, Papel, Tijera ') 
computer_option = 'papel' 

# Empate
if user_option == computer_option:
    print('empate')

elif user_option == 'piedra':
    if computer_option == 'tijera':
        print('piedra gana a tijera')
        print('user gano!')
    else:
        print('Papel gana a Piedra')
        print('Computer Gano')
elif user_option == 'papel':
    if computer_option == 'piedra':
        print('papel gana a tijera')
        print('user Gano!')
    else:
        print('tijera gana a papel')
        print('Computer Gano!')
elif user_option == 'tijera':
    if computer_option == 'papel':
        print('Tijera Gana a Pepel')
        print('user Gano!')
    else:
        print('Piedra gana a tijera')
        print('computer gano!')
