from rich.console import Console 
from rich.theme import Theme 
from rich.table import Table



custom_theme = Theme({
    "good": "green",
    "bad": "bold red"
})

# console = Console(theme=custom_theme)   

# console = Console()

clients = 'pablo, ricardo, jose, roberto'

        
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


def delete_client(client_name):
    global clients
    
    if client_name in clients:
        clients = clients.replace(client_name + ',', '')
    else:
        print("Client is not in clients list")
        

def search_client(client_name):
    # global clients
    
    clients_list = clients.split(',')
    
    for client in clients_list:
        if client != client_name:
            continue
        else:
            return True
        

def _add_comma():
    global clients
    clients += ', '
    
console = Console(theme=custom_theme)  

def _print_welcome():
    welcome = 'WELCOME TO PLATZI VENTAS '
    # style="bold underline red on black"

    table = Table(title = welcome)

    table.add_column("🔍", justify="right", style="cyan")
    table.add_column("Option", justify="left", style="magenta")

    table.add_row("[bold yellow][C][/]", "create clients")
    table.add_row("[bold green][R][/]", "read clients")
    table.add_row("[bold blue][U][/]", "update clients")
    table.add_row("[bold red][D][/]", "delete clients")
    table.add_row("")
    table.add_row("[bold purple][L][/]", "list clients")
    table.add_row("[bold purple][S][/]", "search clients")

    console.print(table)




def _get_client_name():
    return input("What is the client name? ")


if __name__ == '__main__':
    _print_welcome()

    command = input('>>> ')
    command = command.upper()

    if command == 'C':
        clients_name = _get_client_name()
        create_client(clients_name)
        list_clients()
        
    elif command == 'D':
        clients_name = _get_client_name()
        delete_client(clients_name)
        list_clients()
        
    elif command == 'U':
        clients_name = _get_client_name()
        update_client_name = input('What is the updated client name ')
        update_client(clients_name, update_client_name)
        list_clients()
        
    elif command == 'L':
        list_clients()
        
    elif command == 'S':
        client_name = _get_client_name()
        found = search_client(client_name)
        
        if found:
            print('The client is in the client\'s list')
        else:
            print('The client: {} is not in our clients\'s list'.format(client_name))

    else:
        print("Invalid command")
