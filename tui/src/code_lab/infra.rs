//! Persistencia de Code Lab. No depende de Ratatui ni de procesos externos.

use std::collections::HashMap;
use std::path::PathBuf;

use rusqlite::{Connection, OptionalExtension, Row};
use serde::Serialize;

use crate::code_lab::catalog::{TestCaseSeed, all_catalogs};
use crate::code_lab::domain::{
    AnswerResult, AttemptStatus, AttemptSummary, CodeLabItem, CodeLabStats, ConceptReview, Example,
    Exercise, ExerciseProgress, ExerciseStatus, ItemType, Lesson, LessonProgress, LessonStatus,
    QuizQuestion, QuizResult, TestCase, TestSpec, lesson_status_after_open,
    lesson_status_after_quiz,
};
use crate::domain::{ReviewOutcome, ReviewRating, schedule_review};
use crate::error::BasicError;
use crate::infra::catalog::unix_now;
use crate::infra::sqlite::Database;

/// Detalle completo de una lección, incluido su cuestionario.
#[derive(Debug, Clone)]
pub struct LessonDetail {
    pub lesson: Lesson,
    pub questions: Vec<QuizQuestion>,
}

/// Resultado de poblar el catálogo.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SeedReport {
    pub languages: i32,
    pub paths: i32,
    pub lessons: i32,
    pub questions: i32,
    pub exercises: i32,
}

