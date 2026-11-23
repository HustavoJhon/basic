//! Modelos de Code Lab, sin SQLite, Ratatui ni procesos externos.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Lenguaje inicial del catálogo. El código es estable y se usa como clave.
pub const DEFAULT_LANGUAGE: &str = "csharp";

/// Dificultad de un ejercicio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub const ALL: [Self; 3] = [Self::Easy, Self::Medium, Self::Hard];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "fácil",
            Self::Medium => "medio",
            Self::Hard => "difícil",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "easy" => Some(Self::Easy),
            "medium" => Some(Self::Medium),
            "hard" => Some(Self::Hard),
            _ => None,
        }
    }

    /// Nivel orientativo para ordenar el catálogo.
    pub fn level(self) -> i32 {
        match self {
            Self::Easy => 1,
            Self::Medium => 2,
            Self::Hard => 3,
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

/// Progreso de una lección.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LessonStatus {
    NotStarted,
    InProgress,
    Completed,
    NeedsReview,
}

impl LessonStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::NeedsReview => "needs_review",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::NotStarted => "sin empezar",
            Self::InProgress => "en curso",
            Self::Completed => "completada",
            Self::NeedsReview => "repasar",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value {
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "needs_review" => Self::NeedsReview,
            _ => Self::NotStarted,
        }
    }

    /// Permitir avanzar solo cuando la lección está completada.
    pub fn unlocks_next(self) -> bool {
        self == Self::Completed
    }
}

/// Progreso de un ejercicio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStatus {
    NotStarted,
    Attempted,
    Solved,
    NeedsReview,
}

impl ExerciseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Attempted => "attempted",
            Self::Solved => "solved",
            Self::NeedsReview => "needs_review",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::NotStarted => "sin intentar",
            Self::Attempted => "intentado",
            Self::Solved => "resuelto",
            Self::NeedsReview => "repasar",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value {
            "attempted" => Self::Attempted,
            "solved" => Self::Solved,
            "needs_review" => Self::NeedsReview,
            _ => Self::NotStarted,
        }
    }

    pub fn is_solved(self) -> bool {
        self == Self::Solved
    }
}

/// Estado de un intento de resolución.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttemptStatus {
    Running,
    Passed,
    Failed,
    Cancelled,
}

impl AttemptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Running => "en curso",
            Self::Passed => "superado",
            Self::Failed => "fallido",
            Self::Cancelled => "cancelado",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value {
            "passed" => Self::Passed,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => Self::Running,
        }
    }
}

/// Fase registrada de una ejecución.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionPhase {
    Detect,
    Restore,
    Build,
    Run,
    Test,
}

impl ExecutionPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Detect => "detect",
            Self::Restore => "restore",
            Self::Build => "build",
            Self::Run => "run",
            Self::Test => "test",
        }
    }
}

/// Tipo de elemento favorite o buscable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Lesson,
    Exercise,
}

impl ItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lesson => "lesson",
            Self::Exercise => "exercise",
        }
    }
}

/// Tipo de elemento que puede tener una nota Markdown enlazada.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteItemType {
    Lesson,
    Exercise,
    Concept,
}

impl NoteItemType {
    pub const ALL: [Self; 3] = [Self::Lesson, Self::Exercise, Self::Concept];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lesson => "lesson",
            Self::Exercise => "exercise",
            Self::Concept => "concept",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Lesson => "lección",
            Self::Exercise => "ejercicio",
            Self::Concept => "concepto",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "lesson" => Some(Self::Lesson),
            "exercise" => Some(Self::Exercise),
            "concept" => Some(Self::Concept),
            _ => None,
        }
    }
}

/// Enlace persistido entre un elemento de Code Lab y una nota del vault.
#[derive(Debug, Clone, Serialize)]
pub struct NoteLink {
    pub language: String,
    pub item_type: NoteItemType,
    pub item_id: i64,
    pub note_path: String,
    pub document_id: Option<i64>,
}

/// Plantilla declarativa para las notas de un lenguaje.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteTemplate {
    pub language: &'static str,
    pub code_fence: &'static str,
    pub concepts_heading: &'static str,
    pub practice_heading: &'static str,
}

pub const CSHARP_NOTE_TEMPLATE: NoteTemplate = NoteTemplate {
    language: DEFAULT_LANGUAGE,
    code_fence: "csharp",
    concepts_heading: "Conceptos de C#",
    practice_heading: "Practica en C#",
};

