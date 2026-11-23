-- BASIC Code Lab — esquema v2
-- Compatible con bases v1: solo añade tablas nuevas y no reescribe datos.
-- Markdown sigue siendo la fuente de verdad de apuntes; aquí viven catálogo,
-- progreso, intentos y estadísticas de práctica.

CREATE TABLE IF NOT EXISTS code_lab_languages (
    code             TEXT PRIMARY KEY,
    name             TEXT NOT NULL,
    sdk_command      TEXT,
    enabled          INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    detected_version TEXT,
    detected_at      INTEGER
);

CREATE TABLE IF NOT EXISTS code_lab_paths (
    id          INTEGER PRIMARY KEY,
    language    TEXT NOT NULL REFERENCES code_lab_languages (code) ON DELETE CASCADE,
    slug        TEXT NOT NULL,
    title       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    level       INTEGER NOT NULL DEFAULT 1,
    UNIQUE (language, slug)
);

CREATE TABLE IF NOT EXISTS code_lab_lessons (
    id                INTEGER PRIMARY KEY,
    path_id           INTEGER NOT NULL REFERENCES code_lab_paths (id) ON DELETE CASCADE,
    slug              TEXT NOT NULL,
    title             TEXT NOT NULL,
    summary           TEXT NOT NULL DEFAULT '',
    content_md        TEXT NOT NULL DEFAULT '',
    concepts_json     TEXT NOT NULL DEFAULT '[]',
    order_index       INTEGER NOT NULL DEFAULT 0,
    estimated_minutes INTEGER NOT NULL DEFAULT 10,
    created_at        INTEGER NOT NULL,
    UNIQUE (path_id, slug)
);

CREATE INDEX IF NOT EXISTS idx_code_lab_lessons_order
    ON code_lab_lessons (path_id, order_index);

CREATE TABLE IF NOT EXISTS code_lab_exercises (
    id                INTEGER PRIMARY KEY,
    language          TEXT NOT NULL REFERENCES code_lab_languages (code),
    path_id           INTEGER REFERENCES code_lab_paths (id) ON DELETE SET NULL,
    lesson_id         INTEGER REFERENCES code_lab_lessons (id) ON DELETE SET NULL,
    slug              TEXT NOT NULL,
    title             TEXT NOT NULL,
    difficulty        TEXT NOT NULL CHECK (difficulty IN ('easy', 'medium', 'hard')),
    topic             TEXT NOT NULL DEFAULT '',
    level             INTEGER NOT NULL DEFAULT 1,
    concepts_json     TEXT NOT NULL DEFAULT '[]',
    description_md    TEXT NOT NULL DEFAULT '',
    examples_json     TEXT NOT NULL DEFAULT '[]',
    constraints_json  TEXT NOT NULL DEFAULT '[]',
    hints_json        TEXT NOT NULL DEFAULT '[]',
    starter_code      TEXT NOT NULL DEFAULT '',
    -- La solución de referencia se guarda aquí pero ninguna consulta de UI la
    -- devuelve hasta que el usuario pide explícitamente verla.
    reference_solution TEXT NOT NULL DEFAULT '',
    test_spec_json    TEXT NOT NULL DEFAULT '{}',
    created_at        INTEGER NOT NULL,
    UNIQUE (language, slug)
);

CREATE INDEX IF NOT EXISTS idx_code_lab_exercises_filter
    ON code_lab_exercises (language, difficulty, level);
CREATE INDEX IF NOT EXISTS idx_code_lab_exercises_topic
    ON code_lab_exercises (language, topic);

