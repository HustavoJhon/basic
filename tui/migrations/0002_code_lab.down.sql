-- Reversión de la migración 0002_code_lab.
-- No toca tablas de F1–F10. Ejecutar solo desde una copia de la base:
--   sqlite3 basic.db < migrations/0002_code_lab.down.sql
-- Después ajustar `PRAGMA user_version = 1`.

DROP TABLE IF EXISTS code_lab_session_links;
DROP TABLE IF EXISTS code_lab_favorites;
DROP TABLE IF EXISTS code_lab_concept_review;
DROP TABLE IF EXISTS code_lab_executions;
DROP TABLE IF EXISTS code_lab_attempts;
DROP TABLE IF EXISTS code_lab_exercise_progress;
DROP TABLE IF EXISTS code_lab_lesson_progress;
DROP TABLE IF EXISTS code_lab_exercises;
DROP TABLE IF EXISTS code_lab_lessons;
DROP TABLE IF EXISTS code_lab_paths;
DROP TABLE IF EXISTS code_lab_languages;
