from rich.console import Console 
from rich.theme import Theme 
from rich.table import Table
from rich.columns import Columns
from rich.panel import Panel
from rich.markdown import Markdown



custom_theme = Theme({
    "good": "green",
    "bad": "bold red"
})

# console = Console(theme=custom_theme)   

# console = Console()

clients = 'pablo,ricardo,jose,roberto,'

#* CREATE CLIENTS
        
def create_client(clients_name):
    global clients
    
    if client_name not in clients:
        clients += client_name
        _add_comma()
    else:
        console.print('Client already is in the client\'s list')

#* LIST CLIENTS

def list_clients():
    global clients
    print(clients)
    
#* UPDATE CLIENTS

def update_client(client_name, updated_name):
    global clients
    
    if client_name in clients:
        clients = clients.replace(client_name + ',' , updated_name + ',')
    else:
        print('Client not in clients list')

#* DELETE CLIENTS

def delete_client(client_name):
    global clients
    
    if client_name in clients:
        clients = clients.replace(client_name + ',', '')
    else:
        print("Client is not in clients list")
        
#* SEARCH CLIENTS

def search_client(client_name):
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
        client_name = _get_client_name()
        create_client(client_name)
        list_clients()
        
    elif command == 'D':
        client_name = _get_client_name()
        delete_client(client_name)
        list_clients()
        
    elif command == 'U':
        client_name = _get_client_name()
        update_client_name = input('What is the updated client name ')
        update_client(clients_name, update_client_name)
        list_clients()
        
    elif command == 'L':
        list_clients()
        
    elif command == 'S':
        client_name = _get_client_name()
        found = search_client(client_name)
        list_clients()
        
        if found:
            console.print('[good]The client is in the client\'s list[/good]')
        else:
            console.print('[bad]The client: {} is not in our clients\'s list[/bad]'.format(client_name))

    else:
        console.print("[bad]Invalid command[/bad]")