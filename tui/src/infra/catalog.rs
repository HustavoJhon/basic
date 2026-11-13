//! Consultas y mutaciones del catálogo.
//!
//! SQLite es índice y estado complementario; las notas siguen viviendo en
//! Markdown y este módulo nunca reescribe archivos del vault.

use std::collections::HashSet;

use chrono::{Duration, Local, NaiveDate};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::domain::{
    ActivityDay, DashboardStats, Document, LearnMode, LearningStatus, ReviewOutcome, SearchScope,
    schedule_review,
};
use crate::error::BasicError;
use crate::infra::sqlite::Database;

const DOCUMENT_SELECT: &str = "
SELECT d.id, d.path, d.title, d.category, d.subcategory, d.kind,
       d.size_bytes, d.word_count, d.reading_minutes, d.excerpt,
       d.is_stub, d.missing_at, d.indexed_at,
       COALESCE(ls.status, 'pending'), ls.last_opened_at, ls.review_due_at,
       (f.document_id IS NOT NULL)
FROM documents d
LEFT JOIN learning_state ls ON ls.document_id = d.id
LEFT JOIN favorites f ON f.document_id = d.id";

fn document_from_row(row: &Row<'_>) -> rusqlite::Result<Document> {
    Ok(Document {
        id: row.get(0)?,
        path: row.get(1)?,
        title: row.get(2)?,
        category: row.get(3)?,
        subcategory: row.get(4)?,
        kind: row.get(5)?,
        size_bytes: row.get(6)?,
        word_count: row.get(7)?,
        reading_minutes: row.get(8)?,
        excerpt: row.get::<_, Option<String>>(9)?.unwrap_or_default(),
        is_stub: row.get::<_, i64>(10)? != 0,
        missing_at: row.get(11)?,
        indexed_at: row.get(12)?,
        status: LearningStatus::parse(&row.get::<_, String>(13)?),
        last_opened_at: row.get(14)?,
        review_due_at: row.get(15)?,
        favorite: row.get::<_, i64>(16)? != 0,
    })
}

