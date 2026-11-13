use std::fs;
use std::path::Path;
use std::time::Duration;

use rusqlite::{Connection, OpenFlags};

use crate::error::BasicError;

/// Última versión de esquema que entiende este binario.
pub const LATEST_SCHEMA_VERSION: u32 = 1;

/// Tablas que deben existir tras aplicar todas las migraciones.
///
/// No incluye las tablas internas de FTS5 (`documents_fts_*`).
pub const EXPECTED_TABLES: &[&str] = &[
    "documents",
    "tags",
    "document_tags",
    "links",
    "documents_fts",
    "learning_state",
    "document_visits",
    "favorites",
    "review_log",
    "pomodoro_sessions",
    "pomodoro_runtime",
    "events",
    "daily_activity",
    "daily_category_activity",
    "meta",
];

/// Una migración inmutable, embebida en el binario.
#[derive(Debug)]
pub struct Migration {
    pub version: u32,
    pub name: &'static str,
    pub sql: &'static str,
}

/// Migraciones en orden estrictamente creciente.
pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "init",
    sql: include_str!("../../migrations/0001_init.sql"),
}];

/// Conexión SQLite configurada para BASIC.
pub struct Database {
    connection: Connection,
}

impl Database {
    /// Abre (o crea) la base de datos, configura la conexión y aplica las
    /// migraciones pendientes. Crea el directorio padre si hace falta.
    pub fn open(path: &Path) -> Result<Self, BasicError> {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|source| BasicError::CreateDir {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let connection = Connection::open(path)?;
        let mut database = Self { connection };
        database.configure_write(true)?;
        database.migrate()?;
        Ok(database)
    }

    /// Base de datos efímera para pruebas y para validar el esquema.
    pub fn open_in_memory() -> Result<Self, BasicError> {
        let connection = Connection::open_in_memory()?;
        let mut database = Self { connection };
        database.configure_write(false)?;
        database.migrate()?;
        Ok(database)
    }

    /// Abre una base existente sin modificarla. Los `PRAGMA` de conexión
    /// (`foreign_keys`, `busy_timeout`) no escriben en el archivo.
    pub fn open_read_only(path: &Path) -> Result<Self, BasicError> {
        let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        let database = Self { connection };
        database.configure_read()?;
        Ok(database)
    }

    /// Aplica las migraciones pendientes en transacciones individuales.
    ///
    /// Si una migración falla, se revierte completa y `user_version` no avanza.
    pub fn migrate(&mut self) -> Result<(), BasicError> {
        let mut version = self.schema_version()?;
        if version > LATEST_SCHEMA_VERSION {
            return Err(BasicError::DatabaseTooNew {
                found: version,
                supported: LATEST_SCHEMA_VERSION,
            });
        }

        for migration in MIGRATIONS {
            if migration.version <= version {
                continue;
            }
            if migration.version != version + 1 {
                return Err(BasicError::MigrationSequence {
                    expected: version + 1,
                    found: migration.version,
                });
            }

            let transaction = self.connection.transaction()?;
            transaction
                .execute_batch(migration.sql)
                .map_err(|source| BasicError::Migration {
                    version: migration.version,
                    name: migration.name,
                    source,
                })?;
            transaction.pragma_update(None, "user_version", migration.version)?;
            transaction.commit()?;
            version = migration.version;
        }

        Ok(())
    }

    /// Versión de esquema almacenada en `PRAGMA user_version`.
    pub fn schema_version(&self) -> Result<u32, BasicError> {
        Ok(self
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))?)
    }

    /// `true` si la tabla o vista existe.
    pub fn has_table(&self, name: &str) -> Result<bool, BasicError> {
        let count: i64 = self.connection.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type IN ('table', 'view') AND name = ?1",
            [name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Comprueba si SQLite se compiló con FTS5.
    pub fn fts5_compiled(&self) -> Result<bool, BasicError> {
        let used: i64 = self.connection.query_row(
            "SELECT sqlite_compileoption_used('ENABLE_FTS5')",
            [],
            |row| row.get(0),
        )?;
        Ok(used == 1)
    }

    /// Ejecuta una consulta trivial contra el índice full-text.
    pub fn fts_document_count(&self) -> Result<i64, BasicError> {
        Ok(self
            .connection
            .query_row("SELECT count(*) FROM documents_fts", [], |row| row.get(0))?)
    }

    /// `ok` si la comprobación rápida de integridad pasa.
    pub fn quick_check(&self) -> Result<String, BasicError> {
        Ok(self
            .connection
            .query_row("PRAGMA quick_check", [], |row| row.get(0))?)
    }

    /// Número de violaciones de claves foráneas.
    pub fn foreign_key_violations(&self) -> Result<usize, BasicError> {
        let mut statement = self.connection.prepare("PRAGMA foreign_key_check")?;
        let mut rows = statement.query([])?;
        let mut violations = 0;
        while rows.next()?.is_some() {
            violations += 1;
        }
        Ok(violations)
    }

    /// Modo de journal actual (`wal`, `delete`, `memory`...).
    pub fn journal_mode(&self) -> Result<String, BasicError> {
        Ok(self
            .connection
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))?)
    }

    /// Acceso de bajo nivel para los módulos de indexado.
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Acceso mutable para transacciones e indexado.
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    fn configure_write(&self, on_disk: bool) -> Result<(), BasicError> {
        self.connection.busy_timeout(Duration::from_secs(5))?;
        self.connection.pragma_update(None, "foreign_keys", "ON")?;
        self.connection
            .pragma_update(None, "synchronous", "NORMAL")?;
        if on_disk {
            let _: String = self
                .connection
                .query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))?;
        }
        Ok(())
    }

    fn configure_read(&self) -> Result<(), BasicError> {
        self.connection.busy_timeout(Duration::from_secs(5))?;
        self.connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    fn insert_document(database: &Database, path: &str, title: &str) -> i64 {
        database
            .connection()
            .execute(
                "INSERT INTO documents
                 (path, title, category, kind, size_bytes, mtime_ns, indexed_at)
                 VALUES (?1, ?2, 'tests', 'note', 10, 0, 0)",
                [path, title],
            )
            .unwrap();
        database.connection().last_insert_rowid()
    }

    #[test]
    fn migrates_in_memory_and_creates_all_tables() {
        let database = Database::open_in_memory().unwrap();
        assert_eq!(database.schema_version().unwrap(), LATEST_SCHEMA_VERSION);
        for table in EXPECTED_TABLES {
            assert!(database.has_table(table).unwrap(), "falta {table}");
        }
        assert_eq!(database.quick_check().unwrap(), "ok");
        assert_eq!(database.foreign_key_violations().unwrap(), 0);
        assert!(database.fts5_compiled().unwrap());
        assert_eq!(database.fts_document_count().unwrap(), 0);
    }

    #[test]
    fn migration_is_idempotent() {
        let mut database = Database::open_in_memory().unwrap();
        database.migrate().unwrap();
        assert_eq!(database.schema_version().unwrap(), LATEST_SCHEMA_VERSION);
    }

    #[test]
    fn fts5_indexes_and_searches_spanish_text() {
        let database = Database::open_in_memory().unwrap();
        let id = insert_document(&database, "notas/rust.md", "Aprendiendo Rust");
        database
            .connection()
            .execute(
                "INSERT INTO documents_fts (rowid, title, body, path) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![
                    id,
                    "Aprendiendo Rust",
                    "Los iteradores y el préstamo de memoria",
                    "notas/rust.md"
                ],
            )
            .unwrap();

        let matches: i64 = database
            .connection()
            .query_row(
                "SELECT count(*) FROM documents_fts WHERE documents_fts MATCH 'iteradores'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(matches, 1);

        // `remove_diacritics 2` permite buscar sin tilde.
        let matches: i64 = database
            .connection()
            .query_row(
                "SELECT count(*) FROM documents_fts WHERE documents_fts MATCH 'prestamo'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(matches, 1);
    }

    #[test]
    fn deleting_a_document_removes_its_fts_row() {
        let database = Database::open_in_memory().unwrap();
        let id = insert_document(&database, "notas/borrar.md", "Borrar");
        database
            .connection()
            .execute(
                "INSERT INTO documents_fts (rowid, title, body, path) VALUES (?1, 'Borrar', 'texto', 'notas/borrar.md')",
                [id],
            )
            .unwrap();
        assert_eq!(database.fts_document_count().unwrap(), 1);

        database
            .connection()
            .execute("DELETE FROM documents WHERE id = ?1", [id])
            .unwrap();
        assert_eq!(database.fts_document_count().unwrap(), 0);
    }

    #[test]
    fn foreign_keys_are_enforced() {
        let database = Database::open_in_memory().unwrap();
        let error = database
            .connection()
            .execute(
                "INSERT INTO learning_state (document_id, updated_at) VALUES (999, 0)",
                [],
            )
            .unwrap_err();
        assert!(error.to_string().contains("FOREIGN KEY"));
    }

    #[test]
    fn refuses_a_future_schema() {
        let mut database = Database::open_in_memory().unwrap();
        database
            .connection()
            .pragma_update(None, "user_version", 999_u32)
            .unwrap();

        match database.migrate().unwrap_err() {
            BasicError::DatabaseTooNew { found, supported } => {
                assert_eq!(found, 999);
                assert_eq!(supported, LATEST_SCHEMA_VERSION);
            }
            other => panic!("error inesperado: {other:?}"),
        }
    }

    #[test]
    fn creates_parent_directory_and_uses_wal_on_disk() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("nested/basic.db");
        let database = Database::open(&path).unwrap();

        assert!(path.exists());
        assert_eq!(database.journal_mode().unwrap().to_lowercase(), "wal");
        assert_eq!(database.schema_version().unwrap(), LATEST_SCHEMA_VERSION);
    }

    #[test]
    fn read_only_connection_cannot_write() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("basic.db");
        {
            let database = Database::open(&path).unwrap();
            insert_document(&database, "a.md", "A");
        }

        let database = Database::open_read_only(&path).unwrap();
        assert_eq!(database.schema_version().unwrap(), LATEST_SCHEMA_VERSION);
        assert!(database.quick_check().unwrap() == "ok");
        assert!(
            database
                .connection()
                .execute("DELETE FROM documents", [])
                .is_err()
        );
    }
}
