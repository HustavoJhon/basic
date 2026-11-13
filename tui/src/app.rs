//! Estado de la aplicación y transiciones entre pantallas.
//!
//! La UI es un adaptador: aquí viven las acciones y la persistencia, sin
//! depender de Ratatui ni de Crossterm.

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use crate::config::Config;
use crate::domain::{ActivityDay, DashboardStats, Document, LearnMode, LearningStatus};
use crate::error::BasicError;
use crate::infra::catalog;
use crate::infra::index;
use crate::infra::notes;
use crate::infra::sqlite::Database;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    Explore,
    Reader,
    Learn,
    Review,
    Pomodoro,
    Stats,
}

impl Screen {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "inicio",
            Self::Explore => "explorar",
            Self::Reader => "lectura",
            Self::Learn => "aprender",
            Self::Review => "repaso",
            Self::Pomodoro => "pomodoro",
            Self::Stats => "estadísticas",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PomodoroPhase {
    Focus,
    ShortBreak,
    LongBreak,
}

impl PomodoroPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Focus => "focus",
            Self::ShortBreak => "short_break",
            Self::LongBreak => "long_break",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Focus => "concentración",
            Self::ShortBreak => "descanso corto",
            Self::LongBreak => "descanso largo",
        }
    }
}

#[derive(Debug)]
pub struct ReaderState {
    pub document: Document,
    pub body: String,
    pub scroll: u16,
}

#[derive(Debug)]
pub struct Pomodoro {
    pub phase: PomodoroPhase,
    pub planned_seconds: i64,
    pub remaining_seconds: i64,
    pub running: bool,
    pub cycle: i64,
    pub session_id: Option<i64>,
    pub document_id: Option<i64>,
    pub category: Option<String>,
    last_tick: Instant,
}

