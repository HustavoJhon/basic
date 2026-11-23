//! Modelos y reglas que no dependen de SQLite, Ratatui ni del sistema de archivos.

use std::fmt;

use serde::Serialize;

/// Documento Markdown tal como lo consume la aplicación.
///
/// No incluye el cuerpo completo: la vista de lectura lo carga de disco para
/// mantener Markdown como fuente de verdad.
#[derive(Debug, Clone, Serialize)]
pub struct Document {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub kind: String,
    pub size_bytes: i64,
    pub word_count: i64,
    pub reading_minutes: i64,
    pub excerpt: String,
    pub is_stub: bool,
    pub missing_at: Option<i64>,
    pub indexed_at: i64,
    pub status: LearningStatus,
    pub last_opened_at: Option<i64>,
    pub review_due_at: Option<i64>,
    pub favorite: bool,
}

impl Document {
    /// Ruta legible para breadcrumbs y resultados.
    pub fn location(&self) -> String {
        match &self.subcategory {
            Some(subcategory) if !subcategory.is_empty() => {
                format!("{}/{subcategory}", self.category)
            }
            _ => self.category.clone(),
        }
    }

    /// Texto corto para listas: título y, si aporta contexto, la ruta.
    pub fn display(&self) -> String {
        format!("{}  ·  {}", self.title, self.location())
    }
}

/// Estado de aprendizaje de un documento.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningStatus {
    Pending,
    InProgress,
    Learned,
    NeedsReview,
}

impl LearningStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Learned => "learned",
            Self::NeedsReview => "needs_review",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Pending => "pendiente",
            Self::InProgress => "en curso",
            Self::Learned => "aprendido",
            Self::NeedsReview => "repasar",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value {
            "in_progress" => Self::InProgress,
            "learned" => Self::Learned,
            "needs_review" => Self::NeedsReview,
            _ => Self::Pending,
        }
    }
}

/// Alcance de una búsqueda.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    All,
    Title,
    Content,
    Path,
    Category,
    Code,
}

impl SearchScope {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "all" => Some(Self::All),
            "title" => Some(Self::Title),
            "content" => Some(Self::Content),
            "path" => Some(Self::Path),
            "category" => Some(Self::Category),
            "code" => Some(Self::Code),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "todo",
            Self::Title => "título",
            Self::Content => "contenido",
            Self::Path => "ruta",
            Self::Category => "categoría",
            Self::Code => "código",
        }
    }
}

/// Modos de entrada de Learning Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearnMode {
    Continue,
    Explore,
    Pending,
    Review,
    Random,
    CodeLab,
}

impl LearnMode {
    pub const ALL: [Self; 6] = [
        Self::Continue,
        Self::Explore,
        Self::Pending,
        Self::Review,
        Self::Random,
        Self::CodeLab,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Continue => "continuar",
            Self::Explore => "explorar",
            Self::Pending => "pendientes",
            Self::Review => "repasar",
            Self::Random => "aleatorio",
            Self::CodeLab => "code lab",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Continue => Self::Explore,
            Self::Explore => Self::Pending,
            Self::Pending => Self::Review,
            Self::Review => Self::Random,
            Self::Random => Self::CodeLab,
            Self::CodeLab => Self::Continue,
        }
    }
}

/// Calificación de un repaso, de peor a mejor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewRating {
    Again,
    Hard,
    Good,
    Easy,
}

impl ReviewRating {
    pub fn from_key(key: char) -> Option<Self> {
        match key {
            '1' => Some(Self::Again),
            '2' => Some(Self::Hard),
            '3' => Some(Self::Good),
            '4' => Some(Self::Easy),
            _ => None,
        }
    }

    pub fn value(self) -> i64 {
        match self {
            Self::Again => 0,
            Self::Hard => 1,
            Self::Good => 2,
            Self::Easy => 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Again => "otra vez",
            Self::Hard => "difícil",
            Self::Good => "bien",
            Self::Easy => "fácil",
        }
    }
}

