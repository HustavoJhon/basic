//! Editor de código reutilizable para lecciones y ejercicios.
//!
//! Envuelve `tui-textarea 0.7`, compatible con Ratatui 0.29, y añade tema
//! Gruvbox, numeración de líneas, estado de cambios y ayudas adaptadas al
//! ancho disponible. No conoce lenguajes concretos ni comandos de ejecución.

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use tui_textarea::TextArea;

use crate::ui::Theme;

/// Editor de texto plano con estado de modificación.
pub struct CodeEditor {
    textarea: TextArea<'static>,
    theme: Theme,
    language: String,
    baseline: String,
    dirty: bool,
}

impl CodeEditor {
    /// Crea un editor con el código inicial. Un texto vacío produce una línea
    /// vacía, nunca cero líneas.
    pub fn new(language: &str, code: &str, theme: Theme) -> Self {
        let language = language.to_string();
        let textarea = build_textarea(&language, code, theme);
        Self {
            textarea,
            theme,
            language,
            baseline: code.to_string(),
            dirty: false,
        }
    }

    /// Sustituye el contenido y reinicia el estado de modificación.
    pub fn set_code(&mut self, code: &str) {
        self.textarea = build_textarea(&self.language, code, self.theme);
        self.baseline = code.to_string();
        self.dirty = false;
    }

    /// Código actual, preservando saltos de línea finales.
    pub fn code(&self) -> String {
        self.textarea.lines().join("\n")
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Línea y columna, ambas empezando en 1 para mostrarlas al usuario.
    pub fn cursor_position(&self) -> (usize, usize) {
        let (row, column) = self.textarea.cursor();
        (row + 1, column + 1)
    }

    pub fn is_empty(&self) -> bool {
        self.textarea
            .lines()
            .iter()
            .all(|line| line.trim().is_empty())
    }

    /// Guarda o descarta el estado de modificación después de persistir.
    pub fn mark_saved(&mut self) {
        self.baseline = self.code();
        self.dirty = false;
    }

    pub fn baseline(&self) -> &str {
        &self.baseline
    }

    /// Atajos reservados para la capa de aplicación. El editor no los consume.
    pub fn is_reserved_shortcut(key: KeyEvent) -> bool {
        let save = key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL);
        let run = key.code == KeyCode::Enter && key.modifiers.contains(KeyModifiers::CONTROL);
        let note = key.code == KeyCode::Char('n') && key.modifiers.contains(KeyModifiers::CONTROL);
        save || run || note || key.code == KeyCode::Esc
    }

    /// Procesa una tecla. Devuelve `true` cuando pertenece al editor; los
    /// atajos reservados devuelven `false` para que la aplicación decida.
    /// Solo se marca como modificado el contenido, no el movimiento del cursor.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if Self::is_reserved_shortcut(key) {
            return false;
        }
        let before = self.code();
        self.textarea.input(key);
        if self.code() != before {
            self.dirty = true;
        }
        true
    }

    /// Dibuja el editor y una barra de ayuda que se acorta en pantallas
    /// pequeñas. No asume un ancho mínimo.
    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if area.height < 4 {
            frame.render_widget(&self.textarea, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);
        frame.render_widget(&self.textarea, chunks[0]);
        frame.render_widget(self.status_line(area.width), chunks[1]);
    }

    fn status_line(&self, width: u16) -> Paragraph<'static> {
        let (line, column) = self.cursor_position();
        let changed = if self.dirty { " · modificado" } else { "" };
        let position = format!("Ln {line}, Col {column}{changed}");

        let help = match width {
            0..=39 => "Ctrl+S · Ctrl+Enter · Esc".to_string(),
            40..=71 => "Ctrl+S guardar · Ctrl+Enter ejecutar · Esc volver".to_string(),
            _ => format!("Ctrl+S guardar · Ctrl+Enter ejecutar · Esc volver · {position}"),
        };
        let style = Style::default().fg(self.theme.muted).bg(self.theme.panel);
        Paragraph::new(Line::from(vec![Span::styled(help, style)]))
    }

    fn cursor_style(theme: Theme) -> Style {
        Style::default()
            .fg(theme.background)
            .bg(theme.accent)
            .add_modifier(Modifier::BOLD)
    }
}

