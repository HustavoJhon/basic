//! Estado de la aplicación y transiciones entre pantallas.
//!
//! La UI es un adaptador: aquí viven las acciones y la persistencia, sin
//! depender de Ratatui ni de Crossterm.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use crate::code_lab::app::{CodeLabService, LessonView};
use crate::code_lab::domain::{
    AttemptSummary, CodeLabItem, CodeLabStats, ConceptReview, DEFAULT_LANGUAGE, DailyProgress,
    Difficulty, Exercise, ExerciseProgress, ItemType, NoteBacklink, NoteItemType, NoteLink,
    QuizResult, note_template,
};
use crate::code_lab::runners::{self, RunOutcome, RunRequest};
use crate::config::Config;
use crate::domain::{
    ActivityDay, DashboardStats, Document, LearnMode, LearningStatus, ReviewRating,
};
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
    CodeLab,
    Exercise,
    Lesson,
    ConceptReview,
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
            Self::CodeLab => "code lab",
            Self::Exercise => "ejercicio",
            Self::Lesson => "lección",
            Self::ConceptReview => "repaso de conceptos",
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
    /// Elemento de Code Lab del que procede la nota, si la nota lo declara.
    pub source: Option<NoteBacklink>,
}

/// Espacio de trabajo de un ejercicio de Code Lab.
#[derive(Debug)]
pub struct ExerciseWorkspace {
    pub exercise: Exercise,
    pub progress: ExerciseProgress,
    /// Código actual del borrador. La UI lo actualiza antes de guardar o ejecutar.
    pub code: String,
    pub result: Option<RunOutcome>,
    pub solution: Option<String>,
    pub status: String,
}

/// Espacio de trabajo de una lección con su cuestionario.
#[derive(Debug)]
pub struct LessonWorkspace {
    pub view: LessonView,
    pub answers: Vec<Option<usize>>,
    pub question: usize,
    pub content_scroll: usize,
    pub result: Option<QuizResult>,
    pub status: String,
}

/// Estado de la práctica. El editor vive en la UI; aquí solo está su texto.
#[derive(Debug)]
pub struct CodeLabState {
    pub exercises: Vec<Exercise>,
    pub progress: HashMap<i64, ExerciseProgress>,
    pub stats: CodeLabStats,
    pub daily: DailyProgress,
    pub items: Vec<CodeLabItem>,
    pub concepts: Vec<ConceptReview>,
    pub concept_index: usize,
    pub difficulty: Difficulty,
    pub selected: usize,
    pub workspace: Option<ExerciseWorkspace>,
    pub lesson: Option<LessonWorkspace>,
    pub pending_run: bool,
}

impl Default for CodeLabState {
    fn default() -> Self {
        Self {
            exercises: Vec::new(),
            progress: HashMap::new(),
            stats: CodeLabStats::default(),
            daily: DailyProgress::default(),
            items: Vec::new(),
            concepts: Vec::new(),
            concept_index: 0,
            difficulty: Difficulty::Easy,
            selected: 0,
            workspace: None,
            lesson: None,
            pending_run: false,
        }
    }
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

/// Datos necesarios para crear una nota enlazada con un elemento de Code Lab.
#[derive(Debug, Clone)]
pub struct CodeLabNoteDraft {
    pub item_type: NoteItemType,
    pub item_id: i64,
    pub title: String,
    pub body: String,
    pub default_path: String,
}

#[derive(Debug, Clone)]
pub enum PromptKind {
    NewNote,
    CodeLabNote(CodeLabNoteDraft),
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
    pub code_lab: CodeLabState,
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
            code_lab: CodeLabState::default(),
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
            Screen::Reader
            | Screen::CodeLab
            | Screen::Exercise
            | Screen::Lesson
            | Screen::ConceptReview => Vec::new(),
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
        self.refresh()?;
        if self.needs_code_lab_refresh() {
            self.refresh_code_lab()?;
        }
        Ok(())
    }

    fn needs_code_lab_refresh(&self) -> bool {
        matches!(
            self.screen,
            Screen::CodeLab | Screen::Stats | Screen::Review | Screen::ConceptReview
        ) || (self.screen == Screen::Learn && self.learn_mode == LearnMode::CodeLab)
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
        let source = NoteBacklink::parse(&body);
        self.reader = Some(ReaderState {
            document,
            body,
            scroll: 0,
            source,
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
                reader.source = NoteBacklink::parse(&body);
                reader.body = body;
                self.status = "Nota recargada.".to_string();
            }
            Err(error) => self.status = format!("No se pudo recargar: {error}"),
        }
        Ok(())
    }

