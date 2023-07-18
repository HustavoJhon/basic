from rich.console import Console
from rich.table import Table 

table = Table(title="Clients")

rows = [
    ["John", "Doe", "45"],
    ["Jane", "Doe", "32"],
    ["Mary", "Smith", "25"],
]

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

columns = ["Name", "Company", "Email", "Position"]

for column in columns:
    table.add_column(column)

for row in clients:
    table.add_row(*row, style='bright_green')

console = Console()
console.print(table)