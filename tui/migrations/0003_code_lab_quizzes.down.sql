-- Reversión de la migración 0003_code_lab_quizzes.
-- Ejecutar solo desde una copia de la base y ajustar `user_version = 2`.

DROP TABLE IF EXISTS code_lab_quiz_questions;