fn build_textarea(language: &str, code: &str, theme: Theme) -> TextArea<'static> {
    let lines: Vec<String> = code.split('\n').map(str::to_string).collect();
    let mut textarea = TextArea::new(lines);
    textarea.set_style(Style::default().fg(theme.text).bg(theme.panel));
    textarea.set_cursor_line_style(Style::default().bg(theme.selection));
    textarea.set_cursor_style(CodeEditor::cursor_style(theme));
    textarea.set_line_number_style(Style::default().fg(theme.muted));
    textarea.set_placeholder_style(Style::default().fg(theme.muted));
    textarea.set_placeholder_text(format!("// Escribe código {language}…"));
    textarea.set_tab_length(4);
    textarea.set_hard_tab_indent(true);
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(Span::styled(
                format!(" Código · {language} "),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(theme.background)),
    );
    textarea
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::*;

    fn theme() -> Theme {
        Theme::gruvbox_dark()
    }

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }
    }

    #[test]
    fn preserves_starter_code_and_starts_clean() {
        let editor = CodeEditor::new("csharp", "class A {}\n", theme());
        assert_eq!(editor.code(), "class A {}\n");
        assert!(!editor.dirty());
        assert_eq!(editor.cursor_position(), (1, 1));
    }

    #[test]
    fn typing_marks_content_as_modified() {
        let mut editor = CodeEditor::new("csharp", "class A {}", theme());
        assert!(editor.handle_key(key(KeyCode::Char('x'), KeyModifiers::empty())));
        assert_eq!(editor.code(), "xclass A {}");
        assert!(editor.dirty());
        editor.mark_saved();
        assert!(!editor.dirty());
        assert_eq!(editor.baseline(), "xclass A {}");
    }

    #[test]
    fn cursor_movement_does_not_mark_content_as_modified() {
        let mut editor = CodeEditor::new("csharp", "class A {}", theme());
        assert!(editor.handle_key(key(KeyCode::Right, KeyModifiers::empty())));
        assert_eq!(editor.cursor_position(), (1, 2));
        assert!(!editor.dirty());
    }

    #[test]
    fn application_shortcuts_are_not_consumed() {
        let mut editor = CodeEditor::new("csharp", "class A {}", theme());
        for key in [
            key(KeyCode::Char('s'), KeyModifiers::CONTROL),
            key(KeyCode::Enter, KeyModifiers::CONTROL),
            key(KeyCode::Esc, KeyModifiers::empty()),
        ] {
            assert!(!editor.handle_key(key));
        }
        assert_eq!(editor.code(), "class A {}");
        assert!(!editor.dirty());
    }

    #[test]
    fn set_code_replaces_content_and_resets_dirty_state() {
        let mut editor = CodeEditor::new("csharp", "uno", theme());
        editor.handle_key(key(KeyCode::Char('x'), KeyModifiers::empty()));
        assert!(editor.dirty());
        editor.set_code("dos\n");
        assert_eq!(editor.code(), "dos\n");
        assert!(!editor.dirty());
        assert_eq!(editor.baseline(), "dos\n");
    }

    #[test]
    fn renders_title_and_help_in_a_small_area() {
        let editor = CodeEditor::new("csharp", "class A {}", theme());
        let backend = TestBackend::new(30, 7);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| editor.render(frame, frame.area()))
            .unwrap();

        let buffer = terminal.backend().buffer();
        let rendered: String = buffer.content().iter().map(|cell| cell.symbol()).collect();
        assert!(rendered.contains("Código"));
        assert!(rendered.contains("Ctrl+S"));
        assert!(rendered.contains("class"));
    }

    #[test]
    fn renders_in_a_tiny_area_without_panicking() {
        let editor = CodeEditor::new("csharp", "class A {}", theme());
        let backend = TestBackend::new(4, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| editor.render(frame, frame.area()))
            .unwrap();
    }
}
