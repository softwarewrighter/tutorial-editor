//! Core database functionality: schema initialization and timestamp helpers

mod helpers;
mod schema;

pub use helpers::{format_timestamp, parse_timestamp};
pub use schema::init_db;

// Re-export rusqlite types used across DB crates
pub use rusqlite::Connection;
