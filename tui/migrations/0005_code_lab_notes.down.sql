-- Reversión de la migración 0005_code_lab_notes.
-- Hacer una copia de seguridad antes de aplicarla y ajustar `user_version` a 4.

DROP TABLE IF EXISTS code_lab_note_links;
