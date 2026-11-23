//! Catálogo de contenidos de Code Lab.
//!
//! Las lecciones y preguntas se declaran aquí como datos, no como código de
//! UI. La infraestructura las inserta de forma idempotente en SQLite.

mod csharp;

use crate::code_lab::domain::Difficulty;

pub use csharp::{csharp_catalog, csharp_exercises};

/// Datos declarativos de un lenguaje.
#[derive(Debug, Clone)]
pub struct LanguageSeed {
    pub code: &'static str,
    pub name: &'static str,
    pub sdk_command: &'static str,
}

/// Datos declarativos de una ruta de aprendizaje.
#[derive(Debug, Clone)]
pub struct PathSeed {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub level: i32,
}

/// Pregunta declarativa de un cuestionario.
#[derive(Debug, Clone)]
pub struct QuestionSeed {
    pub prompt: &'static str,
    pub options: &'static [&'static str],
    pub correct_index: usize,
    pub explanation: &'static str,
}

/// Lección declarativa con su cuestionario.
#[derive(Debug, Clone)]
pub struct LessonSeed {
    pub path_slug: &'static str,
    pub slug: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub content_md: &'static str,
    pub concepts: &'static [&'static str],
    pub order_index: i32,
    pub estimated_minutes: i32,
    pub questions: &'static [QuestionSeed],
}

/// Ejemplo visible en el enunciado de un ejercicio.
#[derive(Debug, Clone)]
pub struct ExampleSeed {
    pub input: &'static str,
    pub output: &'static str,
    pub explanation: &'static str,
}

/// Caso de prueba declarativo.
#[derive(Debug, Clone)]
pub struct TestCaseSeed {
    pub name: &'static str,
    pub input: &'static str,
    pub expected_output: &'static str,
    pub visible: bool,
}

/// Ejercicio declarativo, incluida su solución de referencia.
#[derive(Debug, Clone)]
pub struct ExerciseSeed {
    pub path_slug: &'static str,
    pub lesson_slug: Option<&'static str>,
    pub slug: &'static str,
    pub title: &'static str,
    pub difficulty: Difficulty,
    pub topic: &'static str,
    pub level: i32,
    pub concepts: &'static [&'static str],
    pub description_md: &'static str,
    pub examples: &'static [ExampleSeed],
    pub constraints: &'static [&'static str],
    pub hints: &'static [&'static str],
    pub starter_code: &'static str,
    pub reference_solution: &'static str,
    pub test_cases: &'static [TestCaseSeed],
}

/// Catálogo completo de un lenguaje.
#[derive(Debug, Clone)]
pub struct CatalogSeed {
    pub language: LanguageSeed,
    pub paths: &'static [PathSeed],
    pub lessons: &'static [LessonSeed],
    pub exercises: &'static [ExerciseSeed],
}

/// Catálogos disponibles en esta versión.
pub fn all_catalogs() -> Vec<CatalogSeed> {
    vec![csharp_catalog()]
}