    /// Vuelve a la lección, ejercicio o concepto que originó la nota abierta.
    pub fn open_note_source(&mut self) -> Result<(), BasicError> {
        let Some(source) = self
            .reader
            .as_ref()
            .and_then(|reader| reader.source.clone())
        else {
            self.status = "Esta nota no tiene un origen de Code Lab.".to_string();
            return Ok(());
        };
        match source.item_type {
            NoteItemType::Lesson => self.open_lesson(source.item_id),
            NoteItemType::Exercise => self.open_exercise(source.item_id),
            NoteItemType::Concept => {
                self.refresh_code_lab()?;
                self.open_concept_review_screen()?;
                if let Some(index) = self
                    .code_lab
                    .concepts
                    .iter()
                    .position(|concept| concept.id == source.item_id)
                {
                    self.code_lab.concept_index = index;
                    self.status = format!(
                        "Concepto abierto: {}",
                        self.code_lab.concepts[index].concept
                    );
                } else {
                    self.status = "El concepto no está pendiente de repaso.".to_string();
                }
                Ok(())
            }
        }
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
        self.refresh()?;
        if self.needs_code_lab_refresh() {
            self.refresh_code_lab()?;
        }
        Ok(())
    }

    pub fn set_learn_mode(&mut self, mode: LearnMode) -> Result<(), BasicError> {
        self.learn_mode = mode;
        self.selected = 0;
        self.refresh()?;
        if self.needs_code_lab_refresh() {
            self.refresh_code_lab()?;
        }
        Ok(())
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

    /// Recarga el catálogo y el progreso de todas las dificultades.
    pub fn refresh_code_lab(&mut self) -> Result<(), BasicError> {
        let lesson_target = self.config.code_lab.daily_lessons;
        let exercise_target = self.config.code_lab.daily_exercises;
        let mut service = CodeLabService::new(&mut self.database);
        service.seed()?;
        let all = service.exercises()?;
        let stats = service.stats()?;
        let daily = service.daily_progress(lesson_target, exercise_target)?;
        let items = service.learning_items()?;
        let concepts = service.due_concepts()?;
        let mut progress = HashMap::new();
        for exercise in &all {
            progress.insert(exercise.id, service.exercise_progress(exercise.id)?);
        }
        self.code_lab.exercises = all
            .into_iter()
            .filter(|exercise| exercise.difficulty == self.code_lab.difficulty)
            .collect();
        self.code_lab.progress = progress;
        self.code_lab.stats = stats;
        self.code_lab.daily = daily;
        self.code_lab.items = items;
        self.code_lab.concepts = concepts;
        if self.code_lab.concepts.is_empty() {
            self.code_lab.concept_index = 0;
        } else {
            self.code_lab.concept_index = self
                .code_lab
                .concept_index
                .min(self.code_lab.concepts.len() - 1);
        }
        if self.code_lab.exercises.is_empty() {
            self.code_lab.selected = 0;
        } else {
            self.code_lab.selected = self
                .code_lab
                .selected
                .min(self.code_lab.exercises.len() - 1);
        }
        Ok(())
    }

    pub fn open_code_lab_screen(&mut self) -> Result<(), BasicError> {
        self.set_screen(Screen::CodeLab)
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) -> Result<(), BasicError> {
        if self.code_lab.difficulty == difficulty {
            return Ok(());
        }
        self.code_lab.difficulty = difficulty;
        self.code_lab.selected = 0;
        self.refresh_code_lab()?;
        self.status = format!("Dificultad: {}.", difficulty.label());
        Ok(())
    }

    pub fn move_exercise_selection(&mut self, delta: isize) {
        if self.code_lab.exercises.is_empty() {
            return;
        }
        let len = self.code_lab.exercises.len() as isize;
        self.code_lab.selected = (self.code_lab.selected as isize + delta).rem_euclid(len) as usize;
    }

    pub fn open_selected_exercise(&mut self) -> Result<(), BasicError> {
        let Some(exercise) = self.code_lab.exercises.get(self.code_lab.selected).cloned() else {
            self.status = "No hay ejercicios disponibles.".to_string();
            return Ok(());
        };
        self.open_exercise(exercise.id)
    }

    pub fn open_exercise(&mut self, exercise_id: i64) -> Result<(), BasicError> {
        let (exercise, progress, code) = {
            let mut service = CodeLabService::new(&mut self.database);
            let Some(exercise) = service.exercise_detail(exercise_id)? else {
                self.status = "El ejercicio ya no existe.".to_string();
                return Ok(());
            };
            let progress = service.open_exercise(exercise_id)?;
            let code = service
                .draft(exercise_id)?
                .unwrap_or_else(|| exercise.starter_code.clone());
            (exercise, progress, code)
        };
        let title = exercise.title.clone();
        self.code_lab.workspace = Some(ExerciseWorkspace {
            exercise,
            progress,
            code,
            result: None,
            solution: None,
            status: "Listo. Ctrl+Enter ejecuta, Ctrl+S guarda el borrador.".to_string(),
        });
        if self.screen != Screen::Exercise {
            self.previous.push(self.screen);
        }
        self.screen = Screen::Exercise;
        self.status = format!("Ejercicio abierto: {title}");
        Ok(())
    }

    /// Guarda el código que hay ahora mismo en el editor.
    pub fn save_exercise_code(&mut self, code: &str) -> Result<(), BasicError> {
        let Some(workspace) = &mut self.code_lab.workspace else {
            return Ok(());
        };
        let id = workspace.exercise.id;
        CodeLabService::new(&mut self.database).save_draft(id, code)?;
        workspace.code = code.to_string();
        workspace.status = "Borrador guardado.".to_string();
        self.status = "Borrador guardado.".to_string();
        Ok(())
    }

    /// Marca la ejecución para que el bucle dibuje el estado «ejecutando»
    /// antes de bloquear la terminal con el SDK.
    pub fn request_exercise_run(&mut self, code: &str) {
        let Some(workspace) = &mut self.code_lab.workspace else {
            return;
        };
        workspace.code = code.to_string();
        self.code_lab.pending_run = true;
        self.status = "Ejecutando ejercicio…".to_string();
    }

    pub fn run_pending(&mut self) -> Result<(), BasicError> {
        if !self.code_lab.pending_run {
            return Ok(());
        }
        self.code_lab.pending_run = false;
        self.run_exercise()
    }

    fn run_exercise(&mut self) -> Result<(), BasicError> {
        let Some(workspace) = &self.code_lab.workspace else {
            return Ok(());
        };
        let exercise = workspace.exercise.clone();
        let code = workspace.code.clone();
        let runner =
            runners::runner_for(&exercise.language).ok_or_else(|| BasicError::InvalidConfig {
                path: "code_lab/runners".into(),
                message: format!("lenguaje no soportado: {}", exercise.language),
            })?;
        let request = RunRequest {
            exercise_id: exercise.id,
            language: exercise.language.clone(),
            code: code.clone(),
            test_spec: exercise.test_spec.clone(),
            timeout_secs: exercise.test_spec.timeout_secs,
            vault: self.config.vault.clone(),
        };
        let outcome = runner.run(&request)?;
        self.apply_outcome(exercise.id, &code, outcome)
    }

    pub(crate) fn apply_outcome(
        &mut self,
        exercise_id: i64,
        code: &str,
        outcome: RunOutcome,
    ) -> Result<(), BasicError> {
        let summary = AttemptSummary::from(&outcome);
        let passed = outcome.passed();
        {
            let mut service = CodeLabService::new(&mut self.database);
            service.save_draft(exercise_id, code)?;
            service.record_attempt(exercise_id, code, &summary)?;
        }
        if let Some(workspace) = &mut self.code_lab.workspace {
            workspace.code = code.to_string();
            workspace.result = Some(outcome);
            workspace.status = if passed {
                "Todos los casos superados.".to_string()
            } else {
                "Revisa los casos fallidos.".to_string()
            };
            workspace.progress =
                CodeLabService::new(&mut self.database).exercise_progress(exercise_id)?;
        }
        self.refresh_code_lab()?;
        self.status = if passed {
            "Ejercicio superado.".to_string()
        } else {
            "Intento registrado; revisa los casos.".to_string()
        };
        Ok(())
    }

    /// Muestra u oculta la solución de referencia solo bajo petición.
    pub fn toggle_solution(&mut self) -> Result<(), BasicError> {
        let Some(workspace) = &mut self.code_lab.workspace else {
            return Ok(());
        };
        if workspace.solution.is_some() {
            workspace.solution = None;
            return Ok(());
        }
        let id = workspace.exercise.id;
        let solution = CodeLabService::new(&mut self.database).solution(id)?;
        workspace.solution = solution;
        Ok(())
    }

    pub fn open_concept_review_screen(&mut self) -> Result<(), BasicError> {
        self.set_screen(Screen::ConceptReview)
    }

    pub fn move_concept_selection(&mut self, delta: isize) {
        if self.code_lab.concepts.is_empty() {
            return;
        }
        let len = self.code_lab.concepts.len() as isize;
        self.code_lab.concept_index =
            (self.code_lab.concept_index as isize + delta).rem_euclid(len) as usize;
    }

    /// Califica el concepto actual y lo saca de la cola de repaso.
    pub fn rate_current_concept(&mut self, rating: ReviewRating) -> Result<(), BasicError> {
        let Some(concept) = self
            .code_lab
            .concepts
            .get(self.code_lab.concept_index)
            .cloned()
        else {
            self.status = "No hay conceptos pendientes.".to_string();
            return Ok(());
        };
        let index = self.code_lab.concept_index;
        let outcome = CodeLabService::new(&mut self.database).rate_concept(concept.id, rating)?;
        self.code_lab.concepts.remove(index);
        if index >= self.code_lab.concepts.len() && !self.code_lab.concepts.is_empty() {
            self.code_lab.concept_index = self.code_lab.concepts.len() - 1;
        }
        self.code_lab.stats = CodeLabService::new(&mut self.database).stats()?;
        self.status = format!(
            "{}: próximo repaso en {} días.",
            concept.concept, outcome.due_in_days
        );
        Ok(())
    }

    pub fn move_learning_selection(&mut self, delta: isize) {
        if self.code_lab.items.is_empty() {
            return;
        }
        let len = self.code_lab.items.len() as isize;
        self.selected = (self.selected as isize + delta).rem_euclid(len) as usize;
    }

    /// Abre el elemento seleccionado en Learning Mode.
    pub fn open_learning_item(&mut self) -> Result<(), BasicError> {
        let Some(item) = self.code_lab.items.get(self.selected).cloned() else {
            self.status = "No hay elementos de Code Lab pendientes.".to_string();
            return Ok(());
        };
        match item.kind {
            ItemType::Exercise => self.open_exercise(item.id),
            ItemType::Lesson => self.open_lesson(item.id),
        }
    }

    pub fn open_lesson(&mut self, lesson_id: i64) -> Result<(), BasicError> {
        let view = {
            let mut service = CodeLabService::new(&mut self.database);
            let Some(view) = service.lesson_view(lesson_id)? else {
                self.status = "La lección ya no existe.".to_string();
                return Ok(());
            };
            service.open_lesson(lesson_id)?;
            view
        };
        let answers = vec![None; view.detail.questions.len()];
        let title = view.detail.lesson.title.clone();
        self.code_lab.lesson = Some(LessonWorkspace {
            view,
            answers,
            question: 0,
            content_scroll: 0,
            result: None,
            status: "Responde las preguntas y pulsa Enter para enviar.".to_string(),
        });
        if self.screen != Screen::Lesson {
            self.previous.push(self.screen);
        }
        self.screen = Screen::Lesson;
        self.status = format!("Lección abierta: {title}");
        Ok(())
    }

    pub fn move_lesson_question(&mut self, delta: isize) {
        let Some(lesson) = &mut self.code_lab.lesson else {
            return;
        };
        if lesson.view.detail.questions.is_empty() {
            return;
        }
        let len = lesson.view.detail.questions.len() as isize;
        lesson.question = (lesson.question as isize + delta).rem_euclid(len) as usize;
    }

    pub fn select_lesson_answer(&mut self, answer: usize) {
        let Some(lesson) = &mut self.code_lab.lesson else {
            return;
        };
        if lesson
            .view
            .detail
            .questions
            .get(lesson.question)
            .is_some_and(|question| answer < question.options.len())
        {
            lesson.answers[lesson.question] = Some(answer);
        }
    }

    pub fn scroll_lesson_content(&mut self, delta: isize) {
        if let Some(lesson) = &mut self.code_lab.lesson {
            lesson.content_scroll = (lesson.content_scroll as isize + delta).max(0) as usize;
        }
    }

    pub fn submit_lesson_quiz(&mut self) -> Result<(), BasicError> {
        let Some(lesson) = &self.code_lab.lesson else {
            return Ok(());
        };
        let lesson_id = lesson.view.detail.lesson.id;
        let answers = lesson.answers.clone();
        let result = CodeLabService::new(&mut self.database).submit_quiz(lesson_id, &answers)?;
        let passed = result.perfect();
        let correct = result.correct;
        let total = result.total;
        if let Some(lesson) = &mut self.code_lab.lesson {
            lesson.result = Some(result);
            lesson.status = format!(
                "Cuestionario: {correct}/{total} · {}",
                if passed {
                    "lección completada"
                } else {
                    "repasa las respuestas falladas"
                }
            );
        }
        self.refresh_code_lab()?;
        self.status = if passed {
            "Lección completada.".to_string()
        } else {
            "Cuestionario registrado; la lección necesita repaso.".to_string()
        };
        Ok(())
    }

    pub fn start_new_note_prompt(&mut self) {
        self.prompt = Some(Prompt {
            kind: PromptKind::NewNote,
            title: "Nueva nota (ruta relativa, Enter para crear)",
            input: String::new(),
        });
    }

    /// Crea una nota enlazada con un elemento de Code Lab, o abre la que ya
    /// existe. Nunca sobrescribe archivos del vault.
    pub fn start_code_lab_note_prompt(
        &mut self,
        item_type: NoteItemType,
        item_id: i64,
    ) -> Result<(), BasicError> {
        if let Some(link) = CodeLabService::new(&mut self.database).note_link(item_type, item_id)? {
            let indexed = catalog::document_by_path(&self.database, &link.note_path)?
                .map(|document| document.id);
            if let Some(id) = indexed {
                self.open_document(id)?;
            } else {
                self.status = format!("Nota enlazada, aún sin indexar: {}", link.note_path);
            }
            return Ok(());
        }

        let draft = self.code_lab_note_draft(item_type, item_id)?;
        self.prompt = Some(Prompt {
            kind: PromptKind::CodeLabNote(draft.clone()),
            title: "Nota de Code Lab (ruta relativa, Enter para crear)",
            input: draft.default_path,
        });
        Ok(())
    }

    /// Ruta de la nota enlazada, para mostrarla en la cabecera.
    pub fn linked_note_path(
        &self,
        item_type: NoteItemType,
        item_id: i64,
    ) -> Result<Option<String>, BasicError> {
        Ok(
            crate::code_lab::infra::note_link(
                &self.database,
                DEFAULT_LANGUAGE,
                item_type,
                item_id,
            )?
            .map(|link| link.note_path),
        )
    }

    fn code_lab_note_draft(
        &mut self,
        item_type: NoteItemType,
        item_id: i64,
    ) -> Result<CodeLabNoteDraft, BasicError> {
        let service = CodeLabService::new(&mut self.database);
        let template = note_template(DEFAULT_LANGUAGE);
        let marker = NoteBacklink {
            language: DEFAULT_LANGUAGE.to_string(),
            item_type,
            item_id,
        }
        .marker();
        let (title, body, default_path) = match item_type {
            NoteItemType::Lesson => {
                let view = service
                    .lesson_view(item_id)?
                    .ok_or_else(|| note_target_error("lección", item_id))?;
                let lesson = &view.detail.lesson;
                let body = format!(
                    "{marker}\n\n> Lección de {} · {}\n\n{}\n\n## {}\n\n{}\n\n## Contenido de la lección\n\n{}\n",
                    template.language,
                    lesson.display(),
                    lesson.summary,
                    template.concepts_heading,
                    bullet_list(&lesson.concepts),
                    lesson.content_md
                );
                (
                    lesson.title.clone(),
                    body,
                    format!("code-lab/{}/{}.md", DEFAULT_LANGUAGE, lesson.slug),
                )
            }
            NoteItemType::Exercise => {
                let exercise = service
                    .exercise_detail(item_id)?
                    .ok_or_else(|| note_target_error("ejercicio", item_id))?;
                let body = format!(
                    "{marker}\n\n> Ejercicio de {} · nivel {}\n\n{}\n\n## {}\n\n{}\n\n## Pistas\n\n{}\n\n## Código inicial\n\n```{}\n{}\n```\n\n## {}\n\nEscribe aquí tus pruebas y resultados.\n",
                    template.language,
                    exercise.level,
                    exercise.description_md,
                    template.concepts_heading,
                    bullet_list(&exercise.concepts),
                    bullet_list(&exercise.hints),
                    template.code_fence,
                    exercise.starter_code,
                    template.practice_heading
                );
                (
                    exercise.title.clone(),
                    body,
                    format!(
                        "code-lab/{}/ejercicio-{}.md",
                        DEFAULT_LANGUAGE, exercise.slug
                    ),
                )
            }
            NoteItemType::Concept => {
                let concept = service
                    .concept(item_id)?
                    .ok_or_else(|| note_target_error("concepto", item_id))?;
                let body = format!(
                    "{marker}\n\n> Concepto de {} · nivel {} · repasado {} vez/veces\n\nEste concepto aparece en lecciones y ejercicios. Añade aquí tus propias notas y ejemplos.\n\n## {}\n\n```{}\n// Tu ejemplo aquí\n```\n",
                    concept.language,
                    concept.level,
                    concept.lapses,
                    template.practice_heading,
                    template.code_fence
                );
                (
                    concept.concept.clone(),
                    body,
                    format!("code-lab/{}/concepto-{}.md", concept.language, concept.id),
                )
            }
        };
        Ok(CodeLabNoteDraft {
            item_type,
            item_id,
            title,
            body,
            default_path,
        })
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
            PromptKind::CodeLabNote(draft) => {
                let relative = notes::normalize_note_path(&prompt.input)?;
                let body = notes::render_note_body(&self.config, &draft.title, &draft.body);
                let created = notes::create_note_with_body(&self.config, &relative, &body)?;
                let report = index::index_vault(&mut self.database, &self.config)?;
                let document_id = catalog::document_by_path(&self.database, &created.relative)?
                    .map(|document| document.id);
                let link = NoteLink {
                    language: DEFAULT_LANGUAGE.to_string(),
                    item_type: draft.item_type,
                    item_id: draft.item_id,
                    note_path: created.relative.clone(),
                    document_id,
                };
                CodeLabService::new(&mut self.database).save_note_link(&link)?;
                self.status = format!(
                    "Nota de Code Lab creada: {} · {}",
                    created.relative,
                    report.summary()
                );
                if let Some(id) = document_id {
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

fn note_target_error(kind: &str, id: i64) -> BasicError {
    BasicError::InvalidConfig {
        path: "code-lab/notes".into(),
        message: format!("{kind} {id} no encontrado"),
    }
}

fn bullet_list(items: &[String]) -> String {
    if items.is_empty() {
        return "- (ninguno)".to_string();
    }
    items
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n")
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

    #[test]
    fn code_lab_lists_easy_exercises_and_opens_without_an_attempt() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.open_code_lab_screen().unwrap();
        assert_eq!(app.code_lab.exercises.len(), 3);
        assert!(
            app.code_lab
                .exercises
                .iter()
                .all(|exercise| exercise.difficulty == Difficulty::Easy)
        );

        app.open_selected_exercise().unwrap();
        assert_eq!(app.screen, Screen::Exercise);
        let workspace = app.code_lab.workspace.as_ref().unwrap();
        assert_eq!(workspace.progress.attempts, 0);
        assert!(workspace.code.contains("TODO"));
        assert!(workspace.result.is_none());
    }

    #[test]
    fn code_lab_filters_all_difficulties() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.open_code_lab_screen().unwrap();
        assert_eq!(app.code_lab.exercises.len(), 3);

        app.set_difficulty(Difficulty::Medium).unwrap();
        assert_eq!(app.code_lab.exercises.len(), 2);
        assert!(
            app.code_lab
                .exercises
                .iter()
                .all(|exercise| exercise.difficulty == Difficulty::Medium)
        );

        app.set_difficulty(Difficulty::Hard).unwrap();
        assert_eq!(app.code_lab.exercises.len(), 1);
        assert_eq!(app.code_lab.exercises[0].slug, "frecuencias-palabras");
        assert_eq!(app.code_lab.progress.len(), 6);
    }

    #[test]
    #[ignore = "requiere el SDK de .NET instalado"]
    fn real_sdk_solves_the_first_easy_exercise() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.open_code_lab_screen().unwrap();
        app.open_selected_exercise().unwrap();
        let id = app.code_lab.workspace.as_ref().unwrap().exercise.id;
        let solution = CodeLabService::new(&mut app.database)
            .solution(id)
            .unwrap()
            .unwrap();
        app.save_exercise_code(&solution).unwrap();
        app.request_exercise_run(&solution);
        app.run_pending().unwrap();

        let workspace = app.code_lab.workspace.as_ref().unwrap();
        assert!(
            workspace.result.as_ref().unwrap().passed(),
            "{:?}",
            workspace.status
        );
        assert_eq!(workspace.progress.best_tests_passed, 4);
    }

    #[test]
    #[ignore = "requiere el SDK de .NET instalado"]
    fn real_sdk_solves_every_catalogued_exercise() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.open_code_lab_screen().unwrap();

        for difficulty in Difficulty::ALL {
            app.set_difficulty(difficulty).unwrap();
            let ids: Vec<i64> = app
                .code_lab
                .exercises
                .iter()
                .map(|exercise| exercise.id)
                .collect();
            for id in ids {
                app.open_exercise(id).unwrap();
                let slug = app
                    .code_lab
                    .workspace
                    .as_ref()
                    .unwrap()
                    .exercise
                    .slug
                    .clone();
                let solution = CodeLabService::new(&mut app.database)
                    .solution(id)
                    .unwrap()
                    .unwrap();
                app.save_exercise_code(&solution).unwrap();
                app.request_exercise_run(&solution);
                app.run_pending().unwrap();
                let workspace = app.code_lab.workspace.as_ref().unwrap();
                assert!(
                    workspace.result.as_ref().unwrap().passed(),
                    "{slug}: {:?}",
                    workspace.result.as_ref().unwrap().cases
                );
            }
        }
        assert_eq!(app.code_lab.progress.len(), 6);
    }

    #[test]
    fn applying_an_outcome_records_attempt_and_draft() {
        use crate::code_lab::domain::AttemptStatus;

        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.open_code_lab_screen().unwrap();
        let id = app.code_lab.exercises[0].id;
        app.open_exercise(id).unwrap();

        let outcome = RunOutcome {
            status: AttemptStatus::Passed,
            sdk_version: Some("10.0.400".to_string()),
            compile_ok: Some(true),
            tests_passed: 4,
            tests_total: 4,
            duration_ms: 500,
            executions: Vec::new(),
            cases: Vec::new(),
        };
        app.apply_outcome(id, "solución propia", outcome).unwrap();
        let workspace = app.code_lab.workspace.as_ref().unwrap();
        assert!(workspace.result.as_ref().unwrap().passed());
        assert_eq!(workspace.progress.attempts, 1);
        assert_eq!(workspace.progress.best_tests_passed, 4);
        assert_eq!(workspace.code, "solución propia");
        assert_eq!(
            CodeLabService::new(&mut app.database)
                .draft(id)
                .unwrap()
                .as_deref(),
            Some("solución propia")
        );
    }

    #[test]
    fn lesson_quiz_completes_only_when_every_answer_is_right() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        let lesson_id = {
            let mut service = CodeLabService::new(&mut app.database);
            service.seed().unwrap();
            service.lessons().unwrap()[0].id
        };
        app.open_lesson(lesson_id).unwrap();
        assert_eq!(app.screen, Screen::Lesson);
        let progress = CodeLabService::new(&mut app.database)
            .lesson_view(lesson_id)
            .unwrap()
            .unwrap()
            .progress;
        assert_ne!(
            progress.status,
            crate::code_lab::domain::LessonStatus::Completed
        );

        let correct: Vec<usize> = app
            .code_lab
            .lesson
            .as_ref()
            .unwrap()
            .view
            .detail
            .questions
            .iter()
            .map(|question| question.correct_index)
            .collect();
        for (index, answer) in correct.iter().enumerate() {
            app.code_lab.lesson.as_mut().unwrap().question = index;
            app.select_lesson_answer(*answer);
        }
        app.submit_lesson_quiz().unwrap();
        assert!(
            app.code_lab
                .lesson
                .as_ref()
                .unwrap()
                .result
                .as_ref()
                .unwrap()
                .perfect()
        );
        let progress = CodeLabService::new(&mut app.database)
            .lesson_view(lesson_id)
            .unwrap()
            .unwrap()
            .progress;
        assert_eq!(
            progress.status,
            crate::code_lab::domain::LessonStatus::Completed
        );
        assert!(progress.completed_at.is_some());
    }

