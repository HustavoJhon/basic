//! Code Lab: aprendizaje y práctica de programación dentro de BASIC.
//!
//! Este módulo se construye por capas. La capa de dominio no depende de
//! SQLite, Ratatui ni de procesos externos; las siguientes fases añadirán
//! catálogo, editor, runners y UI sin acoplarlos entre sí.

pub mod app;
pub mod catalog;
pub mod domain;
pub mod infra;
pub mod runners;
pub mod ui;
