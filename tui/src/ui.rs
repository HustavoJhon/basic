//! Interfaz de terminal con Ratatui y la paleta Gruvbox Dark.

mod markdown;

use std::collections::HashMap;
use std::io;
use std::process::Command;
use std::time::Duration;

use anyhow::Context;
use chrono::{Datelike, Duration as ChronoDuration, NaiveDate};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::cursor::{Hide, Show};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use crate::app::{App, ExerciseWorkspace, Screen, format_clock};
use crate::code_lab::domain::ExerciseStatus;
use crate::code_lab::ui::editor::CodeEditor;
use crate::config::Config;
use crate::domain::{
    Document, LearnMode, LearningStatus, ReviewRating, activity_level, format_duration,
};
use crate::infra::index;
use crate::infra::sqlite::Database;

/// Paleta Gruvbox Dark aprobada en F0.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub panel: Color,
    pub selection: Color,
    pub border: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub link: Color,
}

impl Theme {
    pub fn gruvbox_dark() -> Self {
        Self {
            background: Color::Rgb(0x28, 0x28, 0x28),
            panel: Color::Rgb(0x3c, 0x38, 0x36),
            selection: Color::Rgb(0x50, 0x49, 0x45),
            border: Color::Rgb(0x66, 0x5c, 0x54),
            text: Color::Rgb(0xeb, 0xdb, 0xb2),
            muted: Color::Rgb(0x92, 0x83, 0x74),
            accent: Color::Rgb(0xfe, 0x80, 0x19),
            success: Color::Rgb(0xb8, 0xbb, 0x26),
            warning: Color::Rgb(0xfa, 0xbd, 0x2f),
            error: Color::Rgb(0xfb, 0x49, 0x34),
            link: Color::Rgb(0x83, 0xa5, 0x98),
        }
    }

    fn base(&self) -> Style {
        Style::default().fg(self.text).bg(self.background)
    }

    fn panel(&self) -> Style {
        Style::default().fg(self.text).bg(self.panel)
    }

    fn block(&self, title: &str) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border))
            .title(Span::styled(
                format!(" {title} "),
                Style::default()
                    .fg(self.accent)
                    .add_modifier(Modifier::BOLD),
            ))
            .style(self.base())
    }
}

/// Estado visual que no pertenece a la lógica de la aplicación.
#[derive(Default)]
struct UiState {
    editor: Option<CodeEditor>,
    editor_exercise_id: Option<i64>,
}

impl UiState {
    fn sync_editor(&mut self, app: &App, theme: Theme) {
        match &app.code_lab.workspace {
            Some(workspace) if self.editor_exercise_id != Some(workspace.exercise.id) => {
                self.editor = Some(CodeEditor::new(
                    &workspace.exercise.language,
                    &workspace.code,
                    theme,
                ));
                self.editor_exercise_id = Some(workspace.exercise.id);
            }
            None => {
                self.editor = None;
                self.editor_exercise_id = None;
            }
            _ => {}
        }
    }
}

/// Arranca la TUI hasta que el usuario sale.
pub fn run(config: Config, database: Database) -> anyhow::Result<()> {
    let mut app = App::new(config, database).context("no se pudo inicializar la aplicación")?;
    setup_terminal()?;
    let result = (|| -> anyhow::Result<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        let mut ui = UiState::default();
        run_loop(&mut terminal, &mut app, &mut ui)
    })();
    let restore = restore_terminal();
    restore?;
    result
}

fn setup_terminal() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, Hide)?;
    Ok(())
}

