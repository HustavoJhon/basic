clients = 'pablo, ricardo,'

def create_client(clients_name):
    global clients
    
    if clients_name not in clients:
        clients += clients_name
        _add_comma()
    else:
        print('Client already is in the client\'s list')


def list_clients():
    global clients
    print(clients)
    

def update_client(client_name, update_client_name):
    global clients
    
    if client_name in clients:
        clients = clients.replace(client_name + ',' , update_client_name)
    else:
        print('Client is not in clients list')


def _add_comma():
    global clients
    clients += ', '


def _print_welcome():
    print('WELCOME TO PLATZI VENTAS')
    print('*'*30)
    print('What would you like to do today?')
    print('[C]reate client')
    print('[U]pdate client')
    print('[D]elete client')

def _get_client_name():
    return input("Wha is the client name? ")

if __name__ == '__main__':
    _print_welcome()

    command = input('>>> ')
    command = command.upper()

    if command == 'C':
        clients_name = _get_client_name()
        create_client(clients_name)
        list_clients()
    elif command == 'D':
        pass
    elif command == 'U':
        clients_name = _get_client_name()
        update_client_name = input('What is the updated client name ')
        update_client(clients_name, update_client_name)
        list_clients()
    else:
        print("Invalid command")
