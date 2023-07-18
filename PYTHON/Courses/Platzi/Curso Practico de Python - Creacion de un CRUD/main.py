import sys 

from tabulate import tabulate

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

clients = [
    {
        'name': 'Pablo',
        'company': 'Google',
        'email': 'pablo@google.com',
        'position': 'Software ingineer'
    },
    {
        'name': 'Ricardo',
        'company': 'Meta',
        'email': 'ricardo@meta.com',
        'position': 'Graphic Designer'
    },
    {
        'name': 'Jose',
        'company': 'Twitter',
        'email':'jose@twitter.com',
        'position': 'Data engineer'
    },
    {
        'name': 'Roberto',
        'company': 'Apple',
        'email': 'roberto@apple.com',
        'position': 'Web Developer'
    }
]
    

#* CREATE CLIENTS
        
def create_client(client):
    global clients
    
    if client not in clients:
        clients.append(client)
    else:
        console.print('Client already is in the client\'s list')

#* LIST CLIENTS

def list_clients():
    # for idx, client in enumerate(clients):
        # print('{uid} | {name} | {company} | {email} | {position}'.format(
        #     uid=idx,
        #     name=client['name'],
        #     company=client['company'],
        #     email=client['email'],
        #     position=client['position']
        # ))
    print(tabulate(
        clients, 
        headers='keys', 
        tablefmt='fancy_grid', 
        showindex=True
        )
    )
    
#* UPDATE CLIENTS

def update_client(client_name, updated_name):
    global clients
    
    if client_name in clients:
        index = clients.index(client_name)
        clients[index] = updated_name
    else:
        print('Client not in clients list')

#* DELETE CLIENTS

def delete_client(client_name):
    global clients
    
    if client_name in clients:
        clients.remove(client_name)
    else:
        print("Client is not in clients list")
        
#* SEARCH CLIENTS

def search_client(client_name):
    for client in clients:
        if client != client_name:
            continue
        else:
            return True
    
    
def _get_client_field(field_name):
    field = None 
    
    while not field:
        field = input('What is the client {}'.format(field_name))

    return field

def _get_client_name():
    client_name = None

    while not client_name:
        client_name = input('What is the client name?')

        if client_name == 'exit':
            client_name = None 
            break
    if not client_name:
        sys.exit()
        
    return client_name

    # return input("What is the client name? ")

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


if __name__ == '__main__':
    _print_welcome()

    command = input('>>> ')
    command = command.upper()

    if command == 'C':
        client = {
            'name' : _get_client_field('name'),
            'company': _get_client_field('company'),
            'email': _get_client_field('email'),
            'position': _get_client_field('position'),
        }
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
        update_client(client_name, update_client_name)
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