fn restore_terminal() -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, Show)?;
    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    ui: &mut UiState,
) -> anyhow::Result<()> {
    loop {
        if let Err(error) = app.tick() {
            app.status = format!("Pomodoro: {error}");
        }

        if app.editor_request.is_some() {
            open_editor(terminal, app)?;
        }

        ui.sync_editor(app, Theme::gruvbox_dark());
        terminal.draw(|frame| draw(frame, app, ui))?;

        // El runner es síncrono: dibujamos primero el estado «ejecutando» y
        // solo después bloqueamos la terminal con el SDK del lenguaje.
        if app.code_lab.pending_run
            && let Err(error) = app.run_pending()
        {
            app.status = format!("Ejecución: {error}");
        }

        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    handle_key(app, ui, key)?;
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn open_editor(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> anyhow::Result<()> {
    let Some(path) = app.editor_request.take() else {
        return Ok(());
    };
    restore_terminal()?;
    let editor = app.config.editor.trim();
    let mut parts = editor.split_whitespace();
    let program = parts.next().unwrap_or("vi");
    let status = Command::new(program).args(parts).arg(&path).status();
    setup_terminal()?;
    terminal.clear()?;
    match status {
        Ok(status) if status.success() => app.reload_reader()?,
        Ok(status) => app.status = format!("El editor terminó con {status}."),
        Err(error) => app.status = format!("No se pudo abrir {program}: {error}"),
    }
    Ok(())
}

fn handle_key(app: &mut App, ui: &mut UiState, key: KeyEvent) -> anyhow::Result<()> {
    if app.help_visible {
        if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
            app.help_visible = false;
        }
        return Ok(());
    }

    if app.prompt.is_some() {
        return handle_prompt(app, key);
    }

    if app.search_mode {
        return handle_search(app, key);
    }

    if app.screen == Screen::Exercise {
        if app.code_lab.workspace.is_none() {
            app.back()?;
            return Ok(());
        }
        if let Some(editor) = ui.editor.as_mut()
            && editor.handle_key(key)
        {
            return Ok(());
        }
        match key.code {
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let code = ui.editor.as_ref().map(CodeEditor::code).unwrap_or_default();
                app.save_exercise_code(&code)?;
                if let Some(editor) = ui.editor.as_mut() {
                    editor.mark_saved();
                }
            }
            KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let code = ui.editor.as_ref().map(CodeEditor::code).unwrap_or_default();
                app.request_exercise_run(&code);
                if let Some(editor) = ui.editor.as_mut() {
                    editor.mark_saved();
                }
            }
            KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.toggle_solution()?;
                let solution = app
                    .code_lab
                    .workspace
                    .as_ref()
                    .and_then(|workspace| workspace.solution.clone());
                if let (Some(editor), Some(solution)) = (ui.editor.as_mut(), solution) {
                    editor.set_code(&solution);
                }
            }
            KeyCode::Esc => app.back()?,
            _ => {}
        }
        return Ok(());
    }

    // Los dígitos son contextuales en Learn y Review.
    if app.screen == Screen::Learn
        && let KeyCode::Char(character @ '1'..='5') = key.code
    {
        let index = character as usize - '1' as usize;
        app.set_learn_mode(LearnMode::ALL[index])?;
        return Ok(());
    }
    if app.screen == Screen::Review
        && let KeyCode::Char(character) = key.code
        && let Some(rating) = ReviewRating::from_key(character)
    {
        app.rate_review(rating)?;
        return Ok(());
    }

    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Esc => app.back()?,
        KeyCode::Char('?') => app.help_visible = true,
        KeyCode::Char('1') => app.set_screen(Screen::Dashboard)?,
        KeyCode::Char('2') => app.set_screen(Screen::Explore)?,
        KeyCode::Char('3') => app.set_screen(Screen::Learn)?,
        KeyCode::Char('4') => app.set_screen(Screen::Review)?,
        KeyCode::Char('5') => app.set_screen(Screen::Pomodoro)?,
        KeyCode::Char('6') => app.set_screen(Screen::Stats)?,
        KeyCode::Char('7') => app.open_code_lab_screen()?,
        KeyCode::Tab => {
            let next = match app.screen {
                Screen::Dashboard => Screen::Explore,
                Screen::Explore => Screen::Learn,
                Screen::Learn => Screen::Review,
                Screen::Review => Screen::Pomodoro,
                Screen::Pomodoro => Screen::Stats,
                Screen::Stats => Screen::CodeLab,
                Screen::CodeLab => Screen::Dashboard,
                Screen::Reader | Screen::Exercise => Screen::Dashboard,
            };
            if next == Screen::CodeLab {
                app.open_code_lab_screen()?;
            } else {
                app.set_screen(next)?;
            }
        }
        KeyCode::Char('j') | KeyCode::Down => {
            if app.screen == Screen::Reader {
                app.reader_scroll(1);
            } else if app.screen == Screen::CodeLab {
                app.move_exercise_selection(1);
            } else {
                app.move_selection(1);
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.screen == Screen::Reader {
                app.reader_scroll(-1);
            } else if app.screen == Screen::CodeLab {
                app.move_exercise_selection(-1);
            } else {
                app.move_selection(-1);
            }
        }
        KeyCode::PageDown => app.reader_scroll(12),
        KeyCode::PageUp => app.reader_scroll(-12),
        KeyCode::Char('g') => {
            if app.screen == Screen::Reader {
                app.reader_scroll(-10_000);
            } else if app.screen == Screen::CodeLab {
                app.code_lab.selected = 0;
            } else {
                app.selected = 0;
            }
        }
        KeyCode::Char('G') => {
            if app.screen == Screen::Reader {
                app.reader_scroll(10_000);
            } else if app.screen == Screen::CodeLab {
                if !app.code_lab.exercises.is_empty() {
                    app.code_lab.selected = app.code_lab.exercises.len() - 1;
                }
            } else if !app.documents.is_empty() {
                app.selected = app.documents.len() - 1;
            }
        }
        KeyCode::Char('/') => {
            app.search_mode = true;
            app.screen = Screen::Explore;
            app.status = "Escribe para buscar; Esc limpia.".to_string();
        }
        KeyCode::Char('n') => app.start_new_note_prompt(),
        KeyCode::Char('i') => {
            app.status = "Indexando el vault…".to_string();
            match index::index_vault(&mut app.database, &app.config) {
                Ok(report) => {
                    app.status = report.summary();
                    app.refresh()?;
                }
                Err(error) => app.status = format!("Error al indexar: {error}"),
            }
        }
        KeyCode::Enter => {
            if app.screen == Screen::CodeLab {
                app.open_selected_exercise()?;
            } else if matches!(
                app.screen,
                Screen::Explore | Screen::Learn | Screen::Dashboard
            ) {
                app.open_selected()?;
            }
        }
        KeyCode::Char('s') if app.screen == Screen::Reader => app.cycle_reader_status()?,
        KeyCode::Char('f') if app.screen == Screen::Reader => app.toggle_favorite()?,
        KeyCode::Char('e') if app.screen == Screen::Reader => app.request_editor(),
        KeyCode::Char('p') => {
            app.set_screen(Screen::Pomodoro)?;
            if !app.pomodoro.running {
                app.toggle_pomodoro()?;
            }
        }
        KeyCode::Char(' ') if app.screen == Screen::Pomodoro => app.toggle_pomodoro()?,
        KeyCode::Char('x') if app.screen == Screen::Pomodoro => app.reset_pomodoro()?,
        KeyCode::Char('>') if app.screen == Screen::Pomodoro => app.skip_pomodoro()?,
        _ => {}
    }
    Ok(())
}

