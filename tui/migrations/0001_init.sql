-- BASIC TUI — esquema inicial
-- Markdown es la fuente de verdad; esta base de datos es índice y estado complementario.

CREATE TABLE documents (
    id              INTEGER PRIMARY KEY,
    path            TEXT NOT NULL UNIQUE,
    title           TEXT NOT NULL,
    category        TEXT NOT NULL,
    subcategory     TEXT,
    kind            TEXT NOT NULL
                    CHECK (kind IN ('index', 'note', 'project', 'resource', 'journal')),
    size_bytes      INTEGER NOT NULL,
    mtime_ns        INTEGER NOT NULL,
    content_hash    TEXT,
    word_count      INTEGER NOT NULL DEFAULT 0,
    reading_minutes INTEGER NOT NULL DEFAULT 0,
    excerpt         TEXT,
    is_stub         INTEGER NOT NULL DEFAULT 0,
    missing_at      INTEGER,
    indexed_at      INTEGER NOT NULL
);

CREATE INDEX idx_documents_category ON documents (category, subcategory);
CREATE INDEX idx_documents_missing ON documents (missing_at);

CREATE TABLE tags (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE document_tags (
    document_id INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    tag_id      INTEGER NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (document_id, tag_id)
);

CREATE TABLE links (
    id          INTEGER PRIMARY KEY,
    source_id   INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    target_path TEXT NOT NULL,
    target_id   INTEGER REFERENCES documents (id) ON DELETE SET NULL,
    kind        TEXT NOT NULL
                CHECK (kind IN ('markdown', 'wiki', 'image', 'external')),
    text        TEXT,
    line        INTEGER,
    resolved    INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_links_source ON links (source_id);
CREATE INDEX idx_links_target ON links (target_id);

-- Búsqueda full-text. rowid = documents.id
CREATE VIRTUAL TABLE documents_fts USING fts5 (
    title,
    body,
    path UNINDEXED,
    tokenize = 'unicode61 remove_diacritics 2',
    prefix = '2 3 4'
);

-- Al borrar un documento, su fila FTS no debe sobrevivir.
-- La inserción y actualización del índice las controla el indexador (F3).
CREATE TRIGGER documents_fts_delete AFTER DELETE ON documents BEGIN
    DELETE FROM documents_fts WHERE rowid = old.id;
END;

CREATE TABLE learning_state (
    document_id    INTEGER PRIMARY KEY REFERENCES documents (id) ON DELETE CASCADE,
    status         TEXT NOT NULL DEFAULT 'pending'
                   CHECK (status IN ('pending', 'in_progress', 'learned', 'needs_review')),
    started_at     INTEGER,
    completed_at   INTEGER,
    last_opened_at INTEGER,
    review_due_at  INTEGER,
    interval_days  INTEGER NOT NULL DEFAULT 0,
    ease_factor    REAL NOT NULL DEFAULT 2.5,
    review_count   INTEGER NOT NULL DEFAULT 0,
    updated_at     INTEGER NOT NULL
);

CREATE INDEX idx_learning_due ON learning_state (review_due_at)
    WHERE review_due_at IS NOT NULL;

CREATE TABLE document_visits (
    id          INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    visited_at  INTEGER NOT NULL,
    source      TEXT NOT NULL DEFAULT 'tui'
);

CREATE INDEX idx_visits_doc ON document_visits (document_id, visited_at DESC);

CREATE TABLE favorites (
    document_id INTEGER PRIMARY KEY REFERENCES documents (id) ON DELETE CASCADE,
    created_at  INTEGER NOT NULL
);

CREATE TABLE review_log (
    id                INTEGER PRIMARY KEY,
    document_id       INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    reviewed_at       INTEGER NOT NULL,
    rating            INTEGER NOT NULL CHECK (rating BETWEEN 0 AND 3),
    previous_interval INTEGER NOT NULL,
    next_interval     INTEGER NOT NULL,
    next_due_at       INTEGER NOT NULL
);

CREATE TABLE pomodoro_sessions (
    id              INTEGER PRIMARY KEY,
    phase           TEXT NOT NULL
                    CHECK (phase IN ('focus', 'short_break', 'long_break')),
    planned_seconds INTEGER NOT NULL,
    actual_seconds  INTEGER,
    started_at      INTEGER NOT NULL,
    ended_at        INTEGER,
    status          TEXT NOT NULL
                    CHECK (status IN ('running', 'paused', 'completed', 'cancelled', 'interrupted')),
    cycle_position  INTEGER NOT NULL DEFAULT 1,
    document_id     INTEGER REFERENCES documents (id) ON DELETE SET NULL,
    category        TEXT,
    note            TEXT
);

CREATE INDEX idx_pomodoro_started ON pomodoro_sessions (started_at);

-- Snapshot para recuperar el estado si la aplicación se cierra inesperadamente.
CREATE TABLE pomodoro_runtime (
    id                        INTEGER PRIMARY KEY CHECK (id = 1),
    session_id                INTEGER NOT NULL REFERENCES pomodoro_sessions (id) ON DELETE CASCADE,
    phase                     TEXT NOT NULL,
    cycle_position            INTEGER NOT NULL,
    started_at                INTEGER NOT NULL,
    last_heartbeat_at         INTEGER NOT NULL,
    paused_at                 INTEGER,
    accumulated_pause_seconds INTEGER NOT NULL DEFAULT 0,
    planned_seconds           INTEGER NOT NULL,
    document_id               INTEGER REFERENCES documents (id) ON DELETE SET NULL,
    category                  TEXT
);

CREATE TABLE events (
    id               INTEGER PRIMARY KEY,
    occurred_at      INTEGER NOT NULL,
    kind             TEXT NOT NULL,
    document_id      INTEGER REFERENCES documents (id) ON DELETE SET NULL,
    category         TEXT,
    duration_seconds INTEGER,
    metadata_json    TEXT
);

CREATE INDEX idx_events_day ON events (occurred_at);

CREATE TABLE daily_activity (
    day                 TEXT PRIMARY KEY,
    study_seconds       INTEGER NOT NULL DEFAULT 0,
    pomodoros_completed INTEGER NOT NULL DEFAULT 0,
    docs_created        INTEGER NOT NULL DEFAULT 0,
    docs_modified       INTEGER NOT NULL DEFAULT 0,
    docs_opened         INTEGER NOT NULL DEFAULT 0,
    topics_studied      INTEGER NOT NULL DEFAULT 0,
    updated_at          INTEGER NOT NULL
);

CREATE TABLE daily_category_activity (
    day               TEXT NOT NULL,
    category          TEXT NOT NULL,
    study_seconds     INTEGER NOT NULL DEFAULT 0,
    documents_touched INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (day, category)
);

CREATE TABLE meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
