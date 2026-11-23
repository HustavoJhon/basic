-- Enlaces opcionales entre elementos de Code Lab y notas Markdown del vault.
-- La nota es la fuente de verdad; SQLite solo guarda la asociación.

CREATE TABLE IF NOT EXISTS code_lab_note_links (
    language   TEXT NOT NULL,
    item_type  TEXT NOT NULL CHECK (item_type IN ('lesson', 'exercise', 'concept')),
    item_id    INTEGER NOT NULL,
    note_path  TEXT NOT NULL,
    document_id INTEGER REFERENCES documents (id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (language, item_type, item_id)
);

CREATE INDEX IF NOT EXISTS idx_code_lab_note_links_document
    ON code_lab_note_links (document_id);