fn handle_search(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => app.search_clear(),
        KeyCode::Enter => {
            app.search_mode = false;
            app.open_selected()?;
        }
        KeyCode::Backspace => app.search_pop(),
        KeyCode::Up => app.move_selection(-1),
        KeyCode::Down => app.move_selection(1),
        KeyCode::Char(character) => app.search_push(character),
        _ => {}
    }
    Ok(())
}

fn handle_prompt(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => app.prompt_cancel(),
        KeyCode::Enter => match app.prompt_submit() {
            Ok(()) => {}
            Err(error) => app.status = format!("No se pudo crear la nota: {error}"),
        },
        KeyCode::Backspace => app.prompt_pop(),
        KeyCode::Char(character) => app.prompt_push(character),
        _ => {}
    }
    Ok(())
}

fn draw(frame: &mut Frame, app: &App, ui: &UiState) {
    let theme = Theme::gruvbox_dark();
    let area = frame.area();
    frame.render_widget(Block::default().style(theme.base()), area);

    if area.width < 70 || area.height < 20 {
        frame.render_widget(
            Paragraph::new("Terminal demasiado pequeña. Necesitas al menos 70×20.")
                .style(Style::default().fg(theme.warning))
                .alignment(Alignment::Center)
                .block(theme.block("BASIC")),
            area,
        );
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(area);

    draw_header(frame, layout[0], app, &theme);
    match app.screen {
        Screen::Dashboard => draw_dashboard(frame, layout[1], app, &theme),
        Screen::Explore => draw_explore(frame, layout[1], app, &theme),
        Screen::Reader => draw_reader(frame, layout[1], app, &theme),
        Screen::Learn => draw_learn(frame, layout[1], app, &theme),
        Screen::Review => draw_review(frame, layout[1], app, &theme),
        Screen::Pomodoro => draw_pomodoro(frame, layout[1], app, &theme),
        Screen::Stats => draw_stats(frame, layout[1], app, &theme),
        Screen::CodeLab => draw_code_lab(frame, layout[1], app, &theme),
        Screen::Exercise => draw_exercise(frame, layout[1], app, ui, &theme),
    }
    draw_footer(frame, layout[2], app, &theme);

    if let Some(prompt) = &app.prompt {
        draw_prompt(frame, area, prompt, &theme);
    }
    if app.help_visible {
        draw_help(frame, area, &theme);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let screens = [
        ("1", "inicio", Screen::Dashboard),
        ("2", "explorar", Screen::Explore),
        ("3", "aprender", Screen::Learn),
        ("4", "repaso", Screen::Review),
        ("5", "pomodoro", Screen::Pomodoro),
        ("6", "stats", Screen::Stats),
        ("7", "code lab", Screen::CodeLab),
    ];
    let mut tabs = vec![Span::styled(
        " BASIC ",
        Style::default()
            .fg(theme.background)
            .bg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )];
    for (key, label, screen) in screens {
        let style = if app.screen == screen {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.muted)
        };
        tabs.push(Span::raw(" "));
        tabs.push(Span::styled(
            format!("[{key}]"),
            Style::default().fg(theme.border),
        ));
        tabs.push(Span::styled(format!(" {label}"), style));
    }
    tabs.push(Span::raw("   "));
    tabs.push(Span::styled(&app.status, Style::default().fg(theme.muted)));

    let lines = vec![
        Line::from(tabs),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                format!(
                    "{} · {} documentos · {} pendientes · {} para repasar",
                    app.screen.label(),
                    app.stats.total,
                    app.stats.pending,
                    app.stats.due
                ),
                Style::default().fg(theme.muted),
            ),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines).style(theme.base()).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(theme.border))
                .style(theme.base()),
        ),
        area,
    );
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let hints = match app.screen {
        Screen::Dashboard | Screen::Explore => {
            "j/k mover · Enter abrir · / buscar · n nueva · i indexar · 1-7 pantallas · q salir"
        }
        Screen::Reader => {
            "j/k desplazar · s estado · f favorito · e editar · p pomodoro · Esc volver · q salir"
        }
        Screen::Learn => "1-5 modo · j/k mover · Enter abrir · / buscar · i indexar · Esc volver",
        Screen::Review => "1-4 calificar · j/k mover · Enter abrir · Esc volver",
        Screen::Pomodoro => "Espacio iniciar/pausar · x reiniciar · > saltar · 1-7 pantallas",
        Screen::Stats => "j/k mover · i indexar · 1-7 pantallas · q salir",
        Screen::CodeLab => "j/k elegir · Enter abrir · Esc volver · 1-7 pantallas · q salir",
        Screen::Exercise => "Ctrl+S guardar · Ctrl+Enter ejecutar · Ctrl+V solución · Esc volver",
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("?", Style::default().fg(theme.accent)),
            Span::styled(" ayuda  ", Style::default().fg(theme.muted)),
            Span::styled(hints, Style::default().fg(theme.muted)),
        ]))
        .style(theme.base())
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(theme.border)),
        ),
        area,
    );
}