/// Lista documentos vivos, opcionalmente limitada.
pub fn list_documents(
    database: &Database,
    limit: Option<usize>,
) -> Result<Vec<Document>, BasicError> {
    let limit = limit.unwrap_or(10_000);
    let sql = format!(
        "{DOCUMENT_SELECT}
         WHERE d.missing_at IS NULL
         ORDER BY d.category, d.title
         LIMIT ?1"
    );
    let mut statement = database.connection().prepare(&sql)?;
    let rows = statement.query_map([limit as i64], document_from_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

/// Obtiene un documento por id.
pub fn get_document(database: &Database, id: i64) -> Result<Option<Document>, BasicError> {
    let sql = format!("{DOCUMENT_SELECT} WHERE d.id = ?1");
    Ok(database
        .connection()
        .query_row(&sql, [id], document_from_row)
        .optional()?)
}

/// Documentos abiertos recientemente.
pub fn recent_documents(database: &Database, limit: usize) -> Result<Vec<Document>, BasicError> {
    let sql = format!(
        "{DOCUMENT_SELECT}
         WHERE d.missing_at IS NULL AND ls.last_opened_at IS NOT NULL
         ORDER BY ls.last_opened_at DESC
         LIMIT ?1"
    );
    let mut statement = database.connection().prepare(&sql)?;
    let rows = statement.query_map([limit as i64], document_from_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

/// Resumen para el dashboard.
pub fn dashboard_stats(database: &Database) -> Result<DashboardStats, BasicError> {
    let connection = database.connection();
    let total: i64 = connection.query_row(
        "SELECT count(*) FROM documents WHERE missing_at IS NULL",
        [],
        |row| row.get(0),
    )?;
    let pending: i64 = connection.query_row(
        "SELECT count(*) FROM documents d
         LEFT JOIN learning_state ls ON ls.document_id = d.id
         WHERE d.missing_at IS NULL AND COALESCE(ls.status, 'pending') = 'pending'",
        [],
        |row| row.get(0),
    )?;
    let in_progress: i64 = connection.query_row(
        "SELECT count(*) FROM documents d
         JOIN learning_state ls ON ls.document_id = d.id
         WHERE d.missing_at IS NULL AND ls.status = 'in_progress'",
        [],
        |row| row.get(0),
    )?;
    let due: i64 = connection.query_row(
        "SELECT count(*) FROM learning_state ls
         JOIN documents d ON d.id = ls.document_id
         WHERE d.missing_at IS NULL AND ls.review_due_at IS NOT NULL AND ls.review_due_at <= ?1",
        [unix_now()],
        |row| row.get(0),
    )?;
    let stubs: i64 = connection.query_row(
        "SELECT count(*) FROM documents WHERE missing_at IS NULL AND is_stub = 1",
        [],
        |row| row.get(0),
    )?;
    let (study_seconds_today, pomodoros_today): (i64, i64) = connection
        .query_row(
            "SELECT study_seconds, pomodoros_completed FROM daily_activity WHERE day = ?1",
            [today()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?
        .unwrap_or((0, 0));

    let mut categories_statement = connection.prepare(
        "SELECT category, count(*) FROM documents
         WHERE missing_at IS NULL
         GROUP BY category
         ORDER BY count(*) DESC, category
         LIMIT 8",
    )?;
    let categories = categories_statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DashboardStats {
        total,
        pending,
        in_progress,
        due,
        stubs,
        study_seconds_today,
        pomodoros_today,
        streak_days: streak_days(connection)?,
        categories,
    })
}

fn streak_days(connection: &Connection) -> Result<i64, BasicError> {
    let mut statement = connection.prepare(
        "SELECT day FROM daily_activity WHERE study_seconds > 0 OR pomodoros_completed > 0",
    )?;
    let days = statement
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<HashSet<_>, _>>()?;
    let today = Local::now().date_naive();
    let mut streak = 0;
    for offset in 0..3650 {
        let day = today - Duration::days(offset);
        let key = day.format("%Y-%m-%d").to_string();
        if days.contains(&key) {
            streak += 1;
        } else if offset > 0 {
            break;
        }
    }
    Ok(streak)
}

/// Serie diaria para el heatmap; rellena los huecos con cero.
pub fn activity_days(database: &Database, days: u32) -> Result<Vec<ActivityDay>, BasicError> {
    let mut statement = database
        .connection()
        .prepare("SELECT day, study_seconds FROM daily_activity")?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    let map: std::collections::HashMap<_, _> = rows.into_iter().collect();
    let today = Local::now().date_naive();
    let mut result = Vec::with_capacity(days as usize);
    for offset in (0..days).rev() {
        let day = today - Duration::days(offset as i64);
        let key = day.format("%Y-%m-%d").to_string();
        result.push(ActivityDay {
            study_seconds: map.get(&key).copied().unwrap_or(0),
            day: key,
        });
    }
    Ok(result)
}

/// Búsqueda combinable por título, contenido, ruta, categoría o código.
pub fn search_documents(
    database: &Database,
    query: &str,
    scope: SearchScope,
) -> Result<Vec<Document>, BasicError> {
    let query = query.trim();
    if query.is_empty() {
        return list_documents(database, Some(200));
    }

    let mut results = Vec::new();
    let mut seen = HashSet::new();

    if matches!(
        scope,
        SearchScope::Content | SearchScope::Code | SearchScope::All
    ) {
        for document in search_content(database, query, scope == SearchScope::Code)? {
            if seen.insert(document.id) {
                results.push(document);
            }
        }
    }

    if matches!(
        scope,
        SearchScope::Title | SearchScope::Path | SearchScope::Category | SearchScope::All
    ) {
        let matcher = SkimMatcherV2::default();
        let mut fuzzy = Vec::new();
        for document in list_documents(database, None)? {
            let haystack = match scope {
                SearchScope::Title => document.title.clone(),
                SearchScope::Path => document.path.clone(),
                SearchScope::Category => document.category.clone(),
                SearchScope::All => {
                    format!("{} {} {}", document.title, document.path, document.category)
                }
                _ => continue,
            };
            if let Some(score) = matcher.fuzzy_match(&haystack, query) {
                fuzzy.push((score, document));
            }
        }
        // Los resultados de FTS ya vienen ordenados por relevancia; los
        // difusos se ordenan por puntuación para que el título exacto gane.
        fuzzy.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| a.1.title.to_lowercase().cmp(&b.1.title.to_lowercase()))
        });
        for (_, document) in fuzzy {
            if seen.insert(document.id) {
                results.push(document);
            }
        }
    }

    results.truncate(200);
    Ok(results)
}

fn search_content(
    database: &Database,
    query: &str,
    only_code: bool,
) -> Result<Vec<Document>, BasicError> {
    let fts_query = fts_query(query);
    if fts_query.is_empty() {
        return Ok(Vec::new());
    }
    let code_filter = if only_code {
        " AND documents_fts.body LIKE '%```%'"
    } else {
        ""
    };
    let sql = format!(
        "{DOCUMENT_SELECT}
         JOIN documents_fts ON documents_fts.rowid = d.id
         WHERE documents_fts MATCH ?1 AND d.missing_at IS NULL{code_filter}
         ORDER BY bm25(documents_fts)
         LIMIT 200"
    );
    let mut statement = database.connection().prepare(&sql)?;
    let rows = statement.query_map([fts_query], document_from_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn fts_query(query: &str) -> String {
    query
        .split_whitespace()
        .filter(|term| !term.is_empty())
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" AND ")
}

/// Documentos según el modo de Learning Mode.
pub fn learn_documents(database: &Database, mode: LearnMode) -> Result<Vec<Document>, BasicError> {
    let now = unix_now();
    let (where_clause, order) = match mode {
        LearnMode::Continue => (
            "d.missing_at IS NULL AND COALESCE(ls.status, 'pending') IN ('in_progress', 'needs_review')",
            "ls.last_opened_at DESC, d.title",
        ),
        LearnMode::Explore => ("d.missing_at IS NULL", "d.category, d.title"),
        LearnMode::Pending => (
            "d.missing_at IS NULL AND (COALESCE(ls.status, 'pending') = 'pending' OR d.is_stub = 1)",
            "d.is_stub DESC, d.category, d.title",
        ),
        LearnMode::Review => (
            "d.missing_at IS NULL AND ls.review_due_at IS NOT NULL AND ls.review_due_at <= ?1",
            "ls.review_due_at, d.title",
        ),
        LearnMode::Random => ("d.missing_at IS NULL AND d.is_stub = 0", "d.title"),
    };
    let sql = format!("{DOCUMENT_SELECT} WHERE {where_clause} ORDER BY {order}");
    let mut statement = database.connection().prepare(&sql)?;
    let rows = if mode == LearnMode::Review {
        statement
            .query_map([now], document_from_row)?
            .collect::<Result<Vec<_>, _>>()?
    } else {
        statement
            .query_map([], document_from_row)?
            .collect::<Result<Vec<_>, _>>()?
    };
    Ok(rows)
}

/// Documentos con repaso pendiente.
pub fn due_documents(database: &Database) -> Result<Vec<Document>, BasicError> {
    learn_documents(database, LearnMode::Review)
}

/// Registra una apertura sin pintar actividad de estudio.
pub fn record_visit(database: &Database, document_id: i64) -> Result<(), BasicError> {
    let now = unix_now();
    let connection = database.connection();
    connection.execute(
        "INSERT INTO document_visits (document_id, visited_at, source) VALUES (?1, ?2, 'tui')",
        params![document_id, now],
    )?;
    connection.execute(
        "UPDATE learning_state
         SET last_opened_at = ?1,
             started_at = COALESCE(started_at, ?1),
             status = CASE WHEN status = 'pending' THEN 'in_progress' ELSE status END,
             updated_at = ?1
         WHERE document_id = ?2",
        params![now, document_id],
    )?;
    upsert_activity(connection, &today(), "docs_opened = docs_opened + 1")?;
    Ok(())
}

/// Alterna el favorito y devuelve su estado final.
pub fn toggle_favorite(database: &Database, document_id: i64) -> Result<bool, BasicError> {
    let now = unix_now();
    let connection = database.connection();
    let exists: bool = connection
        .query_row(
            "SELECT 1 FROM favorites WHERE document_id = ?1",
            [document_id],
            |_| Ok(true),
        )
        .optional()?
        .unwrap_or(false);
    if exists {
        connection.execute(
            "DELETE FROM favorites WHERE document_id = ?1",
            [document_id],
        )?;
        Ok(false)
    } else {
        connection.execute(
            "INSERT INTO favorites (document_id, created_at) VALUES (?1, ?2)",
            params![document_id, now],
        )?;
        Ok(true)
    }
}

/// Cambia explícitamente el estado de aprendizaje.
pub fn set_status(
    database: &Database,
    document_id: i64,
    status: LearningStatus,
) -> Result<(), BasicError> {
    let now = unix_now();
    let completed_at = (status == LearningStatus::Learned).then_some(now);
    database.connection().execute(
        "UPDATE learning_state
         SET status = ?1,
             started_at = COALESCE(started_at, ?2),
             completed_at = ?3,
             updated_at = ?2
         WHERE document_id = ?4",
        params![status.as_str(), now, completed_at, document_id],
    )?;
    Ok(())
}

/// Aplica una calificación de repaso y registra el historial.
pub fn record_review(
    database: &Database,
    document_id: i64,
    rating: crate::domain::ReviewRating,
) -> Result<ReviewOutcome, BasicError> {
    let now = unix_now();
    let connection = database.connection();
    let (previous_interval, previous_ease): (i64, f64) = connection
        .query_row(
            "SELECT interval_days, ease_factor FROM learning_state WHERE document_id = ?1",
            [document_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?
        .unwrap_or((0, 2.5));
    let outcome = schedule_review(rating, previous_interval, previous_ease);
    let next_due = now + outcome.due_in_days * 86_400;

    connection.execute(
        "UPDATE learning_state
         SET status = CASE WHEN ?1 >= 2 THEN 'learned' ELSE 'needs_review' END,
             review_due_at = ?2,
             interval_days = ?3,
             ease_factor = ?4,
             review_count = review_count + 1,
             updated_at = ?5
         WHERE document_id = ?6",
        params![
            rating.value(),
            next_due,
            outcome.interval_days,
            outcome.ease_factor,
            now,
            document_id
        ],
    )?;
    connection.execute(
        "INSERT INTO review_log
         (document_id, reviewed_at, rating, previous_interval, next_interval, next_due_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            document_id,
            now,
            rating.value(),
            previous_interval,
            outcome.interval_days,
            next_due
        ],
    )?;
    upsert_activity(connection, &today(), "topics_studied = topics_studied + 1")?;
    Ok(outcome)
}

/// Registra una sesión de Pomodoro completada y su actividad.
#[allow(clippy::too_many_arguments)]
pub fn start_pomodoro(
    database: &Database,
    phase: &str,
    planned_seconds: i64,
    cycle_position: i64,
    document_id: Option<i64>,
    category: Option<&str>,
) -> Result<i64, BasicError> {
    let now = unix_now();
    let connection = database.connection();
    connection.execute(
        "INSERT INTO pomodoro_sessions
         (phase, planned_seconds, started_at, status, cycle_position, document_id, category)
         VALUES (?1, ?2, ?3, 'running', ?4, ?5, ?6)",
        params![
            phase,
            planned_seconds,
            now,
            cycle_position,
            document_id,
            category
        ],
    )?;
    let session_id = connection.last_insert_rowid();
    connection.execute(
        "INSERT OR REPLACE INTO pomodoro_runtime
         (id, session_id, phase, cycle_position, started_at, last_heartbeat_at,
          accumulated_pause_seconds, planned_seconds, document_id, category)
         VALUES (1, ?1, ?2, ?3, ?4, ?4, 0, ?5, ?6, ?7)",
        params![
            session_id,
            phase,
            cycle_position,
            now,
            planned_seconds,
            document_id,
            category
        ],
    )?;
    Ok(session_id)
}

/// Cierra una sesión de Pomodoro.
pub fn finish_pomodoro(
    database: &Database,
    session_id: i64,
    actual_seconds: i64,
    status: &str,
    phase: &str,
    document_id: Option<i64>,
    category: Option<&str>,
) -> Result<(), BasicError> {
    let now = unix_now();
    let connection = database.connection();
    connection.execute(
        "UPDATE pomodoro_sessions
         SET actual_seconds = ?1, ended_at = ?2, status = ?3
         WHERE id = ?4",
        params![actual_seconds, now, status, session_id],
    )?;
    connection.execute(
        "DELETE FROM pomodoro_runtime WHERE session_id = ?1",
        [session_id],
    )?;

    eprintln!("finish: session={session_id} actual={actual_seconds} status={status} phase={phase}");
    if status == "completed" && phase == "focus" {
        let day = today();
        upsert_activity(
            connection,
            &day,
            &format!(
                "study_seconds = study_seconds + {actual_seconds}, \
                 pomodoros_completed = pomodoros_completed + 1"
            ),
        )?;
        if let Some(category) = category {
            connection.execute(
                "INSERT INTO daily_category_activity
                 (day, category, study_seconds, documents_touched)
                 VALUES (?1, ?2, ?3, 1)
                 ON CONFLICT(day, category) DO UPDATE SET
                     study_seconds = study_seconds + excluded.study_seconds,
                     documents_touched = documents_touched + 1",
                params![day, category, actual_seconds],
            )?;
        }
        connection.execute(
            "INSERT INTO events (occurred_at, kind, document_id, category, duration_seconds)
             VALUES (?1, 'pomodoro_completed', ?2, ?3, ?4)",
            params![now, document_id, category, actual_seconds],
        )?;
    }
    Ok(())
}

/// Sesión de Pomodoro recuperable tras un cierre inesperado.
pub type PomodoroRuntime = (i64, String, i64, i64, i64, Option<i64>, Option<String>);

pub fn pomodoro_runtime(database: &Database) -> Result<Option<PomodoroRuntime>, BasicError> {
    Ok(database
        .connection()
        .query_row(
            "SELECT session_id, phase, cycle_position, started_at, planned_seconds,
                    document_id, category
             FROM pomodoro_runtime WHERE id = 1",
            [],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )
        .optional()?)
}

fn upsert_activity(connection: &Connection, day: &str, update: &str) -> Result<(), BasicError> {
    // Garantizamos que la fila existe antes de actualizar, para que las
    // expresiones acumulativas (col = col + N) tengan siempre un valor base.
    let now = unix_now();
    connection.execute(
        "INSERT OR IGNORE INTO daily_activity (day, updated_at) VALUES (?1, ?2)",
        params![day, now],
    )?;
    let sql = format!("UPDATE daily_activity SET {update}, updated_at = ?2 WHERE day = ?1");
    connection.execute(&sql, params![day, now])?;
    Ok(())
}

pub fn today() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

pub fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

/// Fecha local legible a partir de un timestamp Unix.
pub fn format_date(timestamp: i64) -> String {
    chrono::DateTime::from_timestamp(timestamp, 0)
        .map(|datetime| {
            datetime
                .with_timezone(&Local)
                .format("%Y-%m-%d")
                .to_string()
        })
        .unwrap_or_else(|| "-".to_string())
}

/// Número de días que faltan para un repaso (negativo si está vencido).
pub fn days_until(timestamp: i64) -> i64 {
    let now = Local::now().date_naive();
    let date = chrono::DateTime::from_timestamp(timestamp, 0)
        .map(|datetime| datetime.with_timezone(&Local).date_naive())
        .unwrap_or(now);
    (date - now).num_days()
}

/// Fecha de hoy como `NaiveDate`, útil para tests y formato.
pub fn today_date() -> NaiveDate {
    Local::now().date_naive()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::infra::index::index_vault;
    use crate::infra::sqlite::Database;
    use crate::paths::Paths;
    use std::fs;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, Config, Database) {
        let temp = TempDir::new().unwrap();
        let vault = temp.path().join("vault");
        fs::create_dir_all(vault.join("linux")).unwrap();
        fs::write(vault.join("README.md"), "# Vault\n\nNota raíz.\n").unwrap();
        fs::write(
            vault.join("linux/vim.md"),
            "# Atajos de Vim\n\n`dd` y `yy`. #vim\n",
        )
        .unwrap();
        let paths = Paths::from_dirs(
            temp.path(),
            &temp.path().join("config"),
            &temp.path().join("data"),
            &temp.path().join("state"),
            |_| None,
        );
        let mut config = Config::default_for(&paths);
        config.vault = vault;
        let mut database = Database::open_in_memory().unwrap();
        index_vault(&mut database, &config).unwrap();
        (temp, config, database)
    }

    #[test]
    fn lists_and_searches_documents() {
        let (_temp, _config, database) = fixture();
        let documents = list_documents(&database, None).unwrap();
        assert_eq!(documents.len(), 2);
        let hits = search_documents(&database, "vim", SearchScope::All).unwrap();
        assert!(hits.iter().any(|document| document.title.contains("Vim")));
        let hits = search_documents(&database, "dd", SearchScope::Content).unwrap();
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn records_visits_and_reviews() {
        let (_temp, _config, database) = fixture();
        let document = &list_documents(&database, None).unwrap()[0];
        record_visit(&database, document.id).unwrap();
        let document = get_document(&database, document.id).unwrap().unwrap();
        assert_eq!(document.status, LearningStatus::InProgress);
        assert!(document.last_opened_at.is_some());

        let outcome =
            record_review(&database, document.id, crate::domain::ReviewRating::Good).unwrap();
        assert_eq!(outcome.interval_days, 1);
        let document = get_document(&database, document.id).unwrap().unwrap();
        assert_eq!(document.status, LearningStatus::Learned);
        assert!(document.review_due_at.is_some());
    }

    #[test]
    fn dashboard_and_activity_are_consistent() {
        let (_temp, _config, database) = fixture();
        let stats = dashboard_stats(&database).unwrap();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.categories.len(), 2);
        let days = activity_days(&database, 7).unwrap();
        assert_eq!(days.len(), 7);
        assert_eq!(days.last().unwrap().day, today());
    }
}