pub const GENERIC_NOTE_TEMPLATE: NoteTemplate = NoteTemplate {
    language: "generic",
    code_fence: "text",
    concepts_heading: "Conceptos",
    practice_heading: "Practica",
};

/// Devuelve la plantilla del lenguaje, con una genérica como reserva.
pub fn note_template(language: &str) -> NoteTemplate {
    if language == DEFAULT_LANGUAGE {
        CSHARP_NOTE_TEMPLATE
    } else {
        GENERIC_NOTE_TEMPLATE
    }
}

/// Marcador invisible en la nota que permite volver a su elemento de origen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteBacklink {
    pub language: String,
    pub item_type: NoteItemType,
    pub item_id: i64,
}

impl NoteBacklink {
    pub fn marker(&self) -> String {
        format!(
            "<!-- basic-code-lab language=\"{}\" type=\"{}\" id=\"{}\" -->",
            self.language,
            self.item_type.as_str(),
            self.item_id
        )
    }

    /// Lee el primer marcador válido del cuerpo de una nota.
    pub fn parse(body: &str) -> Option<Self> {
        let line = body.lines().find(|line| line.contains("basic-code-lab"))?;
        let language = note_attribute(line, "language")?;
        let raw_type = note_attribute(line, "type")?;
        let item_type = NoteItemType::parse(&raw_type)?;
        let item_id = note_attribute(line, "id")?.parse().ok()?;
        Some(Self {
            language,
            item_type,
            item_id,
        })
    }
}

fn note_attribute(line: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=\"");
    let start = line.find(&needle)? + needle.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Ejemplo visible en el enunciado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub input: String,
    pub output: String,
    #[serde(default)]
    pub explanation: Option<String>,
}

/// Caso de prueba. `visible` distingue ejemplos de evaluación oculta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub input: String,
    pub expected_output: String,
    #[serde(default)]
    pub visible: bool,
}

/// Especificación de pruebas, serializada en `test_spec_json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSpec {
    #[serde(default = "default_runner")]
    pub runner: String,
    #[serde(default)]
    pub cases: Vec<TestCase>,
    #[serde(default)]
    pub entry_point: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_runner() -> String {
    "csharp_stdin".to_string()
}

fn default_timeout() -> u64 {
    10
}

impl Default for TestSpec {
    fn default() -> Self {
        Self {
            runner: default_runner(),
            cases: Vec::new(),
            entry_point: None,
            timeout_secs: default_timeout(),
        }
    }
}

/// Lenguaje con SDK detectado.
#[derive(Debug, Clone)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub sdk_command: Option<String>,
    pub enabled: bool,
    pub detected_version: Option<String>,
    pub detected_at: Option<i64>,
}

impl Language {
    pub fn available(&self) -> bool {
        self.enabled && self.detected_version.is_some()
    }

    pub fn availability_label(&self) -> &'static str {
        if self.available() {
            "disponible"
        } else {
            "SDK no detectado"
        }
    }
}

/// Ruta de aprendizaje de un lenguaje.
#[derive(Debug, Clone)]
pub struct LearningPath {
    pub id: i64,
    pub language: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub level: i32,
}

/// Unidad teórica con contenido Markdown.
#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: i64,
    pub path_id: i64,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content_md: String,
    pub concepts: Vec<String>,
    pub order_index: i32,
    pub estimated_minutes: i32,
}

impl Lesson {
    pub fn display(&self) -> String {
        format!("{:02}. {}", self.order_index, self.title)
    }
}

/// Pregunta de opción múltiple asociada a una lección.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: i64,
    pub lesson_id: i64,
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub explanation: String,
    pub order_index: i32,
}

impl QuizQuestion {
    pub fn is_correct(&self, answer: usize) -> bool {
        answer == self.correct_index
    }
}

/// Resultado de una respuesta concreta del cuestionario.
#[derive(Debug, Clone)]
pub struct AnswerResult {
    pub question_id: i64,
    pub answer: Option<usize>,
    pub correct: bool,
    pub correct_index: usize,
    pub explanation: String,
}

/// Resultado agregado del cuestionario de una lección.
#[derive(Debug, Clone)]
pub struct QuizResult {
    pub correct: i32,
    pub total: i32,
    pub status: LessonStatus,
    pub answers: Vec<AnswerResult>,
}