fn draw_dashboard(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(5)])
        .split(area);

    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(rows[0]);

    stat_card(
        frame,
        cards[0],
        "documentos",
        &app.stats.total.to_string(),
        theme.accent,
        theme,
    );
    stat_card(
        frame,
        cards[1],
        "para repasar",
        &app.stats.due.to_string(),
        theme.warning,
        theme,
    );
    stat_card(
        frame,
        cards[2],
        "estudio hoy",
        &format_duration(app.stats.study_seconds_today),
        theme.success,
        theme,
    );
    stat_card(
        frame,
        cards[3],
        "racha",
        &format!("{} días", app.stats.streak_days),
        theme.link,
        theme,
    );

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(rows[1]);

    document_list(frame, columns[0], app, "continuar", theme);
    let mut lines = Vec::new();
    lines.push(Line::from(vec![Span::styled(
        "categorías",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )]));
    for (category, count) in &app.stats.categories {
        lines.push(Line::from(vec![
            Span::styled(format!("  {category:<20}"), Style::default().fg(theme.text)),
            Span::styled(format!("{count:>4}"), Style::default().fg(theme.muted)),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "acciones rápidas",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(Span::styled(
        "  [3] aprender   [4] repasar   [5] pomodoro",
        Style::default().fg(theme.muted),
    )));
    lines.push(Line::from(Span::styled(
        "  [n] nueva nota   [i] reindexar",
        Style::default().fg(theme.muted),
    )));
    frame.render_widget(
        Paragraph::new(lines)
            .style(theme.base())
            .block(theme.block("resumen")),
        columns[1],
    );
}

fn stat_card(frame: &mut Frame, area: Rect, label: &str, value: &str, color: Color, theme: &Theme) {
    let lines = vec![
        Line::from(Span::styled(
            label.to_uppercase(),
            Style::default().fg(theme.muted),
        )),
        Line::from(Span::styled(
            value.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(theme.panel())
            .block(theme.block(label)),
        area,
    );
}

fn draw_explore(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    let cursor = if app.search_mode { "_" } else { "" };
    let query = if app.query.is_empty() {
        "Buscar por título, contenido, ruta o categoría…".to_string()
    } else {
        app.query.clone()
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  / ", Style::default().fg(theme.accent)),
            Span::styled(query, Style::default().fg(theme.text)),
            Span::styled(cursor, Style::default().fg(theme.accent)),
        ]))
        .style(theme.base())
        .block(theme.block("buscar")),
        rows[0],
    );

    if area.width >= 100 {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
            .split(rows[1]);
        document_list(frame, columns[0], app, "resultados", theme);
        document_preview(frame, columns[1], app, theme);
    } else {
        document_list(frame, rows[1], app, "resultados", theme);
    }
}

fn draw_learn(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    let mut tabs = Vec::new();
    for (index, mode) in LearnMode::ALL.iter().enumerate() {
        let style = if *mode == app.learn_mode {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.muted)
        };
        tabs.push(Span::styled(
            format!(" [{}] {} ", index + 1, mode.label()),
            style,
        ));
    }
    frame.render_widget(
        Paragraph::new(Line::from(tabs))
            .style(theme.base())
            .block(theme.block("modo")),
        rows[0],
    );

    if area.width >= 100 {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
            .split(rows[1]);
        document_list(frame, columns[0], app, app.learn_mode.label(), theme);
        document_preview(frame, columns[1], app, theme);
    } else {
        document_list(frame, rows[1], app, app.learn_mode.label(), theme);
    }
}

fn draw_review(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    if app.review_queue.is_empty() {
        frame.render_widget(
            Paragraph::new(
                "No hay repasos pendientes. Vuelve mañana o marca una nota para repasar.",
            )
            .alignment(Alignment::Center)
            .style(theme.base())
            .block(theme.block("repaso")),
            area,
        );
        return;
    }

    let document = &app.review_queue[app.review_index];
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(5)])
        .split(area);

    let body = vec![
        Line::from(Span::styled(
            document.title.clone(),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("{} · {}", document.path, document.status.label()),
            Style::default().fg(theme.muted),
        )),
        Line::from(""),
        Line::from(document.excerpt.clone()),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                "{} de {} repasos · vence {}",
                app.review_index + 1,
                app.review_queue.len(),
                document
                    .review_due_at
                    .map(crate::infra::catalog::format_date)
                    .unwrap_or_else(|| "-".to_string())
            ),
            Style::default().fg(theme.muted),
        )),
    ];
    frame.render_widget(
        Paragraph::new(body)
            .wrap(Wrap { trim: false })
            .style(theme.base())
            .block(theme.block("repaso")),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  1 ", Style::default().fg(theme.error)),
            Span::styled("otra vez   ", Style::default().fg(theme.text)),
            Span::styled("2 ", Style::default().fg(theme.warning)),
            Span::styled("difícil   ", Style::default().fg(theme.text)),
            Span::styled("3 ", Style::default().fg(theme.success)),
            Span::styled("bien   ", Style::default().fg(theme.text)),
            Span::styled("4 ", Style::default().fg(theme.link)),
            Span::styled("fácil", Style::default().fg(theme.text)),
        ]))
        .alignment(Alignment::Center)
        .style(theme.base())
        .block(theme.block("calificar")),
        rows[1],
    );
}