impl Pomodoro {
    fn new(config: &Config) -> Self {
        let phase = PomodoroPhase::Focus;
        let seconds = phase_seconds(config, phase);
        Self {
            phase,
            planned_seconds: seconds,
            remaining_seconds: seconds,
            running: false,
            cycle: 0,
            session_id: None,
            document_id: None,
            category: None,
            last_tick: Instant::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    NewNote,
}

#[derive(Debug)]
pub struct Prompt {
    pub kind: PromptKind,
    pub title: &'static str,
    pub input: String,
}

/// Estado completo de la TUI.
pub struct App {
    pub config: Config,
    pub database: Database,
    pub screen: Screen,
    pub previous: Vec<Screen>,
    pub documents: Vec<Document>,
    pub selected: usize,
    pub query: String,
    pub search_mode: bool,
    pub reader: Option<ReaderState>,
    pub learn_mode: LearnMode,
    pub pomodoro: Pomodoro,
    pub stats: DashboardStats,
    pub activity: Vec<ActivityDay>,
    pub recent: Vec<Document>,
    pub review_queue: Vec<Document>,
    pub review_index: usize,
    pub prompt: Option<Prompt>,
    pub help_visible: bool,
    pub status: String,
    pub should_quit: bool,
    pub editor_request: Option<PathBuf>,
}

impl App {
    pub fn new(config: Config, database: Database) -> Result<Self, BasicError> {
        let pomodoro = Pomodoro::new(&config);
        let mut app = Self {
            config,
            database,
            screen: Screen::Dashboard,
            previous: Vec::new(),
            documents: Vec::new(),
            selected: 0,
            query: String::new(),
            search_mode: false,
            reader: None,
            learn_mode: LearnMode::Continue,
            pomodoro,
            stats: DashboardStats::default(),
            activity: Vec::new(),
            recent: Vec::new(),
            review_queue: Vec::new(),
            review_index: 0,
            prompt: None,
            help_visible: false,
            status: "Listo. Pulsa ? para ver los atajos.".to_string(),
            should_quit: false,
            editor_request: None,
        };
        app.recover_pomodoro()?;
        app.refresh()?;
        Ok(app)
    }

    /// Si el proceso anterior se cerró a mitad de una sesión, la reanuda en
    /// pausa; si ya habría terminado, la cierra sin duplicar el tiempo.
    fn recover_pomodoro(&mut self) -> Result<(), BasicError> {
        let Some((session_id, phase, cycle, started_at, planned, document_id, category)) =
            catalog::pomodoro_runtime(&self.database)?
        else {
            return Ok(());
        };
        let elapsed = (catalog::unix_now() - started_at).max(0);
        let remaining = (planned - elapsed).max(0);
        eprintln!(
            "recover: session={session_id} phase={phase} cycle={cycle} elapsed={elapsed} planned={planned} remaining={remaining}"
        );
        let Some(phase) = phase_from_str(&phase) else {
            catalog::finish_pomodoro(
                &self.database,
                session_id,
                0,
                "cancelled",
                &phase,
                document_id,
                category.as_deref(),
            )?;
            return Ok(());
        };

        if remaining > 0 && phase == PomodoroPhase::Focus {
            self.pomodoro = Pomodoro {
                phase,
                planned_seconds: planned,
                remaining_seconds: remaining,
                running: false,
                cycle: cycle.saturating_sub(1),
                session_id: Some(session_id),
                document_id,
                category,
                last_tick: Instant::now(),
            };
            self.status = "Sesión de Pomodoro recuperada en pausa.".to_string();
        } else {
            let completed = remaining == 0 && phase == PomodoroPhase::Focus;
            catalog::finish_pomodoro(
                &self.database,
                session_id,
                if completed {
                    planned
                } else {
                    elapsed.min(planned)
                },
                if completed { "completed" } else { "cancelled" },
                phase.as_str(),
                document_id,
                category.as_deref(),
            )?;
        }
        Ok(())
    }

    /// Recarga los datos de la pantalla actual.
    pub fn refresh(&mut self) -> Result<(), BasicError> {
        self.stats = catalog::dashboard_stats(&self.database)?;
        self.activity = catalog::activity_days(&self.database, 365)?;
        self.recent = catalog::recent_documents(&self.database, 8)?;

        self.documents = match self.screen {
            Screen::Explore => {
                if self.query.trim().is_empty() {
                    catalog::list_documents(&self.database, Some(500))?
                } else {
                    catalog::search_documents(
                        &self.database,
                        &self.query,
                        crate::domain::SearchScope::All,
                    )?
                }
            }
            Screen::Learn => {
                let mut documents = catalog::learn_documents(&self.database, self.learn_mode)?;
                if self.learn_mode == LearnMode::Random && documents.len() > 1 {
                    let offset = (catalog::unix_now() as usize) % documents.len();
                    documents.rotate_left(offset);
                }
                documents
            }
            Screen::Review => {
                self.review_queue = catalog::due_documents(&self.database)?;
                self.review_queue.clone()
            }
            Screen::Dashboard => self.recent.clone(),
            Screen::Pomodoro | Screen::Stats => catalog::list_documents(&self.database, Some(20))?,
            Screen::Reader => Vec::new(),
        };

        if self.documents.is_empty() {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(self.documents.len() - 1);
        }
        if self.review_queue.is_empty() {
            self.review_index = 0;
        } else {
            self.review_index = self.review_index.min(self.review_queue.len() - 1);
        }
        Ok(())
    }

    pub fn set_screen(&mut self, screen: Screen) -> Result<(), BasicError> {
        if screen == self.screen {
            return Ok(());
        }
        self.previous.push(self.screen);
        if self.previous.len() > 12 {
            self.previous.remove(0);
        }
        self.screen = screen;
        self.search_mode = false;
        self.selected = 0;
        self.refresh()
    }

    pub fn back(&mut self) -> Result<(), BasicError> {
        if self.search_mode {
            self.search_mode = false;
            return Ok(());
        }
        if self.help_visible {
            self.help_visible = false;
            return Ok(());
        }
        if self.screen == Screen::Reader {
            self.reader = None;
        }
        let target = self.previous.pop().unwrap_or(Screen::Dashboard);
        self.screen = target;
        self.refresh()
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.documents.is_empty() {
            return;
        }
        let len = self.documents.len() as isize;
        let next = (self.selected as isize + delta).rem_euclid(len);
        self.selected = next as usize;
    }

    pub fn selected_document(&self) -> Option<&Document> {
        self.documents.get(self.selected)
    }

    pub fn open_selected(&mut self) -> Result<(), BasicError> {
        let Some(document) = self.selected_document() else {
            self.status = "No hay documentos que abrir.".to_string();
            return Ok(());
        };
        self.open_document(document.id)
    }

    pub fn open_document(&mut self, id: i64) -> Result<(), BasicError> {
        catalog::record_visit(&self.database, id)?;
        let Some(document) = catalog::get_document(&self.database, id)? else {
            self.status = "El documento ya no existe en el índice.".to_string();
            return Ok(());
        };
        let path = self.config.vault.join(&document.path);
        let body = match fs::read_to_string(&path) {
            Ok(body) => body,
            Err(error) => {
                self.status = format!("No se pudo leer {}: {error}", document.path);
                return Ok(());
            }
        };
        self.status = format!("Abierto: {}", document.title);
        self.reader = Some(ReaderState {
            document,
            body,
            scroll: 0,
        });
        self.screen = Screen::Reader;
        self.stats = catalog::dashboard_stats(&self.database)?;
        Ok(())
    }

    pub fn reload_reader(&mut self) -> Result<(), BasicError> {
        let Some(reader) = &mut self.reader else {
            return Ok(());
        };
        let path = self.config.vault.join(&reader.document.path);
        match fs::read_to_string(&path) {
            Ok(body) => {
                reader.body = body;
                self.status = "Nota recargada.".to_string();
            }
            Err(error) => self.status = format!("No se pudo recargar: {error}"),
        }
        Ok(())
    }

    pub fn reader_scroll(&mut self, delta: i32) {
        if let Some(reader) = &mut self.reader {
            let next = reader.scroll as i32 + delta;
            reader.scroll = next.max(0) as u16;
        }
    }

    pub fn cycle_reader_status(&mut self) -> Result<(), BasicError> {
        let Some(reader) = &mut self.reader else {
            return Ok(());
        };
        let next = match reader.document.status {
            LearningStatus::Pending => LearningStatus::InProgress,
            LearningStatus::InProgress => LearningStatus::Learned,
            LearningStatus::Learned => LearningStatus::NeedsReview,
            LearningStatus::NeedsReview => LearningStatus::InProgress,
        };
        catalog::set_status(&self.database, reader.document.id, next)?;
        reader.document.status = next;
        self.status = format!("Estado: {}", next.label());
        self.stats = catalog::dashboard_stats(&self.database)?;
        Ok(())
    }

    pub fn toggle_favorite(&mut self) -> Result<(), BasicError> {
        let Some(reader) = &mut self.reader else {
            return Ok(());
        };
        let favorite = catalog::toggle_favorite(&self.database, reader.document.id)?;
        reader.document.favorite = favorite;
        self.status = if favorite {
            "Añadido a favoritos.".to_string()
        } else {
            "Quitado de favoritos.".to_string()
        };
        Ok(())
    }

    pub fn request_editor(&mut self) {
        if let Some(reader) = &self.reader {
            self.editor_request = Some(self.config.vault.join(&reader.document.path));
        }
    }

    pub fn search_push(&mut self, character: char) {
        self.query.push(character);
        self.search_mode = true;
        self.screen = Screen::Explore;
        let _ = self.refresh();
    }

    pub fn search_pop(&mut self) {
        self.query.pop();
        let _ = self.refresh();
    }

    pub fn search_clear(&mut self) {
        self.query.clear();
        self.search_mode = false;
        let _ = self.refresh();
    }

    pub fn cycle_learn_mode(&mut self) -> Result<(), BasicError> {
        self.learn_mode = self.learn_mode.next();
        self.selected = 0;
        self.refresh()
    }

    pub fn set_learn_mode(&mut self, mode: LearnMode) -> Result<(), BasicError> {
        self.learn_mode = mode;
        self.selected = 0;
        self.refresh()
    }

    /// Califica el documento actual de la cola de repaso.
    pub fn rate_review(&mut self, rating: crate::domain::ReviewRating) -> Result<(), BasicError> {
        let Some(document) = self.review_queue.get(self.review_index).cloned() else {
            self.status = "No hay repasos pendientes.".to_string();
            return Ok(());
        };
        let outcome = catalog::record_review(&self.database, document.id, rating)?;
        self.status = format!(
            "{}: {} → próximo repaso en {} días",
            document.title,
            rating.label(),
            outcome.due_in_days
        );
        if self.review_index + 1 < self.review_queue.len() {
            self.review_index += 1;
        } else {
            self.review_queue.clear();
            self.review_index = 0;
            self.status.push_str(" · cola completada");
        }
        self.stats = catalog::dashboard_stats(&self.database)?;
        Ok(())
    }

    pub fn start_new_note_prompt(&mut self) {
        self.prompt = Some(Prompt {
            kind: PromptKind::NewNote,
            title: "Nueva nota (ruta relativa, Enter para crear)",
            input: String::new(),
        });
    }

    pub fn prompt_push(&mut self, character: char) {
        if let Some(prompt) = &mut self.prompt {
            prompt.input.push(character);
        }
    }

    pub fn prompt_pop(&mut self) {
        if let Some(prompt) = &mut self.prompt {
            prompt.input.pop();
        }
    }

    pub fn prompt_cancel(&mut self) {
        self.prompt = None;
    }

    pub fn prompt_submit(&mut self) -> Result<(), BasicError> {
        let Some(prompt) = self.prompt.take() else {
            return Ok(());
        };
        match prompt.kind {
            PromptKind::NewNote => {
                let created = notes::create_note(&self.config, &prompt.input, None)?;
                let report = index::index_vault(&mut self.database, &self.config)?;
                self.status = format!("Nota creada: {} · {}", created.relative, report.summary());
                let id = self
                    .database
                    .connection()
                    .query_row(
                        "SELECT id FROM documents WHERE path = ?1",
                        [&created.relative],
                        |row| row.get::<_, i64>(0),
                    )
                    .ok();
                if let Some(id) = id {
                    self.open_document(id)?;
                } else {
                    self.refresh()?;
                }
            }
        }
        Ok(())
    }

    pub fn toggle_pomodoro(&mut self) -> Result<(), BasicError> {
        if self.pomodoro.running {
            self.pomodoro.running = false;
            self.status = "Pomodoro en pausa.".to_string();
            return Ok(());
        }

        if self.pomodoro.session_id.is_none() {
            let (document_id, category) = if let Some(reader) = &self.reader {
                (
                    Some(reader.document.id),
                    Some(reader.document.category.clone()),
                )
            } else if let Some(document) = self.selected_document() {
                (Some(document.id), Some(document.category.clone()))
            } else {
                (None, None)
            };
            self.pomodoro.document_id = document_id;
            self.pomodoro.category = category;
            self.pomodoro.session_id = Some(catalog::start_pomodoro(
                &self.database,
                self.pomodoro.phase.as_str(),
                self.pomodoro.planned_seconds,
                self.pomodoro.cycle + 1,
                self.pomodoro.document_id,
                self.pomodoro.category.as_deref(),
            )?);
        }
        self.pomodoro.running = true;
        self.pomodoro.last_tick = Instant::now();
        self.status = format!("{} en marcha.", self.pomodoro.phase.label());
        Ok(())
    }

    pub fn reset_pomodoro(&mut self) -> Result<(), BasicError> {
        self.finish_pomodoro("cancelled")?;
        self.pomodoro.phase = PomodoroPhase::Focus;
        self.pomodoro.running = false;
        self.pomodoro.cycle = 0;
        self.reset_phase_duration();
        self.status = "Pomodoro reiniciado.".to_string();
        Ok(())
    }

    pub fn skip_pomodoro(&mut self) -> Result<(), BasicError> {
        self.finish_pomodoro("cancelled")?;
        self.advance_phase(false)?;
        self.status = "Fase saltada.".to_string();
        Ok(())
    }

    /// Avanza el reloj del Pomodoro; la UI llama a este método cada 250 ms.
    pub fn tick(&mut self) -> Result<(), BasicError> {
        if !self.pomodoro.running {
            return Ok(());
        }
        let elapsed = self.pomodoro.last_tick.elapsed().as_secs() as i64;
        if elapsed <= 0 {
            return Ok(());
        }
        self.pomodoro.last_tick = Instant::now();
        self.pomodoro.remaining_seconds -= elapsed;
        if self.pomodoro.remaining_seconds <= 0 {
            self.pomodoro.remaining_seconds = 0;
            self.finish_pomodoro("completed")?;
            self.advance_phase(true)?;
        }
        Ok(())
    }

    fn finish_pomodoro(&mut self, status: &str) -> Result<(), BasicError> {
        let Some(session_id) = self.pomodoro.session_id.take() else {
            return Ok(());
        };
        let actual = (self.pomodoro.planned_seconds - self.pomodoro.remaining_seconds).max(0);
        catalog::finish_pomodoro(
            &self.database,
            session_id,
            actual,
            status,
            self.pomodoro.phase.as_str(),
            self.pomodoro.document_id,
            self.pomodoro.category.as_deref(),
        )?;
        if status == "completed" {
            self.stats = catalog::dashboard_stats(&self.database)?;
        }
        Ok(())
    }

    fn advance_phase(&mut self, completed: bool) -> Result<(), BasicError> {
        if self.pomodoro.phase == PomodoroPhase::Focus {
            if completed {
                self.pomodoro.cycle += 1;
            }
            let long = completed
                && self.pomodoro.cycle > 0
                && self.pomodoro.cycle % self.config.pomodoro.long_break_every as i64 == 0;
            self.pomodoro.phase = if long {
                PomodoroPhase::LongBreak
            } else {
                PomodoroPhase::ShortBreak
            };
        } else {
            self.pomodoro.phase = PomodoroPhase::Focus;
        }
        self.reset_phase_duration();
        self.pomodoro.running = self.config.pomodoro.auto_start_next;
        if self.pomodoro.running {
            self.pomodoro.session_id = Some(catalog::start_pomodoro(
                &self.database,
                self.pomodoro.phase.as_str(),
                self.pomodoro.planned_seconds,
                self.pomodoro.cycle + 1,
                self.pomodoro.document_id,
                self.pomodoro.category.as_deref(),
            )?);
            self.pomodoro.last_tick = Instant::now();
        }
        Ok(())
    }

    fn reset_phase_duration(&mut self) {
        let seconds = phase_seconds(&self.config, self.pomodoro.phase);
        self.pomodoro.planned_seconds = seconds;
        self.pomodoro.remaining_seconds = seconds;
    }
}

fn phase_from_str(value: &str) -> Option<PomodoroPhase> {
    match value {
        "focus" => Some(PomodoroPhase::Focus),
        "short_break" => Some(PomodoroPhase::ShortBreak),
        "long_break" => Some(PomodoroPhase::LongBreak),
        _ => None,
    }
}

pub fn phase_seconds(config: &Config, phase: PomodoroPhase) -> i64 {
    let minutes = match phase {
        PomodoroPhase::Focus => config.pomodoro.focus_minutes,
        PomodoroPhase::ShortBreak => config.pomodoro.short_break_minutes,
        PomodoroPhase::LongBreak => config.pomodoro.long_break_minutes,
    };
    minutes as i64 * 60
}

pub fn format_clock(seconds: i64) -> String {
    let seconds = seconds.max(0);
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::paths::Paths;

    fn fixture() -> (TempDir, Config) {
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
        fs::create_dir_all(&config.vault).unwrap();
        fs::write(config.vault.join("nota.md"), "# Nota\n\nContenido.\n").unwrap();
        (temp, config)
    }

    #[test]
    fn recovers_an_interrupted_focus_session_in_pause() {
        let (_temp, config) = fixture();
        let database = Database::open_in_memory().unwrap();
        let session = catalog::start_pomodoro(&database, "focus", 300, 1, None, None).unwrap();

        let app = App::new(config, database).unwrap();
        assert_eq!(app.pomodoro.session_id, Some(session));
        assert!(!app.pomodoro.running);
        assert!(app.pomodoro.remaining_seconds > 0);
        assert!(app.status.contains("recuperada"));
    }

    #[test]
    fn closes_an_expired_focus_session_and_records_its_time() {
        let (_temp, config) = fixture();
        let database = Database::open_in_memory().unwrap();
        let session = catalog::start_pomodoro(&database, "focus", 300, 1, None, None).unwrap();
        database
            .connection()
            .execute(
                "UPDATE pomodoro_runtime SET started_at = started_at - 400 WHERE session_id = ?1",
                [session],
            )
            .unwrap();

        let app = App::new(config, database).unwrap();
        assert!(app.pomodoro.session_id.is_none());
        let recorded: i64 = app
            .database
            .connection()
            .query_row(
                "SELECT COALESCE(sum(study_seconds), 0) FROM daily_activity",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(recorded, 300, "actividad registrada");
        assert_eq!(app.stats.study_seconds_today, 300);
        assert!(catalog::pomodoro_runtime(&app.database).unwrap().is_none());
    }

    #[test]
    fn clock_is_zero_padded() {
        assert_eq!(format_clock(0), "00:00");
        assert_eq!(format_clock(65), "01:05");
    }
}