impl QuizResult {
    pub fn perfect(&self) -> bool {
        self.total > 0 && self.correct == self.total
    }
}

/// Ejercicio sin solución: la UI nunca recibe el código de referencia.
#[derive(Debug, Clone)]
pub struct Exercise {
    pub id: i64,
    pub language: String,
    pub path_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub topic: String,
    pub level: i32,
    pub concepts: Vec<String>,
    pub description_md: String,
    pub examples: Vec<Example>,
    pub constraints: Vec<String>,
    pub hints: Vec<String>,
    pub starter_code: String,
    pub test_spec: TestSpec,
}

impl Exercise {
    pub fn visible_cases(&self) -> impl Iterator<Item = &TestCase> {
        self.test_spec.cases.iter().filter(|case| case.visible)
    }

    pub fn hidden_case_count(&self) -> usize {
        self.test_spec
            .cases
            .iter()
            .filter(|case| !case.visible)
            .count()
    }

    pub fn concepts_label(&self) -> String {
        self.concepts.join(", ")
    }

    pub fn display(&self) -> String {
        format!("{} · {}", self.title, self.difficulty.label())
    }
}

/// Solución de referencia, disponible solo bajo petición explícita.
#[derive(Debug, Clone)]
pub struct ExerciseSolution {
    pub exercise_id: i64,
    pub code: String,
}

/// Progreso persistido de una lección.
#[derive(Debug, Clone)]
pub struct LessonProgress {
    pub lesson_id: i64,
    pub status: LessonStatus,
    pub last_opened_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub quiz_correct: i32,
    pub quiz_total: i32,
    pub seconds_spent: i64,
    pub review_due_at: Option<i64>,
    pub updated_at: i64,
}

impl Default for LessonProgress {
    fn default() -> Self {
        Self {
            lesson_id: 0,
            status: LessonStatus::NotStarted,
            last_opened_at: None,
            completed_at: None,
            quiz_correct: 0,
            quiz_total: 0,
            seconds_spent: 0,
            review_due_at: None,
            updated_at: 0,
        }
    }
}

/// Progreso persistido de un ejercicio.
#[derive(Debug, Clone)]
pub struct ExerciseProgress {
    pub exercise_id: i64,
    pub status: ExerciseStatus,
    pub attempts: i32,
    pub solved_at: Option<i64>,
    pub last_opened_at: Option<i64>,
    pub best_tests_passed: i32,
    pub tests_total: i32,
    pub best_score: f64,
    pub seconds_spent: i64,
    pub last_attempt_id: Option<i64>,
    pub updated_at: i64,
}

impl Default for ExerciseProgress {
    fn default() -> Self {
        Self {
            exercise_id: 0,
            status: ExerciseStatus::NotStarted,
            attempts: 0,
            solved_at: None,
            last_opened_at: None,
            best_tests_passed: 0,
            tests_total: 0,
            best_score: 0.0,
            seconds_spent: 0,
            last_attempt_id: None,
            updated_at: 0,
        }
    }
}

impl ExerciseProgress {
    pub fn success_rate(&self) -> f64 {
        if self.tests_total <= 0 {
            return 0.0;
        }
        self.best_tests_passed as f64 / self.tests_total as f64
    }
}

/// Intento de resolución.
#[derive(Debug, Clone)]
pub struct Attempt {
    pub id: i64,
    pub exercise_id: i64,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub duration_seconds: i64,
    pub status: AttemptStatus,
    pub tests_passed: i32,
    pub tests_total: i32,
    pub compile_ok: Option<bool>,
    pub language: String,
}

impl Attempt {
    pub fn minutes(&self) -> i64 {
        self.duration_seconds / 60
    }
}

/// Datos de un intento listos para persistir, sin depender de runners ni UI.
#[derive(Debug, Clone)]
pub struct AttemptSummary {
    pub status: AttemptStatus,
    pub duration_ms: u64,
    pub tests_passed: i32,
    pub tests_total: i32,
    pub compile_ok: Option<bool>,
    pub executions: Vec<ExecutionResult>,
}

impl AttemptSummary {
    pub fn duration_seconds(&self) -> i64 {
        i64::try_from(self.duration_ms / 1000).unwrap_or(i64::MAX)
    }