fn draw_pomodoro(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(area);

    let ratio = if app.pomodoro.planned_seconds > 0 {
        (app.pomodoro.planned_seconds - app.pomodoro.remaining_seconds) as f64
            / app.pomodoro.planned_seconds as f64
    } else {
        0.0
    };
    let state = if app.pomodoro.running {
        "en marcha"
    } else if app.pomodoro.session_id.is_some() {
        "en pausa"
    } else {
        "detenido"
    };
    let lines = vec![
        Line::from(Span::styled(
            format_clock(app.pomodoro.remaining_seconds),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("{} · {state}", app.pomodoro.phase.label())),
        Line::from(format!(
            "ciclo {} · {}",
            app.pomodoro.cycle + 1,
            app.pomodoro
                .document_id
                .and_then(|_| app
                    .reader
                    .as_ref()
                    .map(|reader| reader.document.title.clone()))
                .unwrap_or_else(|| "sin documento".to_string())
        )),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(theme.base())
            .block(theme.block("pomodoro")),
        rows[0],
    );

    let gauge = Gauge::default()
        .block(theme.block("progreso"))
        .gauge_style(Style::default().fg(theme.accent).bg(theme.panel))
        .ratio(ratio.clamp(0.0, 1.0))
        .label(format!("{}%", (ratio * 100.0).round() as u32));
    frame.render_widget(gauge, rows[1]);
}

fn draw_stats(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(area);

    draw_heatmap(frame, columns[0], app, theme);

    let lines = vec![
        Line::from(vec![
            Span::styled("documentos   ", Style::default().fg(theme.muted)),
            Span::styled(
                app.stats.total.to_string(),
                Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("pendientes   ", Style::default().fg(theme.muted)),
            Span::styled(
                app.stats.pending.to_string(),
                Style::default().fg(theme.warning),
            ),
        ]),
        Line::from(vec![
            Span::styled("en curso     ", Style::default().fg(theme.muted)),
            Span::styled(
                app.stats.in_progress.to_string(),
                Style::default().fg(theme.link),
            ),
        ]),
        Line::from(vec![
            Span::styled("para repasar ", Style::default().fg(theme.muted)),
            Span::styled(app.stats.due.to_string(), Style::default().fg(theme.error)),
        ]),
        Line::from(vec![
            Span::styled("vacíos       ", Style::default().fg(theme.muted)),
            Span::styled(
                app.stats.stubs.to_string(),
                Style::default().fg(theme.muted),
            ),
        ]),
        Line::from(vec![
            Span::styled("estudio hoy  ", Style::default().fg(theme.muted)),
            Span::styled(
                format_duration(app.stats.study_seconds_today),
                Style::default().fg(theme.success),
            ),
        ]),
        Line::from(vec![
            Span::styled("pomodoros    ", Style::default().fg(theme.muted)),
            Span::styled(
                app.stats.pomodoros_today.to_string(),
                Style::default().fg(theme.success),
            ),
        ]),
        Line::from(vec![
            Span::styled("racha        ", Style::default().fg(theme.muted)),
            Span::styled(
                format!("{} días", app.stats.streak_days),
                Style::default().fg(theme.accent),
            ),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .style(theme.base())
            .block(theme.block("resumen")),
        columns[1],
    );
}

fn draw_heatmap(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let by_day: HashMap<&str, i64> = app
        .activity
        .iter()
        .map(|day| (day.day.as_str(), day.study_seconds))
        .collect();
    let today = app
        .activity
        .last()
        .and_then(|day| NaiveDate::parse_from_str(&day.day, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Local::now().date_naive());
    // Alinear las columnas al lunes para que las etiquetas de fila sean fieles.
    let mut start = today - ChronoDuration::days(364);
    start -= ChronoDuration::days(start.weekday().num_days_from_monday() as i64);
    let weeks = ((today - start).num_days() / 7 + 1) as usize;

    let labels = ["L", "M", "X", "J", "V", "S", "D"];
    let mut lines = Vec::new();
    for (row, label) in labels.iter().enumerate() {
        let mut spans = vec![Span::styled(
            format!("  {label} "),
            Style::default().fg(theme.muted),
        )];
        for week in 0..weeks {
            let date = start + ChronoDuration::days((week * 7 + row) as i64);
            if date > today {
                spans.push(Span::raw(" "));
                continue;
            }
            let key = date.format("%Y-%m-%d").to_string();
            let seconds = by_day.get(key.as_str()).copied().unwrap_or(0);
            spans.push(Span::styled(
                "■",
                Style::default().fg(heat_color(seconds, theme)),
            ));
        }
        lines.push(Line::from(spans));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  menos ", Style::default().fg(theme.muted)),
        Span::styled("■", Style::default().fg(theme.panel)),
        Span::styled("■", Style::default().fg(Color::Rgb(0x4a, 0x5a, 0x28))),
        Span::styled("■", Style::default().fg(Color::Rgb(0x7a, 0x8a, 0x2a))),
        Span::styled("■", Style::default().fg(theme.success)),
        Span::styled("■", Style::default().fg(theme.warning)),
        Span::styled(" más", Style::default().fg(theme.muted)),
    ]));

    frame.render_widget(
        Paragraph::new(lines)
            .style(theme.base())
            .block(theme.block("actividad · últimos 365 días")),
        area,
    );
}

fn heat_color(seconds: i64, theme: &Theme) -> Color {
    match activity_level(seconds) {
        0 => theme.panel,
        1 => Color::Rgb(0x4a, 0x5a, 0x28),
        2 => Color::Rgb(0x7a, 0x8a, 0x2a),
        3 => theme.success,
        _ => theme.warning,
    }
}

fn draw_code_lab(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(4)])
        .split(area);

    let solved = app
        .code_lab
        .progress
        .values()
        .filter(|progress| progress.status == ExerciseStatus::Solved)
        .count();
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  Fácil", Style::default().fg(theme.accent)),
            Span::styled(
                format!(
                    " · {} ejercicios · {solved} resueltos · Enter abre el ejercicio",
                    app.code_lab.exercises.len()
                ),
                Style::default().fg(theme.muted),
            ),
        ]))
        .style(theme.base())
        .block(theme.block("code lab")),
        rows[0],
    );

    if app.code_lab.exercises.is_empty() {
        frame.render_widget(
            Paragraph::new("No hay ejercicios. Ejecuta `basic code-lab seed`.")
                .alignment(Alignment::Center)
                .style(theme.base())
                .block(theme.block("ejercicios")),
            rows[1],
        );
        return;
    }

    let items = app
        .code_lab
        .exercises
        .iter()
        .map(|exercise| {
            let progress = app
                .code_lab
                .progress
                .get(&exercise.id)
                .map(|progress| progress.status)
                .unwrap_or(ExerciseStatus::NotStarted);
            let color = match progress {
                ExerciseStatus::Solved => theme.success,
                ExerciseStatus::Attempted | ExerciseStatus::NeedsReview => theme.warning,
                ExerciseStatus::NotStarted => theme.muted,
            };
            Line::from(vec![
                Span::styled(
                    format!(" {:<3}", exercise.level),
                    Style::default().fg(theme.muted),
                ),
                Span::styled(
                    truncate(&exercise.title, 34),
                    Style::default().fg(theme.text),
                ),
                Span::styled(
                    format!(" · {}", truncate(&exercise.topic, 20)),
                    Style::default().fg(theme.muted),
                ),
                Span::styled(
                    format!("  {}", progress.label()),
                    Style::default().fg(color),
                ),
            ])
        })
        .map(ListItem::new)
        .collect::<Vec<_>>();
    let list = List::new(items)
        .block(theme.block("ejercicios fáciles"))
        .highlight_style(
            Style::default()
                .bg(theme.selection)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▌ ");
    let mut state = ListState::default();
    state.select(Some(app.code_lab.selected));
    frame.render_stateful_widget(list, rows[1], &mut state);
}

