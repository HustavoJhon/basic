-- Reversión de la migración 0004_code_lab_practice.
-- Ejecutar solo desde una copia de la base y ajustar `user_version = 3`.

DROP TABLE IF EXISTS code_lab_attempt_code;
DROP TABLE IF EXISTS code_lab_drafts;
