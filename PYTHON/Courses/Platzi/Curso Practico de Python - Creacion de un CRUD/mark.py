from rich.columns import Columns
from rich.console import Console
from rich.panel import Panel
from rich.markdown import Markdown

md1 = """
# Hello World

## This is Markdown

And it renders *very* **nicely**!
"""

md2 = """
## This is Markdown, Again

With code!

```python
print("hello world")
```
"""

console = Console(record=True)
panel_1 = Panel.fit(Markdown(md1), title="panel one", width=30)
panel_2 = Panel.fit(Markdown(md2), title="panel two", width=30)
console.print(Columns([panel_1, panel_2]))