fn draw_exercise(frame: &mut Frame, area: Rect, app: &App, ui: &UiState, theme: &Theme) {
    let Some(workspace) = &app.code_lab.workspace else {
        frame.render_widget(
            Paragraph::new("No hay un ejercicio abierto. Pulsa Esc para volver.")
                .alignment(Alignment::Center)
                .style(theme.base())
                .block(theme.block("ejercicio")),
            area,
        );
        return;
    };

    if area.width >= 100 {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
            .split(area);
        draw_exercise_detail(frame, columns[0], workspace, theme);
        draw_editor_panel(frame, columns[1], workspace, ui, theme);
    } else {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(5)])
            .split(area);
        draw_exercise_detail(frame, rows[0], workspace, theme);
        draw_editor_panel(frame, rows[1], workspace, ui, theme);
    }
}

fn draw_exercise_detail(
    frame: &mut Frame,
    area: Rect,
    workspace: &ExerciseWorkspace,
    theme: &Theme,
) {
    let exercise = &workspace.exercise;
    let status_color = match workspace.progress.status {
        ExerciseStatus::Solved => theme.success,
        ExerciseStatus::Attempted | ExerciseStatus::NeedsReview => theme.warning,
        ExerciseStatus::NotStarted => theme.muted,
    };
    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                truncate(&exercise.title, 30),
                Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" · {}", exercise.difficulty.label()),
                Style::default().fg(theme.accent),
            ),
        ]),
        Line::from(Span::styled(
            truncate(&exercise.description_md, 60),
            Style::default().fg(theme.muted),
        )),
    ];

    for example in exercise.examples.iter().take(2) {
        lines.push(Line::from(vec![
            Span::styled("  → ", Style::default().fg(theme.link)),
            Span::styled(
                truncate(&format!("{} ⇒ {}", example.input, example.output), 58),
                Style::default().fg(theme.text),
            ),
        ]));
    }
    lines.push(Line::from(vec![
        Span::styled("  casos: ", Style::default().fg(theme.muted)),
        Span::styled(
            format!(
                "{} visibles · {} ocultos",
                exercise.visible_cases().count(),
                exercise.hidden_case_count()
            ),
            Style::default().fg(theme.muted),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  estado: ", Style::default().fg(theme.muted)),
        Span::styled(
            workspace.progress.status.label(),
            Style::default().fg(status_color),
        ),
        Span::styled(
            format!(
                " · {} intento(s) · {:.0}%",
                workspace.progress.attempts,
                workspace.progress.best_score * 100.0
            ),
            Style::default().fg(theme.muted),
        ),
    ]));
    lines.extend(result_lines(workspace, theme));
    lines.push(Line::from(Span::styled(
        truncate(&workspace.status, 60),
        Style::default().fg(theme.warning),
    )));

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .style(theme.panel())
            .block(theme.block("enunciado")),
        area,
    );
}

fn result_lines(workspace: &ExerciseWorkspace, theme: &Theme) -> Vec<Line<'static>> {
    let Some(result) = &workspace.result else {
        return Vec::new();
    };
    let mut lines = vec![Line::from(vec![
        Span::styled("  resultado: ", Style::default().fg(theme.muted)),
        Span::styled(
            format!(
                "{} · {}/{} · {} ms",
                result.status.label(),
                result.tests_passed,
                result.tests_total,
                result.duration_ms
            ),
            Style::default().fg(if result.passed() {
                theme.success
            } else {
                theme.error
            }),
        ),
    ])];
    for case in result.cases.iter().filter(|case| case.visible).take(4) {
        let color = if case.passed {
            theme.success
        } else {
            theme.error
        };
        lines.push(Line::from(vec![
            Span::styled(
                if case.passed { "  ✓ " } else { "  ✗ " },
                Style::default().fg(color),
            ),
            Span::styled(
                truncate(
                    &format!(
                        "{} · esperado «{}» · real «{}»",
                        case.name, case.expected, case.actual
                    ),
                    58,
                ),
                Style::default().fg(theme.text),
            ),
        ]));
    }
    let hidden = result.cases.iter().filter(|case| !case.visible).count();
    if hidden > 0 {
        let passed = result
            .cases
            .iter()
            .filter(|case| !case.visible && case.passed)
            .count();
        lines.push(Line::from(Span::styled(
            format!("  ocultos: {passed}/{hidden} superados"),
            Style::default().fg(theme.muted),
        )));
    }
    if let Some(error) = result.first_error() {
        lines.push(Line::from(Span::styled(
            truncate(error, 58),
            Style::default().fg(theme.error),
        )));
    }
    lines
}

