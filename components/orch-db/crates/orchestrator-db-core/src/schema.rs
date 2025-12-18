//! Database schema initialization

use anyhow::Result;
use rusqlite::Connection;

/// Initialize database tables
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(())
}