/// Inserta o actualiza el catálogo declarativo en una sola transacción.
pub fn seed_catalog(database: &mut Database) -> Result<SeedReport, BasicError> {
    let now = unix_now();
    let mut report = SeedReport::default();
    let transaction = database.connection_mut().transaction()?;

    for catalog in all_catalogs() {
        transaction.execute(
            "INSERT INTO code_lab_languages (code, name, sdk_command, enabled)
             VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(code) DO UPDATE SET
                 name = excluded.name,
                 sdk_command = excluded.sdk_command,
                 enabled = 1",
            rusqlite::params![
                catalog.language.code,
                catalog.language.name,
                catalog.language.sdk_command
            ],
        )?;
        report.languages += 1;

        let mut path_ids = HashMap::new();
        for path in catalog.paths {
            transaction.execute(
                "INSERT INTO code_lab_paths (language, slug, title, description, level)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(language, slug) DO UPDATE SET
                     title = excluded.title,
                     description = excluded.description,
                     level = excluded.level",
                rusqlite::params![
                    catalog.language.code,
                    path.slug,
                    path.title,
                    path.description,
                    path.level
                ],
            )?;
            let id: i64 = transaction.query_row(
                "SELECT id FROM code_lab_paths WHERE language = ?1 AND slug = ?2",
                rusqlite::params![catalog.language.code, path.slug],
                |row| row.get(0),
            )?;
            path_ids.insert(path.slug, id);
            report.paths += 1;
        }

        let mut lesson_ids = HashMap::new();
        for lesson in catalog.lessons {
            let path_id = path_ids.get(lesson.path_slug).copied().ok_or_else(|| {
                BasicError::InvalidConfig {
                    path: PathBuf::from("code_lab/catalog"),
                    message: format!("ruta desconocida: {}", lesson.path_slug),
                }
            })?;
            let concepts = to_json(&lesson.concepts)?;
            for concept in lesson.concepts {
                transaction.execute(
                    "INSERT OR IGNORE INTO code_lab_concept_review
                     (language, concept, level, ease_factor)
                     VALUES (?1, ?2, ?3, 2.5)",
                    rusqlite::params![catalog.language.code, concept, lesson.order_index],
                )?;
            }
            transaction.execute(
                "INSERT INTO code_lab_lessons
                 (path_id, slug, title, summary, content_md, concepts_json,
                  order_index, estimated_minutes, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                 ON CONFLICT(path_id, slug) DO UPDATE SET
                     title = excluded.title,
                     summary = excluded.summary,
                     content_md = excluded.content_md,
                     concepts_json = excluded.concepts_json,
                     order_index = excluded.order_index,
                     estimated_minutes = excluded.estimated_minutes",
                rusqlite::params![
                    path_id,
                    lesson.slug,
                    lesson.title,
                    lesson.summary,
                    lesson.content_md,
                    concepts,
                    lesson.order_index,
                    lesson.estimated_minutes,
                    now
                ],
            )?;
            let lesson_id: i64 = transaction.query_row(
                "SELECT id FROM code_lab_lessons WHERE path_id = ?1 AND slug = ?2",
                rusqlite::params![path_id, lesson.slug],
                |row| row.get(0),
            )?;
            lesson_ids.insert(lesson.slug, lesson_id);
            report.lessons += 1;

            for (index, question) in lesson.questions.iter().enumerate() {
                let options = to_json(&question.options)?;
                transaction.execute(
                    "INSERT INTO code_lab_quiz_questions
                     (lesson_id, prompt, options_json, correct_index, explanation, order_index)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                     ON CONFLICT(lesson_id, order_index) DO UPDATE SET
                         prompt = excluded.prompt,
                         options_json = excluded.options_json,
                         correct_index = excluded.correct_index,
                         explanation = excluded.explanation",
                    rusqlite::params![
                        lesson_id,
                        question.prompt,
                        options,
                        i64::try_from(question.correct_index).unwrap_or(i64::MAX),
                        question.explanation,
                        i32::try_from(index).unwrap_or(i32::MAX)
                    ],
                )?;
                report.questions += 1;
            }
        }

        for exercise in catalog.exercises {
            let path_id = path_ids.get(exercise.path_slug).copied();
            let lesson_id = exercise
                .lesson_slug
                .and_then(|slug| lesson_ids.get(slug).copied());
            let concepts = to_json(&exercise.concepts)?;
            for concept in exercise.concepts {
                transaction.execute(
                    "INSERT OR IGNORE INTO code_lab_concept_review
                     (language, concept, level, ease_factor)
                     VALUES (?1, ?2, ?3, 2.5)",
                    rusqlite::params![catalog.language.code, concept, exercise.level],
                )?;
            }
            let examples = to_json(
                &exercise
                    .examples
                    .iter()
                    .map(|example| Example {
                        input: example.input.to_string(),
                        output: example.output.to_string(),
                        explanation: if example.explanation.is_empty() {
                            None
                        } else {
                            Some(example.explanation.to_string())
                        },
                    })
                    .collect::<Vec<_>>(),
            )?;
            let constraints = to_json(&exercise.constraints)?;
            let hints = to_json(&exercise.hints)?;
            let test_spec = TestSpec {
                cases: exercise.test_cases.iter().map(test_case_seed).collect(),
                ..TestSpec::default()
            };
            let tests = to_json(&test_spec)?;
            transaction.execute(
                "INSERT INTO code_lab_exercises
                 (language, path_id, lesson_id, slug, title, difficulty, topic, level,
                  concepts_json, description_md, examples_json, constraints_json,
                  hints_json, starter_code, reference_solution, test_spec_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
                 ON CONFLICT(language, slug) DO UPDATE SET
                     path_id = excluded.path_id,
                     lesson_id = excluded.lesson_id,
                     title = excluded.title,
                     difficulty = excluded.difficulty,
                     topic = excluded.topic,
                     level = excluded.level,
                     concepts_json = excluded.concepts_json,
                     description_md = excluded.description_md,
                     examples_json = excluded.examples_json,
                     constraints_json = excluded.constraints_json,
                     hints_json = excluded.hints_json,
                     starter_code = excluded.starter_code,
                     reference_solution = excluded.reference_solution,
                     test_spec_json = excluded.test_spec_json",
                rusqlite::params![
                    catalog.language.code,
                    path_id,
                    lesson_id,
                    exercise.slug,
                    exercise.title,
                    exercise.difficulty.as_str(),
                    exercise.topic,
                    exercise.level,
                    concepts,
                    exercise.description_md,
                    examples,
                    constraints,
                    hints,
                    exercise.starter_code,
                    exercise.reference_solution,
                    tests,
                    now
                ],
            )?;
            report.exercises += 1;
        }
    }

    transaction.commit()?;
    Ok(report)
}

fn test_case_seed(case: &TestCaseSeed) -> TestCase {
    TestCase {
        name: case.name.to_string(),
        input: case.input.to_string(),
        expected_output: case.expected_output.to_string(),
        visible: case.visible,
    }
}

/// Lista las lecciones de un lenguaje en orden de la ruta.
pub fn list_lessons(database: &Database, language: &str) -> Result<Vec<Lesson>, BasicError> {
    let mut statement = database.connection().prepare(
        "SELECT l.id, l.path_id, l.slug, l.title, l.summary, l.content_md,
                l.concepts_json, l.order_index, l.estimated_minutes
         FROM code_lab_lessons l
         JOIN code_lab_paths p ON p.id = l.path_id
         WHERE p.language = ?1
         ORDER BY p.level, l.order_index, l.id",
    )?;
    let rows = statement.query_map([language], row_to_lesson)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(BasicError::from)
}

/// Devuelve una lección con su cuestionario, o `None` si no existe.
pub fn lesson_detail(
    database: &Database,
    lesson_id: i64,
) -> Result<Option<LessonDetail>, BasicError> {
    let connection = database.connection();
    let Some(lesson) = connection
        .query_row(
            "SELECT id, path_id, slug, title, summary, content_md,
                    concepts_json, order_index, estimated_minutes
             FROM code_lab_lessons WHERE id = ?1",
            [lesson_id],
            row_to_lesson,
        )
        .optional()?
    else {
        return Ok(None);
    };

    let mut statement = connection.prepare(
        "SELECT id, lesson_id, prompt, options_json, correct_index, explanation, order_index
         FROM code_lab_quiz_questions WHERE lesson_id = ?1 ORDER BY order_index, id",
    )?;
    let questions = statement
        .query_map([lesson_id], row_to_question)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(LessonDetail { lesson, questions }))
}

/// Progreso actual de una lección; si no existe, devuelve un estado inicial.
pub fn lesson_progress(database: &Database, lesson_id: i64) -> Result<LessonProgress, BasicError> {
    Ok(
        read_lesson_progress(database.connection(), lesson_id)?.unwrap_or_else(|| LessonProgress {
            lesson_id,
            ..LessonProgress::default()
        }),
    )
}

/// Abre una lección. Nunca la marca como completada.
pub fn open_lesson(database: &mut Database, lesson_id: i64) -> Result<LessonProgress, BasicError> {
    let now = unix_now();
    let mut progress =
        read_lesson_progress(database.connection(), lesson_id)?.unwrap_or_else(|| LessonProgress {
            lesson_id,
            ..LessonProgress::default()
        });
    progress.status = lesson_status_after_open(progress.status);
    progress.last_opened_at = Some(now);
    progress.updated_at = now;

    let transaction = database.connection_mut().transaction()?;
    upsert_lesson_progress(&transaction, &progress)?;
    transaction.commit()?;
    Ok(progress)
}

/// Acumula tiempo propio de Code Lab, sin tocar el Pomodoro.
pub fn record_lesson_time(
    database: &mut Database,
    lesson_id: i64,
    seconds: i64,
) -> Result<(), BasicError> {
    if seconds <= 0 {
        return Ok(());
    }
    let now = unix_now();
    let updated = database.connection().execute(
        "UPDATE code_lab_lesson_progress
         SET seconds_spent = seconds_spent + ?1, updated_at = ?2
         WHERE lesson_id = ?3",
        rusqlite::params![seconds, now, lesson_id],
    )?;
    if updated == 0 {
        let progress = LessonProgress {
            lesson_id,
            status: LessonStatus::InProgress,
            seconds_spent: seconds,
            updated_at: now,
            ..LessonProgress::default()
        };
        let transaction = database.connection_mut().transaction()?;
        upsert_lesson_progress(&transaction, &progress)?;
        transaction.commit()?;
    }
    Ok(())
}

/// Corrige un cuestionario y persiste el mejor resultado y el estado.
pub fn submit_quiz(
    database: &mut Database,
    lesson_id: i64,
    answers: &[Option<usize>],
) -> Result<QuizResult, BasicError> {
    let now = unix_now();
    let detail = lesson_detail(database, lesson_id)?.ok_or_else(|| BasicError::InvalidConfig {
        path: PathBuf::from("code_lab/lessons"),
        message: format!("lección {lesson_id} no encontrada"),
    })?;
    let mut progress =
        read_lesson_progress(database.connection(), lesson_id)?.unwrap_or_else(|| LessonProgress {
            lesson_id,
            ..LessonProgress::default()
        });

    let mut correct = 0;
    let mut details = Vec::new();
    for (index, question) in detail.questions.iter().enumerate() {
        let answer = answers.get(index).copied().flatten();
        let is_correct = answer.is_some_and(|value| question.is_correct(value));
        if is_correct {
            correct += 1;
        }
        details.push(AnswerResult {
            question_id: question.id,
            answer,
            correct: is_correct,
            correct_index: question.correct_index,
            explanation: question.explanation.clone(),
        });
    }

    let total = detail.questions.len() as i32;
    let status = lesson_status_after_quiz(progress.status, correct, total);
    progress.status = status;
    progress.quiz_correct = correct;
    progress.quiz_total = total;
    if status == LessonStatus::Completed && progress.completed_at.is_none() {
        progress.completed_at = Some(now);
    }
    if status == LessonStatus::NeedsReview {
        progress.review_due_at = Some(now + 86_400);
    }
    progress.updated_at = now;

    let transaction = database.connection_mut().transaction()?;
    upsert_lesson_progress(&transaction, &progress)?;
    transaction.commit()?;

    Ok(QuizResult {
        correct,
        total,
        status,
        answers: details,
    })
}

const EXERCISE_COLUMNS: &str = "id, language, path_id, lesson_id, slug, title, difficulty, \
     topic, level, concepts_json, description_md, examples_json, \
     constraints_json, hints_json, starter_code, test_spec_json";

/// Lista los ejercicios de un lenguaje en orden de dificultad.
pub fn list_exercises(database: &Database, language: &str) -> Result<Vec<Exercise>, BasicError> {
    let sql = format!(
        "SELECT {EXERCISE_COLUMNS} FROM code_lab_exercises \
         WHERE language = ?1 ORDER BY level, difficulty, id"
    );
    let mut statement = database.connection().prepare(&sql)?;
    let rows = statement.query_map([language], row_to_exercise)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(BasicError::from)
}

/// Devuelve un ejercicio concreto, incluido su código inicial y sus pruebas.
pub fn exercise_detail(
    database: &Database,
    exercise_id: i64,
) -> Result<Option<Exercise>, BasicError> {
    let sql = format!("SELECT {EXERCISE_COLUMNS} FROM code_lab_exercises WHERE id = ?1");
    database
        .connection()
        .query_row(&sql, [exercise_id], row_to_exercise)
        .optional()
        .map_err(BasicError::from)
}

/// La solución de referencia solo se entrega cuando se pide explícitamente.
pub fn reveal_solution(
    database: &Database,
    exercise_id: i64,
) -> Result<Option<String>, BasicError> {
    database
        .connection()
        .query_row(
            "SELECT reference_solution FROM code_lab_exercises WHERE id = ?1",
            [exercise_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(BasicError::from)
}

/// Progreso actual de un ejercicio; si no existe, devuelve un estado inicial.
pub fn exercise_progress(
    database: &Database,
    exercise_id: i64,
) -> Result<ExerciseProgress, BasicError> {
    Ok(
        read_exercise_progress(database.connection(), exercise_id)?.unwrap_or_else(|| {
            ExerciseProgress {
                exercise_id,
                ..ExerciseProgress::default()
            }
        }),
    )
}

/// Registra la apertura de un ejercicio. Nunca cuenta como intento ni lo
/// marca como resuelto.
pub fn open_exercise(
    database: &mut Database,
    exercise_id: i64,
) -> Result<ExerciseProgress, BasicError> {
    let now = unix_now();
    database.connection().execute(
        "INSERT INTO code_lab_exercise_progress
         (exercise_id, status, last_opened_at, updated_at)
         VALUES (?1, 'not_started', ?2, ?2)
         ON CONFLICT(exercise_id) DO UPDATE SET
             last_opened_at = excluded.last_opened_at,
             updated_at = excluded.updated_at",
        rusqlite::params![exercise_id, now],
    )?;
    exercise_progress(database, exercise_id)
}

/// Carga el borrador guardado para un ejercicio, si existe.
pub fn load_draft(database: &Database, exercise_id: i64) -> Result<Option<String>, BasicError> {
    database
        .connection()
        .query_row(
            "SELECT code FROM code_lab_drafts WHERE exercise_id = ?1",
            [exercise_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(BasicError::from)
}

/// Guarda el código actual del editor como borrador.
pub fn save_draft(database: &mut Database, exercise_id: i64, code: &str) -> Result<(), BasicError> {
    let language: String = database.connection().query_row(
        "SELECT language FROM code_lab_exercises WHERE id = ?1",
        [exercise_id],
        |row| row.get(0),
    )?;
    let now = unix_now();
    database.connection().execute(
        "INSERT INTO code_lab_drafts (language, exercise_id, code, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(exercise_id) DO UPDATE SET
             code = excluded.code,
             updated_at = excluded.updated_at",
        rusqlite::params![language, exercise_id, code, now],
    )?;
    Ok(())
}

/// Persiste un intento, sus trazas y el progreso derivado en una transacción.
pub fn record_attempt(
    database: &mut Database,
    exercise_id: i64,
    code: &str,
    summary: &AttemptSummary,
) -> Result<i64, BasicError> {
    let now = unix_now();
    let transaction = database.connection_mut().transaction()?;
    transaction.execute(
        "INSERT INTO code_lab_attempts
         (exercise_id, started_at, finished_at, duration_seconds, status,
          tests_passed, tests_total, compile_ok, language, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?2)",
        rusqlite::params![
            exercise_id,
            now - summary.duration_seconds(),
            now,
            summary.duration_seconds(),
            summary.status.as_str(),
            summary.tests_passed,
            summary.tests_total,
            summary.compile_ok.map(i32::from),
            crate::code_lab::domain::DEFAULT_LANGUAGE
        ],
    )?;
    let attempt_id = transaction.last_insert_rowid();
    transaction.execute(
        "INSERT INTO code_lab_attempt_code (attempt_id, code) VALUES (?1, ?2)",
        rusqlite::params![attempt_id, code],
    )?;

    for execution in &summary.executions {
        transaction.execute(
            "INSERT INTO code_lab_executions
             (attempt_id, phase, command, exit_code, stdout, stderr, duration_ms, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                attempt_id,
                execution.phase.as_str(),
                execution.command,
                execution.exit_code,
                execution.stdout,
                execution.stderr,
                i64::try_from(execution.duration_ms).unwrap_or(i64::MAX),
                now
            ],
        )?;
    }

    let mut progress =
        read_exercise_progress(&transaction, exercise_id)?.unwrap_or_else(|| ExerciseProgress {
            exercise_id,
            ..ExerciseProgress::default()
        });
    progress.attempts += 1;
    progress.tests_total = summary.tests_total;
    progress.best_tests_passed = progress.best_tests_passed.max(summary.tests_passed);
    progress.best_score = progress.best_score.max(summary.score());
    progress.last_attempt_id = Some(attempt_id);
    progress.last_opened_at = Some(progress.last_opened_at.unwrap_or(now));
    progress.updated_at = now;
    if summary.status == AttemptStatus::Passed {
        progress.status = ExerciseStatus::Solved;
        if progress.solved_at.is_none() {
            progress.solved_at = Some(now);
        }
    } else if progress.status != ExerciseStatus::Solved {
        progress.status = ExerciseStatus::Attempted;
    }
    upsert_exercise_progress(&transaction, &progress)?;
    transaction.commit()?;
    Ok(attempt_id)
}

/// Resumen agregado de Code Lab para la TUI y la CLI.
pub fn code_lab_stats(database: &Database, now: i64) -> Result<CodeLabStats, BasicError> {
    Ok(database.connection().query_row(
        "SELECT
            (SELECT count(*) FROM code_lab_languages),
            (SELECT count(*) FROM code_lab_paths),
            (SELECT count(*) FROM code_lab_lessons),
            (SELECT count(*) FROM code_lab_lesson_progress WHERE status = 'completed'),
            (SELECT count(*) FROM code_lab_exercises),
            (SELECT count(*) FROM code_lab_exercise_progress WHERE status = 'solved'),
            (SELECT count(*) FROM code_lab_attempts),
            (SELECT count(*) FROM code_lab_attempts WHERE status = 'passed'),
            (SELECT COALESCE(sum(seconds_spent), 0) FROM code_lab_lesson_progress)
             + (SELECT COALESCE(sum(seconds_spent), 0) FROM code_lab_exercise_progress),
            (SELECT count(*) FROM code_lab_concept_review
             WHERE due_at IS NULL OR due_at <= ?1)",
        [now],
        |row| {
            let count = |index: usize| -> Result<i32, rusqlite::Error> {
                i32::try_from(row.get::<_, i64>(index)?).map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        index,
                        "count".to_string(),
                        rusqlite::types::Type::Integer,
                    )
                })
            };
            Ok(CodeLabStats {
                languages: count(0)?,
                paths: count(1)?,
                lessons: count(2)?,
                lessons_completed: count(3)?,
                exercises: count(4)?,
                exercises_solved: count(5)?,
                attempts: count(6)?,
                attempts_passed: count(7)?,
                seconds_spent: row.get(8)?,
                concepts_due: count(9)?,
            })
        },
    )?)
}

/// Conceptos con repaso pendiente, ordenados por antigüedad.
pub fn due_concepts(database: &Database, now: i64) -> Result<Vec<ConceptReview>, BasicError> {
    let mut statement = database.connection().prepare(
        "SELECT id, language, concept, level, interval_days, ease_factor,
                due_at, lapses, last_reviewed_at
         FROM code_lab_concept_review
         WHERE due_at IS NULL OR due_at <= ?1
         ORDER BY due_at IS NULL DESC, due_at, concept
         LIMIT 50",
    )?;
    statement
        .query_map([now], row_to_concept)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(BasicError::from)
}

/// Califica un concepto y programa su siguiente repaso con SM-2.
pub fn rate_concept(
    database: &mut Database,
    concept_id: i64,
    rating: ReviewRating,
) -> Result<ReviewOutcome, BasicError> {
    let now = unix_now();
    let previous = database
        .connection()
        .query_row(
            "SELECT language, concept, level, interval_days, ease_factor, lapses
             FROM code_lab_concept_review WHERE id = ?1",
            [concept_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i32>(5)?,
                ))
            },
        )
        .optional()?
        .ok_or_else(|| BasicError::InvalidConfig {
            path: PathBuf::from("code_lab/concepts"),
            message: format!("concepto {concept_id} no encontrado"),
        })?;
    let outcome = schedule_review(rating, i64::from(previous.3), previous.4);
    let next_due = now + outcome.due_in_days * 86_400;
    database.connection().execute(
        "INSERT INTO code_lab_concept_review
         (id, language, concept, level, interval_days, ease_factor, due_at,
          lapses, last_reviewed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(id) DO UPDATE SET
             interval_days = excluded.interval_days,
             ease_factor = excluded.ease_factor,
             due_at = excluded.due_at,
             lapses = excluded.lapses,
             last_reviewed_at = excluded.last_reviewed_at",
        rusqlite::params![
            concept_id,
            previous.0,
            previous.1,
            previous.2,
            outcome.interval_days,
            outcome.ease_factor,
            next_due,
            previous.5 + i32::from(rating == ReviewRating::Again),
            now
        ],
    )?;
    Ok(outcome)
}

/// Lecciones en curso o a repasar y ejercicios intentados o a repasar.
pub fn learning_items(database: &Database) -> Result<Vec<CodeLabItem>, BasicError> {
    let mut statement = database.connection().prepare(
        "SELECT id, kind, title, subtitle, status, due_at FROM (
            SELECT l.id AS id, 'lesson' AS kind, l.title AS title,
                   l.summary AS subtitle, COALESCE(p.status, 'not_started') AS status,
                   p.review_due_at AS due_at, l.order_index AS ordering
            FROM code_lab_lessons l
            LEFT JOIN code_lab_lesson_progress p ON p.lesson_id = l.id
            WHERE COALESCE(p.status, 'not_started') IN ('in_progress', 'needs_review')
            UNION ALL
            SELECT e.id, 'exercise', e.title, e.topic,
                   COALESCE(p.status, 'not_started'), p.updated_at, e.level
            FROM code_lab_exercises e
            LEFT JOIN code_lab_exercise_progress p ON p.exercise_id = e.id
            WHERE COALESCE(p.status, 'not_started') IN ('attempted', 'needs_review')
         ) ORDER BY ordering, id LIMIT 40",
    )?;
    statement
        .query_map([], |row| {
            let kind = match row.get::<_, String>(1)?.as_str() {
                "exercise" => ItemType::Exercise,
                _ => ItemType::Lesson,
            };
            let status = row.get::<_, String>(4)?;
            let label = match kind {
                ItemType::Lesson => LessonStatus::parse(&status).label(),
                ItemType::Exercise => ExerciseStatus::parse(&status).label(),
            };
            Ok(CodeLabItem {
                id: row.get(0)?,
                kind,
                title: row.get(2)?,
                subtitle: row.get(3)?,
                status: label.to_string(),
                due_at: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(BasicError::from)
}

fn row_to_concept(row: &Row<'_>) -> Result<ConceptReview, rusqlite::Error> {
    Ok(ConceptReview {
        id: row.get(0)?,
        language: row.get(1)?,
        concept: row.get(2)?,
        level: row.get(3)?,
        interval_days: row.get(4)?,
        ease_factor: row.get(5)?,
        due_at: row.get(6)?,
        lapses: row.get(7)?,
        last_reviewed_at: row.get(8)?,
    })
}

fn row_to_lesson(row: &Row<'_>) -> Result<Lesson, rusqlite::Error> {
    Ok(Lesson {
        id: row.get(0)?,
        path_id: row.get(1)?,
        slug: row.get(2)?,
        title: row.get(3)?,
        summary: row.get(4)?,
        content_md: row.get(5)?,
        concepts: from_json(&row.get::<_, String>(6)?),
        order_index: row.get(7)?,
        estimated_minutes: row.get(8)?,
    })
}

fn row_to_question(row: &Row<'_>) -> Result<QuizQuestion, rusqlite::Error> {
    Ok(QuizQuestion {
        id: row.get(0)?,
        lesson_id: row.get(1)?,
        prompt: row.get(2)?,
        options: from_json(&row.get::<_, String>(3)?),
        correct_index: usize::try_from(row.get::<_, i64>(4)?).unwrap_or(usize::MAX),
        explanation: row.get(5)?,
        order_index: row.get(6)?,
    })
}

fn row_to_exercise(row: &Row<'_>) -> Result<Exercise, rusqlite::Error> {
    let difficulty = crate::code_lab::domain::Difficulty::parse(&row.get::<_, String>(6)?)
        .ok_or_else(|| {
            rusqlite::Error::InvalidColumnType(
                6,
                "difficulty".to_string(),
                rusqlite::types::Type::Text,
            )
        })?;
    Ok(Exercise {
        id: row.get(0)?,
        language: row.get(1)?,
        path_id: row.get(2)?,
        lesson_id: row.get(3)?,
        slug: row.get(4)?,
        title: row.get(5)?,
        difficulty,
        topic: row.get(7)?,
        level: row.get(8)?,
        concepts: from_json(&row.get::<_, String>(9)?),
        description_md: row.get(10)?,
        examples: from_json(&row.get::<_, String>(11)?),
        constraints: from_json(&row.get::<_, String>(12)?),
        hints: from_json(&row.get::<_, String>(13)?),
        starter_code: row.get(14)?,
        test_spec: from_json(&row.get::<_, String>(15)?),
    })
}

fn read_exercise_progress(
    connection: &Connection,
    exercise_id: i64,
) -> Result<Option<ExerciseProgress>, BasicError> {
    connection
        .query_row(
            "SELECT exercise_id, status, attempts, solved_at, last_opened_at,
                    best_tests_passed, tests_total, best_score, seconds_spent,
                    last_attempt_id, updated_at
             FROM code_lab_exercise_progress WHERE exercise_id = ?1",
            [exercise_id],
            |row| {
                Ok(ExerciseProgress {
                    exercise_id: row.get(0)?,
                    status: ExerciseStatus::parse(&row.get::<_, String>(1)?),
                    attempts: row.get(2)?,
                    solved_at: row.get(3)?,
                    last_opened_at: row.get(4)?,
                    best_tests_passed: row.get(5)?,
                    tests_total: row.get(6)?,
                    best_score: row.get(7)?,
                    seconds_spent: row.get(8)?,
                    last_attempt_id: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            },
        )
        .optional()
        .map_err(BasicError::from)
}

fn upsert_exercise_progress(
    connection: &Connection,
    progress: &ExerciseProgress,
) -> Result<(), BasicError> {
    connection.execute(
        "INSERT INTO code_lab_exercise_progress
         (exercise_id, status, attempts, solved_at, last_opened_at, best_tests_passed,
          tests_total, best_score, seconds_spent, last_attempt_id, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
         ON CONFLICT(exercise_id) DO UPDATE SET
             status = excluded.status,
             attempts = excluded.attempts,
             solved_at = excluded.solved_at,
             last_opened_at = excluded.last_opened_at,
             best_tests_passed = excluded.best_tests_passed,
             tests_total = excluded.tests_total,
             best_score = excluded.best_score,
             seconds_spent = excluded.seconds_spent,
             last_attempt_id = excluded.last_attempt_id,
             updated_at = excluded.updated_at",
        rusqlite::params![
            progress.exercise_id,
            progress.status.as_str(),
            progress.attempts,
            progress.solved_at,
            progress.last_opened_at,
            progress.best_tests_passed,
            progress.tests_total,
            progress.best_score,
            progress.seconds_spent,
            progress.last_attempt_id,
            progress.updated_at
        ],
    )?;
    Ok(())
}

fn read_lesson_progress(
    connection: &Connection,
    lesson_id: i64,
) -> Result<Option<LessonProgress>, BasicError> {
    connection
        .query_row(
            "SELECT lesson_id, status, last_opened_at, completed_at, quiz_correct,
                    quiz_total, seconds_spent, review_due_at, updated_at
             FROM code_lab_lesson_progress WHERE lesson_id = ?1",
            [lesson_id],
            |row| {
                Ok(LessonProgress {
                    lesson_id: row.get(0)?,
                    status: LessonStatus::parse(&row.get::<_, String>(1)?),
                    last_opened_at: row.get(2)?,
                    completed_at: row.get(3)?,
                    quiz_correct: row.get(4)?,
                    quiz_total: row.get(5)?,
                    seconds_spent: row.get(6)?,
                    review_due_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
        .optional()
        .map_err(BasicError::from)
}

fn upsert_lesson_progress(
    connection: &Connection,
    progress: &LessonProgress,
) -> Result<(), BasicError> {
    connection.execute(
        "INSERT INTO code_lab_lesson_progress
         (lesson_id, status, last_opened_at, completed_at, quiz_correct, quiz_total,
          seconds_spent, review_due_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(lesson_id) DO UPDATE SET
             status = excluded.status,
             last_opened_at = excluded.last_opened_at,
             completed_at = excluded.completed_at,
             quiz_correct = excluded.quiz_correct,
             quiz_total = excluded.quiz_total,
             seconds_spent = excluded.seconds_spent,
             review_due_at = excluded.review_due_at,
             updated_at = excluded.updated_at",
        rusqlite::params![
            progress.lesson_id,
            progress.status.as_str(),
            progress.last_opened_at,
            progress.completed_at,
            progress.quiz_correct,
            progress.quiz_total,
            progress.seconds_spent,
            progress.review_due_at,
            progress.updated_at
        ],
    )?;
    Ok(())
}

fn to_json<T: Serialize>(value: &T) -> Result<String, BasicError> {
    Ok(serde_json::to_string(value)?)
}

fn from_json<T: serde::de::DeserializeOwned + Default>(text: &str) -> T {
    serde_json::from_str(text).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded() -> Database {
        let mut database = Database::open_in_memory().unwrap();
        seed_catalog(&mut database).unwrap();
        database
    }

    fn first_lesson(database: &Database) -> i64 {
        list_lessons(database, "csharp").unwrap()[0].id
    }

    #[test]
    fn seed_is_idempotent() {
        let mut database = seeded();
        let first = seed_catalog(&mut database).unwrap();
        let second = seed_catalog(&mut database).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.lessons, 5);
        assert_eq!(first.questions, 10);
        assert_eq!(first.exercises, 6);
        assert_eq!(list_lessons(&database, "csharp").unwrap().len(), 5);
        assert_eq!(list_exercises(&database, "csharp").unwrap().len(), 6);
    }

    #[test]
    fn opening_a_lesson_does_not_complete_it() {
        let mut database = seeded();
        let id = first_lesson(&database);
        let progress = open_lesson(&mut database, id).unwrap();
        assert_eq!(progress.status, LessonStatus::InProgress);
        assert!(progress.completed_at.is_none());
        assert!(progress.last_opened_at.is_some());
        assert!(!lesson_status_after_open(LessonStatus::InProgress).unlocks_next());
        let stored = lesson_progress(&database, id).unwrap();
        assert_eq!(stored.status, LessonStatus::InProgress);
    }

    #[test]
    fn quiz_completion_requires_every_answer() {
        let mut database = seeded();
        let id = first_lesson(&database);
        open_lesson(&mut database, id).unwrap();

        let partial = submit_quiz(&mut database, id, &[Some(0), Some(99)]).unwrap();
        assert_eq!(partial.status, LessonStatus::NeedsReview);
        assert_eq!(partial.correct, 1);
        assert!(!partial.perfect());

        let detail = lesson_detail(&database, id).unwrap().unwrap();
        let answers: Vec<Option<usize>> = detail
            .questions
            .iter()
            .map(|question| Some(question.correct_index))
            .collect();
        let perfect = submit_quiz(&mut database, id, &answers).unwrap();
        assert_eq!(perfect.status, LessonStatus::Completed);
        assert!(perfect.perfect());
        let stored = lesson_progress(&database, id).unwrap();
        assert_eq!(stored.quiz_correct, 2);
        assert_eq!(stored.quiz_total, 2);
        assert!(stored.completed_at.is_some());
    }

    #[test]
    fn lesson_time_is_accumulated_separately() {
        let mut database = seeded();
        let id = first_lesson(&database);
        open_lesson(&mut database, id).unwrap();
        record_lesson_time(&mut database, id, 90).unwrap();
        record_lesson_time(&mut database, id, 30).unwrap();
        assert_eq!(lesson_progress(&database, id).unwrap().seconds_spent, 120);
    }

    #[test]
    fn exercises_expose_starter_but_not_solution_through_the_domain() {
        let database = seeded();
        let exercises = list_exercises(&database, "csharp").unwrap();
        let first = exercises
            .iter()
            .find(|exercise| exercise.slug == "suma-dos-numeros")
            .unwrap();
        assert!(first.starter_code.contains("TODO"));
        assert_eq!(first.test_spec.cases.len(), 4);
        assert_eq!(first.visible_cases().count(), 2);
        assert_eq!(first.hidden_case_count(), 2);
    }

    #[test]
    fn opening_an_exercise_does_not_create_an_attempt() {
        let mut database = seeded();
        let id = list_exercises(&database, "csharp").unwrap()[0].id;
        let progress = open_exercise(&mut database, id).unwrap();
        assert_eq!(progress.status, ExerciseStatus::NotStarted);
        assert_eq!(progress.attempts, 0);
        assert!(progress.last_opened_at.is_some());
        assert!(exercise_detail(&database, id).unwrap().is_some());
    }

    #[test]
    fn drafts_and_attempts_are_persisted_together() {
        use crate::code_lab::domain::{ExecutionPhase, ExecutionResult};

        let mut database = seeded();
        let id = list_exercises(&database, "csharp").unwrap()[0].id;
        open_exercise(&mut database, id).unwrap();
        save_draft(&mut database, id, "// código propio\n").unwrap();
        assert_eq!(
            load_draft(&database, id).unwrap().as_deref(),
            Some("// código propio\n")
        );

        let failed = AttemptSummary {
            status: AttemptStatus::Failed,
            duration_ms: 1200,
            tests_passed: 1,
            tests_total: 4,
            compile_ok: Some(true),
            executions: vec![ExecutionResult {
                phase: ExecutionPhase::Build,
                command: "dotnet build".to_string(),
                exit_code: Some(0),
                stdout: "ok".to_string(),
                stderr: String::new(),
                duration_ms: 900,
            }],
        };
        let attempt_id = record_attempt(&mut database, id, "// código propio\n", &failed).unwrap();
        let progress = exercise_progress(&database, id).unwrap();
        assert_eq!(progress.status, ExerciseStatus::Attempted);
        assert_eq!(progress.attempts, 1);
        assert_eq!(progress.best_tests_passed, 1);
        assert!((progress.best_score - 0.25).abs() < f64::EPSILON);
        assert_eq!(progress.last_attempt_id, Some(attempt_id));
        assert_eq!(progress.seconds_spent, 0);

        let stored_code: String = database
            .connection()
            .query_row(
                "SELECT code FROM code_lab_attempt_code WHERE attempt_id = ?1",
                [attempt_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(stored_code, "// código propio\n");
        let executions: i64 = database
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM code_lab_executions WHERE attempt_id = ?1",
                [attempt_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(executions, 1);

        let passed = AttemptSummary {
            status: AttemptStatus::Passed,
            duration_ms: 2000,
            tests_passed: 4,
            tests_total: 4,
            compile_ok: Some(true),
            executions: Vec::new(),
        };
        record_attempt(&mut database, id, "solución", &passed).unwrap();
        let solved = exercise_progress(&database, id).unwrap();
        assert_eq!(solved.status, ExerciseStatus::Solved);
        assert_eq!(solved.attempts, 2);
        assert_eq!(solved.best_tests_passed, 4);
        assert!(solved.solved_at.is_some());
    }

    #[test]
    fn seed_prepares_concepts_and_aggregated_stats() {
        let database = seeded();
        let now = crate::infra::catalog::unix_now();
        let concepts = due_concepts(&database, now).unwrap();
        assert!(!concepts.is_empty());
        let stats = code_lab_stats(&database, now).unwrap();
        assert_eq!(stats.lessons, 5);
        assert_eq!(stats.exercises, 6);
        assert_eq!(stats.concepts_due, concepts.len() as i32);
        assert_eq!(stats.attempts, 0);
    }

    #[test]
    fn rating_a_concept_schedules_its_next_review() {
        let mut database = seeded();
        let concept =
            due_concepts(&database, crate::infra::catalog::unix_now()).unwrap()[0].clone();
        let now = crate::infra::catalog::unix_now();
        let outcome = rate_concept(&mut database, concept.id, ReviewRating::Again).unwrap();
        assert_eq!(outcome.interval_days, 1);
        assert!(outcome.due_in_days > 0);

        let updated = database
            .connection()
            .query_row(
                "SELECT interval_days, lapses, due_at FROM code_lab_concept_review WHERE id = ?1",
                [concept.id],
                |row| {
                    Ok((
                        row.get::<_, i32>(0)?,
                        row.get::<_, i32>(1)?,
                        row.get::<_, i64>(2)?,
                    ))
                },
            )
            .unwrap();
        assert_eq!(updated.0, 1);
        assert_eq!(updated.1, 1);
        assert!(updated.2 > now);
        assert!(
            !due_concepts(&database, now)
                .unwrap()
                .iter()
                .any(|item| item.id == concept.id)
        );
    }

    #[test]
    fn learning_items_reflect_progress() {
        let mut database = seeded();
        let lesson = first_lesson(&database);
        open_lesson(&mut database, lesson).unwrap();
        let exercise = list_exercises(&database, "csharp").unwrap()[0].id;
        open_exercise(&mut database, exercise).unwrap();
        let failed = AttemptSummary {
            status: AttemptStatus::Failed,
            duration_ms: 100,
            tests_passed: 0,
            tests_total: 4,
            compile_ok: Some(true),
            executions: Vec::new(),
        };
        record_attempt(&mut database, exercise, "código", &failed).unwrap();

        let items = learning_items(&database).unwrap();
        assert!(
            items
                .iter()
                .any(|item| item.id == lesson && item.kind == ItemType::Lesson)
        );
        assert!(
            items
                .iter()
                .any(|item| item.id == exercise && item.kind == ItemType::Exercise)
        );
    }

    #[test]
    fn reference_solution_requires_an_explicit_read() {
        let database = seeded();
        let id = list_exercises(&database, "csharp").unwrap()[0].id;
        let detail = exercise_detail(&database, id).unwrap().unwrap();
        assert!(detail.starter_code.contains("TODO"));
        let solution = reveal_solution(&database, id).unwrap().unwrap();
        assert!(solution.contains("Console.WriteLine(a + b)"));
    }
}
