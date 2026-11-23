//! Casos de uso de Code Lab. No conoce Ratatui ni procesos externos.

use crate::code_lab::domain::{Exercise, Lesson, LessonProgress, QuizResult};
use crate::code_lab::infra::{
    LessonDetail, SeedReport, lesson_detail, lesson_progress, list_exercises, list_lessons,
    open_lesson, record_lesson_time, seed_catalog, submit_quiz,
};
use crate::error::BasicError;
use crate::infra::sqlite::Database;

/// Vista combinada de contenido y progreso para una lección.
#[derive(Debug, Clone)]
pub struct LessonView {
    pub detail: LessonDetail,
    pub progress: LessonProgress,
}

impl LessonView {
    pub fn title(&self) -> &str {
        &self.detail.lesson.title
    }
}

/// Servicio usado por la UI y la CLI. Toda escritura pasa por aquí.
pub struct CodeLabService<'a> {
    database: &'a mut Database,
}

impl<'a> CodeLabService<'a> {
    pub fn new(database: &'a mut Database) -> Self {
        Self { database }
    }

    /// Puebla el catálogo. Llamar varias veces es seguro.
    pub fn seed(&mut self) -> Result<SeedReport, BasicError> {
        seed_catalog(self.database)
    }

    pub fn lessons(&self) -> Result<Vec<Lesson>, BasicError> {
        list_lessons(self.database, crate::code_lab::domain::DEFAULT_LANGUAGE)
    }

    pub fn exercises(&self) -> Result<Vec<Exercise>, BasicError> {
        list_exercises(self.database, crate::code_lab::domain::DEFAULT_LANGUAGE)
    }

    pub fn lesson_view(&self, lesson_id: i64) -> Result<Option<LessonView>, BasicError> {
        let Some(detail) = lesson_detail(self.database, lesson_id)? else {
            return Ok(None);
        };
        let progress = lesson_progress(self.database, lesson_id)?;
        Ok(Some(LessonView { detail, progress }))
    }

    /// Abrir una lección solo actualiza `last_opened_at` y, como mucho, la
    /// marca `in_progress`.
    pub fn open_lesson(&mut self, lesson_id: i64) -> Result<LessonView, BasicError> {
        let detail =
            lesson_detail(self.database, lesson_id)?.ok_or_else(|| BasicError::InvalidConfig {
                path: "code_lab/lessons".into(),
                message: format!("lección {lesson_id} no encontrada"),
            })?;
        let progress = open_lesson(self.database, lesson_id)?;
        Ok(LessonView { detail, progress })
    }

    pub fn submit_quiz(
        &mut self,
        lesson_id: i64,
        answers: &[Option<usize>],
    ) -> Result<QuizResult, BasicError> {
        submit_quiz(self.database, lesson_id, answers)
    }

    pub fn record_time(&mut self, lesson_id: i64, seconds: i64) -> Result<(), BasicError> {
        record_lesson_time(self.database, lesson_id, seconds)
    }
}

#[cfg(test)]
mod tests {
    use crate::code_lab::domain::LessonStatus;
    use crate::infra::sqlite::Database;

    use super::*;

    #[test]
    fn service_seeds_and_opens_without_completing() {
        let mut database = Database::open_in_memory().unwrap();
        let mut service = CodeLabService::new(&mut database);
        let report = service.seed().unwrap();
        assert_eq!(report.lessons, 5);

        let id = service.lessons().unwrap()[0].id;
        let view = service.open_lesson(id).unwrap();
        assert_eq!(view.progress.status, LessonStatus::InProgress);
        assert!(!view.detail.questions.is_empty());
    }
}