    pub fn score(&self) -> f64 {
        if self.tests_total <= 0 {
            return 0.0;
        }
        f64::from(self.tests_passed) / f64::from(self.tests_total)
    }
}

/// Resultado de una fase de ejecución.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub phase: ExecutionPhase,
    pub command: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

impl ExecutionResult {
    pub fn succeeded(&self) -> bool {
        self.exit_code == Some(0)
    }

    pub fn output(&self) -> &str {
        if self.stderr.trim().is_empty() {
            &self.stdout
        } else {
            &self.stderr
        }
    }
}

/// Concepto pendiente de repaso.
#[derive(Debug, Clone)]
pub struct ConceptReview {
    pub id: i64,
    pub language: String,
    pub concept: String,
    pub level: i32,
    pub interval_days: i32,
    pub ease_factor: f64,
    pub due_at: Option<i64>,
    pub lapses: i32,
    pub last_reviewed_at: Option<i64>,
}

impl ConceptReview {
    /// Los conceptos nuevos entran en la cola desde el primer día.
    pub fn is_due(&self, now: i64) -> bool {
        self.due_at.is_none_or(|due| due <= now)
    }
}

/// Elemento de Code Lab que necesita atención en Learning Mode.
#[derive(Debug, Clone)]
pub struct CodeLabItem {
    pub id: i64,
    pub kind: ItemType,
    pub title: String,
    pub subtitle: String,
    pub status: String,
    pub due_at: Option<i64>,
}

/// Resumen de Code Lab para dashboard y estadísticas.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CodeLabStats {
    pub languages: i32,
    pub paths: i32,
    pub lessons: i32,
    pub lessons_completed: i32,
    pub exercises: i32,
    pub exercises_solved: i32,
    pub attempts: i32,
    pub attempts_passed: i32,
    pub seconds_spent: i64,
    pub concepts_due: i32,
}

impl CodeLabStats {
    pub fn solved_rate(&self) -> f64 {
        if self.exercises <= 0 {
            return 0.0;
        }
        self.exercises_solved as f64 / self.exercises as f64
    }
}

/// Objetivo diario de una actividad de Code Lab.
#[derive(Debug, Clone, Serialize)]
pub struct DailyGoal {
    pub label: &'static str,
    pub target: i32,
    pub done: i32,
}

impl DailyGoal {
    pub fn complete(&self) -> bool {
        self.done >= self.target
    }
}

/// Avance de los objetivos del día; `day` usa el formato `YYYY-MM-DD`.
#[derive(Debug, Clone, Serialize)]
pub struct DailyProgress {
    pub day: String,
    pub lessons: DailyGoal,
    pub exercises: DailyGoal,
}

impl Default for DailyProgress {
    fn default() -> Self {
        Self {
            day: String::new(),
            lessons: DailyGoal {
                label: "lecciones",
                target: 0,
                done: 0,
            },
            exercises: DailyGoal {
                label: "ejercicios",
                target: 0,
                done: 0,
            },
        }
    }
}

/// Informe exportable con el estado agregado del laboratorio.
#[derive(Debug, Clone, Serialize)]
pub struct ProgressReport {
    pub generated_at: i64,
    pub stats: CodeLabStats,
    pub daily: DailyProgress,
    pub note_links: Vec<NoteLink>,
}

/// Abrir una lección nunca la marca como completada.
pub fn lesson_status_after_open(current: LessonStatus) -> LessonStatus {
    match current {
        LessonStatus::NotStarted | LessonStatus::NeedsReview => LessonStatus::InProgress,
        other => other,
    }
}

/// Solo un cuestionario correcto completa la lección.
pub fn lesson_status_after_quiz(_current: LessonStatus, correct: i32, total: i32) -> LessonStatus {
    if total > 0 && correct >= total {
        LessonStatus::Completed
    } else {
        LessonStatus::NeedsReview
    }
}

