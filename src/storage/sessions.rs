//! Session persistence

use super::database::Database;
use anyhow::Result;
use chrono::{DateTime, Utc};

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

        let sessions = stmt
            .query_map([], |row| {
                let scrollback_json: String = row.get(5)?;
                let scrollback: Vec<String> =
                    serde_json::from_str(&scrollback_json).unwrap_or_default();

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
        db.connection()
            .execute("DELETE FROM saved_sessions WHERE id = ?1", [id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::database::Database;

    // NOTE: `Database::initialize()` (src/storage/database.rs) never creates a
    // `saved_sessions` table (it only creates `connections`, `ssh_keys`,
    // `known_hosts`, `themes`, `settings`). As a result every method here
    // currently fails against a freshly-initialized database with
    // "no such table: saved_sessions". These tests document that actual,
    // current (buggy) behavior rather than asserting a happy path that does
    // not exist yet. This is flagged separately as a bug outside the scope
    // of this test-only change (see TODO.AI.md).

    fn sample_session(id: &str) -> SavedSession {
        SavedSession {
            id: id.to_string(),
            connection_id: "conn-1".to_string(),
            host: "example.com".to_string(),
            user: "root".to_string(),
            port: 22,
            scrollback: vec!["line1".to_string(), "line2".to_string()],
            cursor_row: 3,
            cursor_col: 7,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn save_fails_because_saved_sessions_table_does_not_exist() {
        let db = Database::open_in_memory().unwrap();
        let session = sample_session("s1");

        let result = session.save(&db);

        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("no such table"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn load_all_fails_because_saved_sessions_table_does_not_exist() {
        let db = Database::open_in_memory().unwrap();

        let result = SavedSession::load_all(&db);

        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("no such table"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn delete_fails_because_saved_sessions_table_does_not_exist() {
        let db = Database::open_in_memory().unwrap();

        let result = SavedSession::delete("s1", &db);

        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("no such table"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn delete_on_empty_id_also_fails_same_way() {
        let db = Database::open_in_memory().unwrap();

        let result = SavedSession::delete("", &db);

        assert!(result.is_err());
    }

    #[test]
    fn sample_session_round_trips_field_values() {
        // Boundary/happy-path check on the struct itself, independent of
        // the broken persistence layer: verifies field construction and
        // clone semantics are correct.
        let session = sample_session("s1");
        let cloned = session.clone();

        assert_eq!(cloned.id, "s1");
        assert_eq!(cloned.connection_id, "conn-1");
        assert_eq!(cloned.host, "example.com");
        assert_eq!(cloned.user, "root");
        assert_eq!(cloned.port, 22);
        assert_eq!(cloned.scrollback, vec!["line1", "line2"]);
        assert_eq!(cloned.cursor_row, 3);
        assert_eq!(cloned.cursor_col, 7);
    }

    #[test]
    fn sample_session_supports_empty_scrollback() {
        let mut session = sample_session("s2");
        session.scrollback = vec![];

        assert!(session.scrollback.is_empty());
    }
}