fn draw_editor_panel(
    frame: &mut Frame,
    area: Rect,
    workspace: &ExerciseWorkspace,
    ui: &UiState,
    theme: &Theme,
) {
    if area.height == 0 {
        return;
    }
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);
    if let Some(editor) = &ui.editor {
        editor.render(frame, rows[0]);
    } else {
        frame.render_widget(
            Paragraph::new("Preparando editor…")
                .alignment(Alignment::Center)
                .style(theme.base()),
            rows[0],
        );
    }
    let hint = if workspace.solution.is_some() {
        "Solución visible · Ctrl+V para ocultarla · no cuenta como intento"
    } else {
        "Ctrl+S guardar · Ctrl+Enter ejecutar · Ctrl+V solución · Esc volver"
    };
    frame.render_widget(
        Paragraph::new(Span::styled(
            truncate(hint, 78),
            Style::default().fg(theme.muted),
        )),
        rows[1],
    );
}

fn document_list(frame: &mut Frame, area: Rect, app: &App, title: &str, theme: &Theme) {
    if app.documents.is_empty() {
        frame.render_widget(
            Paragraph::new("Sin resultados.")
                .alignment(Alignment::Center)
                .style(theme.base())
                .block(theme.block(title)),
            area,
        );
        return;
    }

    let items = app
        .documents
        .iter()
        .map(|document| ListItem::new(document_line(document, theme)))
        .collect::<Vec<_>>();
    let list = List::new(items)
        .block(theme.block(title))
        .highlight_style(
            Style::default()
                .bg(theme.selection)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▌ ");
    let mut state = ListState::default();
    state.select(Some(app.selected.min(app.documents.len() - 1)));
    frame.render_stateful_widget(list, area, &mut state);
}

fn document_line(document: &Document, theme: &Theme) -> Line<'static> {
    let status_color = match document.status {
        LearningStatus::Pending => theme.muted,
        LearningStatus::InProgress => theme.link,
        LearningStatus::Learned => theme.success,
        LearningStatus::NeedsReview => theme.warning,
    };
    let star = if document.favorite { "★ " } else { "  " };
    Line::from(vec![
        Span::styled(star, Style::default().fg(theme.warning)),
        Span::styled(
            format!("{:<40}", truncate(&document.title, 40)),
            Style::default().fg(theme.text),
        ),
        Span::styled(
            format!(" {:<12}", document.category),
            Style::default().fg(theme.muted),
        ),
        Span::styled(
            status_marker(document.status),
            Style::default().fg(status_color),
        ),
    ])
}

fn status_marker(status: LearningStatus) -> &'static str {
    match status {
        LearningStatus::Pending => "·",
        LearningStatus::InProgress => "◐",
        LearningStatus::Learned => "✓",
        LearningStatus::NeedsReview => "!",
    }
}

fn document_preview(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let Some(document) = app.selected_document() else {
        frame.render_widget(
            Paragraph::new("Selecciona un documento.")
                .style(theme.base())
                .block(theme.block("vista previa")),
            area,
        );
        return;
    };
    let lines = vec![
        Line::from(Span::styled(
            document.title.clone(),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            document.path.clone(),
            Style::default().fg(theme.muted),
        )),
        Line::from(vec![
            Span::styled(
                document.status.label().to_string(),
                Style::default().fg(theme.link),
            ),
            Span::styled(" · ", Style::default().fg(theme.border)),
            Span::styled(document.location(), Style::default().fg(theme.muted)),
            Span::styled(" · ", Style::default().fg(theme.border)),
            Span::styled(
                format!(
                    "{} min · {} palabras",
                    document.reading_minutes, document.word_count
                ),
                Style::default().fg(theme.muted),
            ),
        ]),
        Line::from(""),
        Line::from(document.excerpt.clone()),
        Line::from(""),
        Line::from(Span::styled(
            if document.is_stub {
                "Documento vacío."
            } else {
                "Enter para abrir · e para editar desde la lectura"
            },
            Style::default().fg(theme.muted),
        )),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .style(theme.base())
            .block(theme.block("vista previa")),
        area,
    );
}

fn draw_reader(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let Some(reader) = &app.reader else {
        frame.render_widget(
            Paragraph::new("No hay ninguna nota abierta.")
                .style(theme.base())
                .block(theme.block("lectura")),
            area,
        );
        return;
    };
    let lines = markdown::render(&reader.body, theme);
    let title = format!(
        "{} · {}",
        reader.document.title,
        reader.document.status.label()
    );
    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .wrap(Wrap { trim: false })
            .scroll((reader.scroll, 0))
            .style(theme.base())
            .block(theme.block(&title)),
        area,
    );
}

fn draw_prompt(frame: &mut Frame, area: Rect, prompt: &crate::app::Prompt, theme: &Theme) {
    let width = area.width.saturating_sub(12).min(90);
    let height = 3;
    let popup = centered_rect(width, height, area);
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  > ", Style::default().fg(theme.accent)),
            Span::styled(prompt.input.clone(), Style::default().fg(theme.text)),
            Span::styled("_", Style::default().fg(theme.accent)),
        ]))
        .style(theme.panel())
        .block(theme.block(prompt.title)),
        popup,
    );
}