/// Resultado de aplicar el algoritmo de repaso.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReviewOutcome {
    pub interval_days: i64,
    pub ease_factor: f64,
    pub due_in_days: i64,
}

/// Programación tipo SM-2, deliberadamente simple y determinista.
pub fn schedule_review(
    rating: ReviewRating,
    previous_interval: i64,
    previous_ease: f64,
) -> ReviewOutcome {
    let previous_interval = previous_interval.max(0);
    let previous_ease = previous_ease.clamp(1.3, 3.0);

    let (interval_days, ease_factor) = match rating {
        ReviewRating::Again => (1, (previous_ease - 0.20).max(1.3)),
        ReviewRating::Hard => (
            ((previous_interval as f64 * 1.2).round() as i64).max(1),
            (previous_ease - 0.15).max(1.3),
        ),
        ReviewRating::Good => {
            let interval = match previous_interval {
                0 => 1,
                1 => 3,
                other => (other as f64 * previous_ease).round() as i64,
            };
            (interval.max(1), previous_ease)
        }
        ReviewRating::Easy => {
            let interval = match previous_interval {
                0 => 3,
                1 => 5,
                other => (other as f64 * previous_ease * 1.3).round() as i64,
            };
            (interval.max(1), (previous_ease + 0.15).min(3.0))
        }
    };

    ReviewOutcome {
        interval_days,
        ease_factor,
        due_in_days: interval_days,
    }
}

/// Resumen para el dashboard.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DashboardStats {
    pub total: i64,
    pub pending: i64,
    pub in_progress: i64,
    pub due: i64,
    pub stubs: i64,
    pub study_seconds_today: i64,
    pub pomodoros_today: i64,
    pub streak_days: i64,
    pub categories: Vec<(String, i64)>,
}

impl DashboardStats {
    pub fn study_minutes_today(&self) -> i64 {
        self.study_seconds_today / 60
    }
}

/// Un día del heatmap de actividad.
#[derive(Debug, Clone, Serialize)]
pub struct ActivityDay {
    pub day: String,
    pub study_seconds: i64,
}

/// Etiqueta de intensidad para el heatmap.
pub fn activity_level(seconds: i64) -> u8 {
    match seconds {
        ..=0 => 0,
        1..=899 => 1,
        900..=1799 => 2,
        1800..=3599 => 3,
        _ => 4,
    }
}

/// Formatea una duración en segundos como `1h 05m` o `12m`.
pub fn format_duration(seconds: i64) -> String {
    let seconds = seconds.max(0);
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    if hours > 0 {
        format!("{hours}h {minutes:02}m")
    } else if minutes > 0 {
        format!("{minutes}m")
    } else {
        format!("{seconds}s")
    }
}

impl fmt::Display for LearnMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_rating_grows_the_interval() {
        let first = schedule_review(ReviewRating::Good, 0, 2.5);
        assert_eq!(first.interval_days, 1);
        let second = schedule_review(ReviewRating::Good, first.interval_days, 2.5);
        assert_eq!(second.interval_days, 3);
        let third = schedule_review(ReviewRating::Good, second.interval_days, 2.5);
        assert_eq!(third.interval_days, 8);
    }

    #[test]
    fn again_resets_the_interval_and_lowers_ease() {
        let outcome = schedule_review(ReviewRating::Again, 20, 2.5);
        assert_eq!(outcome.interval_days, 1);
        assert!(outcome.ease_factor < 2.5);
    }

    #[test]
    fn ease_stays_within_bounds() {
        let mut ease = 2.5;
        for _ in 0..20 {
            ease = schedule_review(ReviewRating::Again, 1, ease).ease_factor;
        }
        assert!(ease >= 1.3);
    }

    #[test]
    fn activity_levels_cover_the_scale() {
        assert_eq!(activity_level(0), 0);
        assert_eq!(activity_level(60), 1);
        assert_eq!(activity_level(1200), 2);
        assert_eq!(activity_level(2400), 3);
        assert_eq!(activity_level(4000), 4);
    }
}
