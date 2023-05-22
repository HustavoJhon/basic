from rich.console import Console
from rich.tree import Tree

console = Console(record=True, width=100)

tree = Tree("🙂 Vincent D. Warmerdam", guide_style="bold bright_black")

python_tree = tree.add("📦 Open Source Packages", guide_style="bright_black")
python_tree.add("[bold]scikit-lego[/] - [bright_black]lego bricks for sklearn")
python_tree.add("[bold]human-learn[/] - [bright_black]rule-based components for sklearn")

online_tree = tree.add("⭐ Online Projects", guide_style="bright_black")
online_tree.add("[bold]koaning.io[/]   - [bright_black]personal blog")
online_tree.add("[bold]calmcode.io[/]  - [bright_black]dev education service")

talk_tree = tree.add("🎙️ Popular Talks", guide_style="bright_black")
talk_tree.add("[bold]Optimal Benchmarks and Other Failures[/]")
talk_tree.add("[bold]Playing by the Rules-Based-Systems[/]")

employer_tree = tree.add("👨‍💻 Employer", guide_style="bright_black")
employer_tree.add("[bold]Rasa[/] - [bright_black]conversational software")

console.print(tree)
console.print("")
console.print("[green]Follow me on twitter [bold]@hustavojhon[/]")

CONSOLE_HTML_FORMAT = """\
<pre style="font-family:Menlo,'DejaVu Sans Mono',consolas,'Courier New',monospace">{code}</pre>
"""

# console.save_html("README.md", inline_styles=True, code_format=CONSOLE_HTML_FORMAT)