fn draw_help(frame: &mut Frame, area: Rect, theme: &Theme) {
    let popup = centered_rect(area.width.saturating_sub(16).min(90), 27, area);
    frame.render_widget(Clear, popup);
    let lines = vec![
        Line::from(Span::styled(
            "navegación",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  1-7        cambiar de pantalla"),
        Line::from("  Tab        siguiente pantalla"),
        Line::from("  j/k        mover o desplazar"),
        Line::from("  Enter      abrir documento"),
        Line::from("  /          buscar"),
        Line::from("  n          nueva nota"),
        Line::from("  i          reindexar el vault"),
        Line::from("  Esc        volver / limpiar"),
        Line::from("  q          salir"),
        Line::from(""),
        Line::from(Span::styled(
            "lectura y repaso",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  s          cambiar estado de aprendizaje"),
        Line::from("  f          favorito"),
        Line::from("  e          abrir en el editor externo"),
        Line::from("  p          pomodoro"),
        Line::from("  1-4        calificar repaso"),
        Line::from(""),
        Line::from(Span::styled(
            "code lab",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  7          abrir ejercicios fáciles"),
        Line::from("  Enter      abrir ejercicio seleccionado"),
        Line::from("  Ctrl+S     guardar borrador"),
        Line::from("  Ctrl+Enter ejecutar casos de prueba"),
        Line::from("  Ctrl+V     mostrar u ocultar solución"),
        Line::from(""),
        Line::from(Span::styled(
            "  ? o Esc para cerrar la ayuda",
            Style::default().fg(theme.muted),
        )),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .style(theme.panel())
            .block(theme.block("ayuda")),
        popup,
    );
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_string();
    }
    let mut result: String = value.chars().take(max.saturating_sub(1)).collect();
    result.push('…');
    result
}

#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;
    use tempfile::TempDir;

    use super::*;
    use crate::code_lab::domain::AttemptStatus;
    use crate::code_lab::runners::CaseResult;
    use crate::infra::sqlite::Database;
    use crate::paths::Paths;

    fn fixture() -> (TempDir, App) {
        let temp = TempDir::new().unwrap();
        let paths = Paths::from_dirs(
            temp.path(),
            &temp.path().join("config"),
            &temp.path().join("data"),
            &temp.path().join("state"),
            |_| None,
        );
        let mut config = Config::default_for(&paths);
        config.vault = temp.path().join("vault");
        std::fs::create_dir_all(&config.vault).unwrap();
        let app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        (temp, app)
    }

    fn render(width: u16, height: u16, app: &App, ui: &UiState) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| draw(frame, app, ui)).unwrap();
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }
    }

    #[test]
    fn code_lab_screen_lists_easy_exercises() {
        let (_temp, mut app) = fixture();
        app.open_code_lab_screen().unwrap();
        let ui = UiState::default();
        let rendered = render(110, 32, &app, &ui);
        assert!(rendered.contains("code lab"));
        assert!(rendered.contains("ejercicios fáciles"));
        assert!(rendered.contains("Suma de dos números"));
    }

    #[test]
    fn exercise_screen_shows_statement_and_editor() {
        let (_temp, mut app) = fixture();
        app.open_code_lab_screen().unwrap();
        app.open_selected_exercise().unwrap();
        let mut ui = UiState::default();
        ui.sync_editor(&app, Theme::gruvbox_dark());
        let rendered = render(110, 32, &app, &ui);
        assert!(rendered.contains("enunciado"));
        assert!(rendered.contains("Código"));
        assert!(rendered.contains("Ctrl+Enter"));
    }

    #[test]
    fn hidden_case_output_is_never_rendered() {
        let (_temp, mut app) = fixture();
        app.open_code_lab_screen().unwrap();
        app.open_selected_exercise().unwrap();
        let id = app.code_lab.workspace.as_ref().unwrap().exercise.id;
        let outcome = crate::code_lab::runners::RunOutcome {
            status: AttemptStatus::Failed,
            sdk_version: Some("10.0.400".to_string()),
            compile_ok: Some(true),
            tests_passed: 1,
            tests_total: 2,
            duration_ms: 900,
            executions: Vec::new(),
            cases: vec![CaseResult {
                name: "oculto".to_string(),
                passed: false,
                visible: false,
                expected: "SECRETO_OCULTO".to_string(),
                actual: "otro".to_string(),
                duration_ms: 10,
            }],
        };
        app.apply_outcome(id, "código", outcome).unwrap();
        let mut ui = UiState::default();
        ui.sync_editor(&app, Theme::gruvbox_dark());
        let rendered = render(110, 32, &app, &ui);
        assert!(!rendered.contains("SECRETO_OCULTO"));
        assert!(rendered.contains("ocultos: 0/1"));
    }

    #[test]
    fn editor_typing_and_save_are_wired_to_the_draft() {
        let (_temp, mut app) = fixture();
        app.open_code_lab_screen().unwrap();
        app.open_selected_exercise().unwrap();
        let mut ui = UiState::default();
        ui.sync_editor(&app, Theme::gruvbox_dark());
        let id = app.code_lab.workspace.as_ref().unwrap().exercise.id;

        handle_key(
            &mut app,
            &mut ui,
            key(KeyCode::Char('x'), KeyModifiers::empty()),
        )
        .unwrap();
        handle_key(
            &mut app,
            &mut ui,
            key(KeyCode::Char('s'), KeyModifiers::CONTROL),
        )
        .unwrap();
        let draft = crate::code_lab::app::CodeLabService::new(&mut app.database)
            .draft(id)
            .unwrap()
            .unwrap();
        assert!(draft.starts_with('x'));
        assert!(!ui.editor.as_ref().unwrap().dirty());
    }
}
