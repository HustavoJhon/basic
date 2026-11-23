//! Casos de uso de Code Lab. No conoce Ratatui ni procesos externos.

use crate::code_lab::domain::{
    AttemptSummary, CodeLabItem, CodeLabStats, ConceptReview, DEFAULT_LANGUAGE, DailyProgress,
    Exercise, ExerciseProgress, Lesson, LessonProgress, NoteItemType, NoteLink, ProgressReport,
    QuizResult,
};
use crate::code_lab::infra::{
    LessonDetail, SeedReport, code_lab_stats, concept, daily_progress, due_concepts,
    exercise_detail, exercise_progress, learning_items, lesson_detail, lesson_progress,
    list_exercises, list_lessons, load_draft, note_link, note_links, open_exercise, open_lesson,
    rate_concept, record_attempt, record_lesson_time, reveal_solution, save_draft, save_note_link,
    seed_catalog, submit_quiz,
};
use crate::domain::{ReviewOutcome, ReviewRating};
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

    pub fn exercise_detail(&self, exercise_id: i64) -> Result<Option<Exercise>, BasicError> {
        exercise_detail(self.database, exercise_id)
    }

    pub fn exercise_progress(&self, exercise_id: i64) -> Result<ExerciseProgress, BasicError> {
        exercise_progress(self.database, exercise_id)
    }

    /// Abrir un ejercicio no cuenta como intento ni revela la solución.
    pub fn open_exercise(&mut self, exercise_id: i64) -> Result<ExerciseProgress, BasicError> {
        open_exercise(self.database, exercise_id)
    }

    pub fn draft(&self, exercise_id: i64) -> Result<Option<String>, BasicError> {
        load_draft(self.database, exercise_id)
    }

    pub fn save_draft(&mut self, exercise_id: i64, code: &str) -> Result<(), BasicError> {
        save_draft(self.database, exercise_id, code)
    }

    pub fn solution(&self, exercise_id: i64) -> Result<Option<String>, BasicError> {
        reveal_solution(self.database, exercise_id)
    }

    pub fn record_attempt(
        &mut self,
        exercise_id: i64,
        code: &str,
        summary: &AttemptSummary,
    ) -> Result<i64, BasicError> {
        record_attempt(self.database, exercise_id, code, summary)
    }

    /// Estadísticas del catálogo y del progreso de práctica.
    pub fn stats(&self) -> Result<CodeLabStats, BasicError> {
        code_lab_stats(self.database, crate::infra::catalog::unix_now())
    }

    /// Conceptos con repaso pendiente.
    pub fn due_concepts(&self) -> Result<Vec<ConceptReview>, BasicError> {
        due_concepts(self.database, crate::infra::catalog::unix_now())
    }

    pub fn concept(&self, concept_id: i64) -> Result<Option<ConceptReview>, BasicError> {
        concept(self.database, concept_id)
    }

    pub fn rate_concept(
        &mut self,
        concept_id: i64,
        rating: ReviewRating,
    ) -> Result<ReviewOutcome, BasicError> {
        rate_concept(self.database, concept_id, rating)
    }

    /// Elementos que Learning Mode debe mostrar para continuar.
    pub fn learning_items(&self) -> Result<Vec<CodeLabItem>, BasicError> {
        learning_items(self.database)
    }

    pub fn note_link(
        &self,
        item_type: NoteItemType,
        item_id: i64,
    ) -> Result<Option<NoteLink>, BasicError> {
        note_link(self.database, DEFAULT_LANGUAGE, item_type, item_id)
    }

    pub fn save_note_link(&mut self, link: &NoteLink) -> Result<(), BasicError> {
        save_note_link(self.database, link)
    }

    pub fn note_links(&self) -> Result<Vec<NoteLink>, BasicError> {
        note_links(self.database)
    }

    pub fn daily_progress(
        &self,
        lesson_target: i32,
        exercise_target: i32,
    ) -> Result<DailyProgress, BasicError> {
        daily_progress(self.database, lesson_target, exercise_target)
    }

    /// Informe completo para exportar sin acceder a servicios externos.
    pub fn report(
        &self,
        lesson_target: i32,
        exercise_target: i32,
    ) -> Result<ProgressReport, BasicError> {
        Ok(ProgressReport {
            generated_at: crate::infra::catalog::unix_now(),
            stats: self.stats()?,
            daily: self.daily_progress(lesson_target, exercise_target)?,
            note_links: self.note_links()?,
        })
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