    #[test]
    fn learning_mode_shows_pending_items_and_opens_them() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.refresh_code_lab().unwrap();
        let exercise_id = app.code_lab.exercises[0].id;
        app.apply_outcome(
            exercise_id,
            "código",
            RunOutcome {
                status: crate::code_lab::domain::AttemptStatus::Failed,
                sdk_version: None,
                compile_ok: Some(true),
                tests_passed: 1,
                tests_total: 4,
                duration_ms: 100,
                executions: Vec::new(),
                cases: Vec::new(),
            },
        )
        .unwrap();
        app.set_learn_mode(LearnMode::CodeLab).unwrap();
        assert!(app.code_lab.items.iter().any(|item| item.id == exercise_id));
        app.selected = app
            .code_lab
            .items
            .iter()
            .position(|item| item.id == exercise_id)
            .unwrap();
        app.open_learning_item().unwrap();
        assert_eq!(app.screen, Screen::Exercise);
    }

    #[test]
    fn code_lab_note_prompt_creates_a_linked_markdown_note() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        let lesson_id = {
            let mut service = CodeLabService::new(&mut app.database);
            service.seed().unwrap();
            service.lessons().unwrap()[0].id
        };

        app.start_code_lab_note_prompt(NoteItemType::Lesson, lesson_id)
            .unwrap();
        assert!(app.prompt.is_some());
        app.prompt.as_mut().unwrap().input = "code-lab/mi-leccion.md".to_string();
        app.prompt_submit().unwrap();

        let path = app.config.vault.join("code-lab/mi-leccion.md");
        let body = fs::read_to_string(&path).unwrap();
        assert!(body.contains("# "));
        assert!(body.contains("Contenido de la lección"));
        assert_eq!(app.screen, Screen::Reader);
        assert_eq!(
            app.linked_note_path(NoteItemType::Lesson, lesson_id)
                .unwrap()
                .as_deref(),
            Some("code-lab/mi-leccion.md")
        );

        // Una segunda petición abre la nota existente en vez de crear otra.
        app.start_code_lab_note_prompt(NoteItemType::Lesson, lesson_id)
            .unwrap();
        assert!(app.prompt.is_none());
        assert_eq!(app.screen, Screen::Reader);
    }

    #[test]
    fn reader_can_return_to_the_note_source() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        let lesson_id = {
            let mut service = CodeLabService::new(&mut app.database);
            service.seed().unwrap();
            service.lessons().unwrap()[0].id
        };
        app.start_code_lab_note_prompt(NoteItemType::Lesson, lesson_id)
            .unwrap();
        app.prompt_submit().unwrap();

        let source = app.reader.as_ref().unwrap().source.clone().unwrap();
        assert_eq!(source.item_type, NoteItemType::Lesson);
        assert_eq!(source.item_id, lesson_id);
        app.open_note_source().unwrap();
        assert_eq!(app.screen, Screen::Lesson);
        assert_eq!(
            app.code_lab.lesson.as_ref().unwrap().view.detail.lesson.id,
            lesson_id
        );
    }

    #[test]
    fn rating_a_concept_advances_the_review_queue() {
        let (_temp, config) = fixture();
        let mut app = App::new(config, Database::open_in_memory().unwrap()).unwrap();
        app.open_concept_review_screen().unwrap();
        let total = app.code_lab.concepts.len();
        assert!(total > 0);
        let due_before = app.code_lab.stats.concepts_due;
        let first = app.code_lab.concepts[0].concept.clone();
        app.rate_current_concept(ReviewRating::Good).unwrap();
        assert_eq!(app.code_lab.concepts.len(), total - 1);
        assert!(
            !app.code_lab
                .concepts
                .iter()
                .any(|item| item.concept == first)
        );
        assert_eq!(app.code_lab.stats.concepts_due, due_before - 1);
    }
}
