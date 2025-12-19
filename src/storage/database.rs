//! SQLite database for persistent storage

#![allow(dead_code)]

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

/// Database wrapper for SQLite
pub struct Database {
    /// SQLite connection
    conn: Connection,
}

impl Database {
    /// Open or create the database
    pub fn open() -> Result<Self> {
        let path = Self::database_path()?;

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;

        let db = Self { conn };
        db.initialize()?;

        Ok(db)
    }

    /// Get the database file path
    fn database_path() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;

        Ok(data_dir.join("tabssh").join("tabssh.db"))
    }

    /// Initialize database schema
    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- Connection profiles
            CREATE TABLE IF NOT EXISTS connections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                host TEXT NOT NULL,
                port INTEGER NOT NULL DEFAULT 22,
                username TEXT NOT NULL,
                auth_type TEXT NOT NULL DEFAULT 'password',
                key_id TEXT,
                group_name TEXT,
                timeout INTEGER NOT NULL DEFAULT 30,
                keepalive INTEGER NOT NULL DEFAULT 60,
                compression INTEGER NOT NULL DEFAULT 0,
                connection_count INTEGER NOT NULL DEFAULT 0,
                last_connected TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            -- SSH keys
            CREATE TABLE IF NOT EXISTS ssh_keys (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                key_type TEXT NOT NULL,
                fingerprint TEXT NOT NULL,
                public_key TEXT NOT NULL,
                encrypted_private_key BLOB,
                created_at TEXT NOT NULL
            );

            -- Known hosts
            CREATE TABLE IF NOT EXISTS known_hosts (
                id TEXT PRIMARY KEY,
                host TEXT NOT NULL,
                port INTEGER NOT NULL,
                key_type TEXT NOT NULL,
                fingerprint TEXT NOT NULL,
                public_key TEXT NOT NULL,
                first_seen TEXT NOT NULL,
                last_seen TEXT NOT NULL,
                UNIQUE(host, port)
            );

            -- Themes
            CREATE TABLE IF NOT EXISTS themes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                is_builtin INTEGER NOT NULL DEFAULT 0,
                colors TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            -- Settings
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )?;

        Ok(())
    }

    /// Get the underlying connection (for advanced queries)
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}