CREATE TABLE IF NOT EXISTS code_lab_lesson_progress (
    lesson_id      INTEGER PRIMARY KEY REFERENCES code_lab_lessons (id) ON DELETE CASCADE,
    status         TEXT NOT NULL DEFAULT 'not_started'
                   CHECK (status IN ('not_started', 'in_progress', 'completed', 'needs_review')),
    last_opened_at INTEGER,
    completed_at   INTEGER,
    quiz_correct   INTEGER NOT NULL DEFAULT 0,
    quiz_total     INTEGER NOT NULL DEFAULT 0,
    seconds_spent  INTEGER NOT NULL DEFAULT 0,
    review_due_at  INTEGER,
    updated_at     INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS code_lab_exercise_progress (
    exercise_id      INTEGER PRIMARY KEY REFERENCES code_lab_exercises (id) ON DELETE CASCADE,
    status           TEXT NOT NULL DEFAULT 'not_started'
                     CHECK (status IN ('not_started', 'attempted', 'solved', 'needs_review')),
    attempts         INTEGER NOT NULL DEFAULT 0,
    solved_at        INTEGER,
    last_opened_at   INTEGER,
    best_tests_passed INTEGER NOT NULL DEFAULT 0,
    tests_total      INTEGER NOT NULL DEFAULT 0,
    best_score       REAL NOT NULL DEFAULT 0,
    seconds_spent    INTEGER NOT NULL DEFAULT 0,
    last_attempt_id  INTEGER,
    updated_at       INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS code_lab_attempts (
    id               INTEGER PRIMARY KEY,
    exercise_id      INTEGER NOT NULL REFERENCES code_lab_exercises (id) ON DELETE CASCADE,
    started_at       INTEGER NOT NULL,
    finished_at      INTEGER,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    status           TEXT NOT NULL CHECK (status IN ('running', 'passed', 'failed', 'cancelled')),
    tests_passed     INTEGER NOT NULL DEFAULT 0,
    tests_total      INTEGER NOT NULL DEFAULT 0,
    compile_ok       INTEGER,
    language         TEXT NOT NULL,
    created_at       INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_code_lab_attempts_exercise
    ON code_lab_attempts (exercise_id, started_at DESC);

CREATE TABLE IF NOT EXISTS code_lab_executions (
    id         INTEGER PRIMARY KEY,
    attempt_id INTEGER NOT NULL REFERENCES code_lab_attempts (id) ON DELETE CASCADE,
    phase      TEXT NOT NULL CHECK (phase IN ('detect', 'restore', 'build', 'run', 'test')),
    command    TEXT NOT NULL DEFAULT '',
    exit_code  INTEGER,
    stdout     TEXT NOT NULL DEFAULT '',
    stderr     TEXT NOT NULL DEFAULT '',
    duration_ms INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_code_lab_executions_attempt
    ON code_lab_executions (attempt_id, id);

CREATE TABLE IF NOT EXISTS code_lab_concept_review (
    id              INTEGER PRIMARY KEY,
    language        TEXT NOT NULL,
    concept         TEXT NOT NULL,
    level           INTEGER NOT NULL DEFAULT 0,
    interval_days   INTEGER NOT NULL DEFAULT 0,
    ease_factor     REAL NOT NULL DEFAULT 2.5,
    due_at          INTEGER,
    lapses          INTEGER NOT NULL DEFAULT 0,
    last_reviewed_at INTEGER,
    UNIQUE (language, concept)
);

CREATE TABLE IF NOT EXISTS code_lab_favorites (
    language   TEXT NOT NULL,
    item_type  TEXT NOT NULL CHECK (item_type IN ('lesson', 'exercise')),
    item_id    INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (language, item_type, item_id)
);

CREATE TABLE IF NOT EXISTS code_lab_session_links (
    id          INTEGER PRIMARY KEY,
    session_id  INTEGER NOT NULL UNIQUE
                 REFERENCES pomodoro_sessions (id) ON DELETE CASCADE,
    language    TEXT,
    topic       TEXT,
    exercise_id INTEGER REFERENCES code_lab_exercises (id) ON DELETE SET NULL,
    lesson_id   INTEGER REFERENCES code_lab_lessons (id) ON DELETE SET NULL
);
