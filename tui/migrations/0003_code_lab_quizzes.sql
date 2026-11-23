-- BASIC Code Lab — esquema v3
-- Añade cuestionarios por lección sin tocar v1 ni v2.

CREATE TABLE IF NOT EXISTS code_lab_quiz_questions (
    id            INTEGER PRIMARY KEY,
    lesson_id     INTEGER NOT NULL
                  REFERENCES code_lab_lessons (id) ON DELETE CASCADE,
    prompt        TEXT NOT NULL,
    options_json  TEXT NOT NULL,
    correct_index INTEGER NOT NULL CHECK (correct_index >= 0),
    explanation   TEXT NOT NULL DEFAULT '',
    order_index   INTEGER NOT NULL DEFAULT 0,
    UNIQUE (lesson_id, order_index)
);

CREATE INDEX IF NOT EXISTS idx_code_lab_quiz_lesson
    ON code_lab_quiz_questions (lesson_id, order_index);
