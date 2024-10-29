from rich.console import Console
from rich.markdown import Markdown

MARKDOWN = """
# This is basic 
"""


console = Console()
md = Markdown(MARKDOWN)
console.print(md)