/// Resolver todos los casos marca el ejercicio como resuelto; un fallo posterior
/// lo devuelve a repaso sin borrar el hecho de que llegó a resolverse.
pub fn exercise_status_after_attempt(previous: ExerciseStatus, passed: bool) -> ExerciseStatus {
    if passed {
        ExerciseStatus::Solved
    } else if previous == ExerciseStatus::Solved {
        ExerciseStatus::NeedsReview
    } else {
        ExerciseStatus::Attempted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_round_trips() {
        for difficulty in Difficulty::ALL {
            assert_eq!(Difficulty::parse(difficulty.as_str()), Some(difficulty));
        }
        assert_eq!(Difficulty::parse("trivial"), None);
        assert_eq!(Difficulty::Medium.level(), 2);
    }

    #[test]
    fn opening_a_lesson_does_not_complete_it() {
        assert_eq!(
            lesson_status_after_open(LessonStatus::NotStarted),
            LessonStatus::InProgress
        );
        assert_eq!(
            lesson_status_after_open(LessonStatus::InProgress),
            LessonStatus::InProgress
        );
        assert_eq!(
            lesson_status_after_open(LessonStatus::Completed),
            LessonStatus::Completed
        );
        assert!(!LessonStatus::InProgress.unlocks_next());
        assert!(LessonStatus::Completed.unlocks_next());
    }

    #[test]
    fn quiz_requires_every_answer() {
        assert_eq!(
            lesson_status_after_quiz(LessonStatus::InProgress, 2, 3),
            LessonStatus::NeedsReview
        );
        assert_eq!(
            lesson_status_after_quiz(LessonStatus::InProgress, 3, 3),
            LessonStatus::Completed
        );
        assert_eq!(
            lesson_status_after_quiz(LessonStatus::Completed, 0, 0),
            LessonStatus::NeedsReview
        );
    }

    #[test]
    fn exercise_failure_after_success_needs_review() {
        assert_eq!(
            exercise_status_after_attempt(ExerciseStatus::NotStarted, true),
            ExerciseStatus::Solved
        );
        assert_eq!(
            exercise_status_after_attempt(ExerciseStatus::Solved, false),
            ExerciseStatus::NeedsReview
        );
        assert_eq!(
            exercise_status_after_attempt(ExerciseStatus::NeedsReview, false),
            ExerciseStatus::Attempted
        );
    }

    #[test]
    fn hidden_cases_are_not_visible() {
        let exercise = Exercise {
            id: 1,
            language: DEFAULT_LANGUAGE.to_string(),
            path_id: None,
            lesson_id: None,
            slug: "suma".to_string(),
            title: "Suma".to_string(),
            difficulty: Difficulty::Easy,
            topic: "operadores".to_string(),
            level: 1,
            concepts: vec!["variables".to_string()],
            description_md: String::new(),
            examples: Vec::new(),
            constraints: Vec::new(),
            hints: Vec::new(),
            starter_code: String::new(),
            test_spec: TestSpec {
                cases: vec![
                    TestCase {
                        name: "ejemplo".to_string(),
                        input: "1 2".to_string(),
                        expected_output: "3".to_string(),
                        visible: true,
                    },
                    TestCase {
                        name: "oculto".to_string(),
                        input: "5 7".to_string(),
                        expected_output: "12".to_string(),
                        visible: false,
                    },
                ],
                ..TestSpec::default()
            },
        };
        assert_eq!(exercise.visible_cases().count(), 1);
        assert_eq!(exercise.hidden_case_count(), 1);
    }

    #[test]
    fn note_backlinks_survive_a_round_trip() {
        let backlink = NoteBacklink {
            language: "csharp".to_string(),
            item_type: NoteItemType::Exercise,
            item_id: 42,
        };
        let body = format!("# Prueba\n\n{}\n\nContenido\n", backlink.marker());
        assert_eq!(NoteBacklink::parse(&body), Some(backlink));
        assert_eq!(NoteBacklink::parse("# Sin marcador\n"), None);
        let malformed = "<!-- basic-code-lab language=\"csharp\" type=\"otro\" id=\"1\" -->";
        assert_eq!(NoteBacklink::parse(malformed), None);
    }

    #[test]
    fn note_templates_use_the_language_code_fence() {
        let csharp = note_template(DEFAULT_LANGUAGE);
        assert_eq!(csharp.code_fence, "csharp");
        assert_eq!(csharp.concepts_heading, "Conceptos de C#");
        assert_eq!(note_template("python").code_fence, "text");
    }

    #[test]
    fn stats_rate_handles_empty_catalog() {
        let stats = CodeLabStats::default();
        assert_eq!(stats.solved_rate(), 0.0);
        let stats = CodeLabStats {
            exercises: 4,
            exercises_solved: 3,
            ..CodeLabStats::default()
        };
        assert!((stats.solved_rate() - 0.75).abs() < f64::EPSILON);
    }
}
