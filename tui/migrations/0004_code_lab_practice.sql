-- BASIC Code Lab — esquema v4
-- Persistencia de borradores y código por intento. No altera v1, v2 ni v3.

CREATE TABLE IF NOT EXISTS code_lab_drafts (
    language    TEXT NOT NULL REFERENCES code_lab_languages (code) ON DELETE CASCADE,
    exercise_id INTEGER NOT NULL UNIQUE
                REFERENCES code_lab_exercises (id) ON DELETE CASCADE,
    code        TEXT NOT NULL DEFAULT '',
    updated_at  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS code_lab_attempt_code (
    attempt_id INTEGER PRIMARY KEY
               REFERENCES code_lab_attempts (id) ON DELETE CASCADE,
    code       TEXT NOT NULL DEFAULT ''
);
