//! Renderizado de Markdown a líneas de Ratatui.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use super::Theme;

/// Convierte Markdown en líneas con estilos Gruvbox.
pub fn render(raw: &str, theme: &Theme) -> Vec<Line<'static>> {
    let mut renderer = Renderer::new(theme);
    let options = Options::all();
    for event in Parser::new_ext(raw, options) {
        renderer.event(event);
    }
    renderer.finish()
}

struct Renderer<'a> {
    theme: &'a Theme,
    lines: Vec<Line<'static>>,
    spans: Vec<Span<'static>>,
    prefix: String,
    heading_level: u8,
    in_code: bool,
    code: String,
    italic: bool,
    bold: bool,
    in_link: bool,
    in_table: bool,
    in_image: bool,
    list_depth: usize,
    pending_item: bool,
}

impl<'a> Renderer<'a> {
    fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            lines: Vec::new(),
            spans: Vec::new(),
            prefix: String::new(),
            heading_level: 0,
            in_code: false,
            code: String::new(),
            italic: false,
            bold: false,
            in_link: false,
            in_table: false,
            in_image: false,
            list_depth: 0,
            pending_item: false,
        }
    }

    fn style(&self) -> Style {
        let mut style = Style::default().fg(if self.in_link {
            self.theme.link
        } else if self.heading_level > 0 {
            self.theme.accent
        } else {
            self.theme.text
        });
        if self.bold || self.heading_level > 0 {
            style = style.add_modifier(Modifier::BOLD);
        }
        if self.italic {
            style = style.add_modifier(Modifier::ITALIC);
        }
        if self.in_link {
            style = style.add_modifier(Modifier::UNDERLINED);
        }
        style
    }

    fn event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                self.flush_line();
                self.heading_level = level as u8;
                self.prefix = format!("{} ", "#".repeat(self.heading_level as usize));
            }
            Event::End(TagEnd::Heading(_)) => {
                self.flush_line();
                self.lines.push(Line::from(""));
                self.heading_level = 0;
                self.prefix.clear();
            }
            Event::Start(Tag::Paragraph) => {
                if !self.spans.is_empty() {
                    self.flush_line();
                }
            }
            Event::End(TagEnd::Paragraph) => {
                self.flush_line();
                if !self.in_table {
                    self.lines.push(Line::from(""));
                }
            }
            Event::Start(Tag::BlockQuote(_)) => {
                self.flush_line();
                self.prefix = "│ ".to_string();
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                self.flush_line();
                self.prefix.clear();
            }
            Event::Start(Tag::CodeBlock(_)) => {
                self.flush_line();
                self.in_code = true;
                self.code.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                self.in_code = false;
                for line in self.code.lines() {
                    self.lines.push(Line::from(Span::styled(
                        format!("  {line}"),
                        Style::default().fg(self.theme.warning).bg(self.theme.panel),
                    )));
                }
                self.code.clear();
                self.lines.push(Line::from(""));
            }
            Event::Start(Tag::List(_)) => {
                self.flush_line();
                self.list_depth += 1;
            }
            Event::End(TagEnd::List(_)) => {
                self.flush_line();
                self.list_depth = self.list_depth.saturating_sub(1);
            }
            Event::Start(Tag::Item) => {
                self.flush_line();
                self.pending_item = true;
            }
            Event::End(TagEnd::Item) => {
                self.flush_line();
                self.pending_item = false;
            }
            Event::Start(Tag::Emphasis) => self.italic = true,
            Event::End(TagEnd::Emphasis) => self.italic = false,
            Event::Start(Tag::Strong) => self.bold = true,
            Event::End(TagEnd::Strong) => self.bold = false,
            Event::Start(Tag::Link { .. }) => self.in_link = true,
            Event::End(TagEnd::Link) => self.in_link = false,
            Event::Start(Tag::Image { .. }) => {
                self.in_image = true;
                self.push_span(
                    "[imagen] ".to_string(),
                    Style::default().fg(self.theme.muted),
                );
            }
            Event::End(TagEnd::Image) => self.in_image = false,
            Event::Start(Tag::Table(_)) => {
                self.flush_line();
                self.in_table = true;
            }
            Event::End(TagEnd::Table) => {
                self.flush_line();
                self.in_table = false;
            }
            Event::Start(Tag::TableHead) | Event::End(TagEnd::TableHead) => {
                self.flush_line();
            }
            Event::Start(Tag::TableRow) => self.flush_line(),
            Event::End(TagEnd::TableRow) => self.flush_line(),
            Event::Start(Tag::TableCell) => {
                if !self.spans.is_empty() {
                    self.push_span(" │ ".to_string(), Style::default().fg(self.theme.border));
                }
            }
            Event::End(TagEnd::TableCell) => {}
            Event::Text(text) => {
                if self.in_code {
                    self.code.push_str(&text);
                } else if self.in_image {
                    self.push_span(format!("{text} "), Style::default().fg(self.theme.muted));
                } else {
                    if self.pending_item {
                        let indent = "  ".repeat(self.list_depth.saturating_sub(1));
                        self.prefix = format!("{indent}• ");
                        self.pending_item = false;
                    }
                    self.push_span(text.to_string(), self.style());
                }
            }
            Event::Code(code) => {
                if !self.in_code {
                    self.push_span(
                        format!("`{code}`"),
                        Style::default().fg(self.theme.accent).bg(self.theme.panel),
                    );
                }
            }
            Event::SoftBreak => self.push_span(" ".to_string(), self.style()),
            Event::HardBreak => self.flush_line(),
            Event::Rule => {
                self.flush_line();
                self.lines.push(Line::from(Span::styled(
                    "─".repeat(48),
                    Style::default().fg(self.theme.border),
                )));
            }
            Event::TaskListMarker(done) => {
                self.push_span(
                    if done { "[x] " } else { "[ ] " }.to_string(),
                    Style::default().fg(self.theme.success),
                );
            }
            Event::Html(_) | Event::InlineHtml(_) | Event::FootnoteReference(_) => {}
            _ => {}
        }
    }

    fn push_span(&mut self, text: String, style: Style) {
        if text.is_empty() {
            return;
        }
        if self.spans.is_empty() && !self.prefix.is_empty() {
            let prefix = std::mem::take(&mut self.prefix);
            self.spans
                .push(Span::styled(prefix, Style::default().fg(self.theme.muted)));
        }
        self.spans.push(Span::styled(text, style));
    }

    fn flush_line(&mut self) {
        if self.spans.is_empty() {
            return;
        }
        let spans = std::mem::take(&mut self.spans);
        self.lines.push(Line::from(spans));
        if !self.in_table {
            self.prefix.clear();
        }
    }

    fn finish(mut self) -> Vec<Line<'static>> {
        if self.in_code && !self.code.is_empty() {
            for line in self.code.lines() {
                self.lines.push(Line::from(Span::styled(
                    format!("  {line}"),
                    Style::default().fg(self.theme.warning).bg(self.theme.panel),
                )));
            }
        }
        self.flush_line();
        if self.lines.is_empty() {
            self.lines.push(Line::from(Span::styled(
                "(nota vacía)",
                Style::default().fg(self.theme.muted),
            )));
        }
        self.lines
    }
}
