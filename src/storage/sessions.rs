//! Session persistence

use anyhow::Result;
use chrono::{DateTime, Utc};
use super::database::Database;

#[derive(Debug, Clone)]
pub struct SavedSession {
    pub id: String,
    pub connection_id: String,
    pub host: String,
    pub user: String,
    pub port: u16,
    pub scrollback: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub created_at: DateTime<Utc>,
}

impl SavedSession {
    pub fn save(&self, db: &Database) -> Result<()> {
        let scrollback_json = serde_json::to_string(&self.scrollback)?;
        let now = Utc::now().to_rfc3339();
        
        db.connection().execute(
            "INSERT OR REPLACE INTO saved_sessions 
             (id, connection_id, host, user, port, scrollback, cursor_row, cursor_col, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                &self.id,
                &self.connection_id,
                &self.host,
                &self.user,
                self.port as i64,
                &scrollback_json,
                self.cursor_row as i64,
                self.cursor_col as i64,
                &now,
            ],
        )?;
        
        Ok(())
    }
    
    pub fn load_all(db: &Database) -> Result<Vec<SavedSession>> {
        let conn = db.connection();
        let mut stmt = conn.prepare(
            "SELECT id, connection_id, host, user, port, scrollback, cursor_row, cursor_col, created_at
             FROM saved_sessions ORDER BY created_at DESC"
        )?;
        
        let sessions = stmt.query_map([], |row| {
            let scrollback_json: String = row.get(5)?;
            let scrollback: Vec<String> = serde_json::from_str(&scrollback_json)
                .unwrap_or_default();
            
            Ok(SavedSession {
                id: row.get(0)?,
                connection_id: row.get(1)?,
                host: row.get(2)?,
                user: row.get(3)?,
                port: row.get::<_, i64>(4)? as u16,
                scrollback,
                cursor_row: row.get::<_, i64>(6)? as usize,
                cursor_col: row.get::<_, i64>(7)? as usize,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap_or_else(|_| Utc::now().into())
                    .into(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(sessions)
    }
    
    pub fn delete(id: &str, db: &Database) -> Result<()> {
        db.connection().execute(
            "DELETE FROM saved_sessions WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }
}
