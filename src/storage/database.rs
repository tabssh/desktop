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

    // ========== Known Hosts Methods ==========
}

/// Known host entry
#[derive(Debug, Clone)]
pub struct KnownHost {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub key_type: String,
    pub fingerprint: String,
    pub public_key: Vec<u8>,
    pub first_seen: String,
    pub last_seen: String,
}

impl Database {
    /// Get known host by host and port
    pub fn get_known_host(&self, host: &str, port: u16) -> Result<Option<KnownHost>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, host, port, key_type, fingerprint, public_key, first_seen, last_seen 
             FROM known_hosts WHERE host = ?1 AND port = ?2"
        )?;

        let result = stmt.query_row(rusqlite::params![host, port], |row| {
            Ok(KnownHost {
                id: row.get(0)?,
                host: row.get(1)?,
                port: row.get::<_, i64>(2)? as u16,
                key_type: row.get(3)?,
                fingerprint: row.get(4)?,
                public_key: row.get(5)?,
                first_seen: row.get(6)?,
                last_seen: row.get(7)?,
            })
        });

        match result {
            Ok(host) => Ok(Some(host)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Add new known host
    pub fn add_known_host(
        &self,
        host: &str,
        port: u16,
        key_type: &str,
        fingerprint: &str,
        public_key: &[u8],
    ) -> Result<()> {
        use uuid::Uuid;
        
        let id = Uuid::new_v4().to_string();
        let now = chrono::Local::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO known_hosts (id, host, port, key_type, fingerprint, public_key, first_seen, last_seen)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![id, host, port as i64, key_type, fingerprint, public_key, &now, &now],
        )?;

        log::info!("Added known host: {}:{} ({})", host, port, fingerprint);
        Ok(())
    }

    /// Update last_seen timestamp for known host
    pub fn update_known_host_last_seen(&self, host: &str, port: u16) -> Result<()> {
        let now = chrono::Local::now().to_rfc3339();
        
        self.conn.execute(
            "UPDATE known_hosts SET last_seen = ?1 WHERE host = ?2 AND port = ?3",
            rusqlite::params![&now, host, port as i64],
        )?;

        Ok(())
    }

    /// Remove known host
    pub fn remove_known_host(&self, host: &str, port: u16) -> Result<()> {
        self.conn.execute(
            "DELETE FROM known_hosts WHERE host = ?1 AND port = ?2",
            rusqlite::params![host, port as i64],
        )?;

        log::info!("Removed known host: {}:{}", host, port);
        Ok(())
    }

    /// Get all known hosts
    pub fn list_known_hosts(&self) -> Result<Vec<KnownHost>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, host, port, key_type, fingerprint, public_key, first_seen, last_seen 
             FROM known_hosts ORDER BY last_seen DESC"
        )?;

        let hosts = stmt.query_map([], |row| {
            Ok(KnownHost {
                id: row.get(0)?,
                host: row.get(1)?,
                port: row.get::<_, i64>(2)? as u16,
                key_type: row.get(3)?,
                fingerprint: row.get(4)?,
                public_key: row.get(5)?,
                first_seen: row.get(6)?,
                last_seen: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(hosts)
    }
}
