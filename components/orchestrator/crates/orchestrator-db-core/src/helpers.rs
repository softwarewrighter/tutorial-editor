//! Timestamp parsing and formatting helpers

use anyhow::{Context, Result};
use time::OffsetDateTime;

/// Parse an RFC3339 timestamp string into OffsetDateTime
pub fn parse_timestamp(s: &str) -> OffsetDateTime {
    OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
}

/// Format an OffsetDateTime as RFC3339 string
pub fn format_timestamp(dt: OffsetDateTime) -> Result<String> {
    dt.format(&time::format_description::well_known::Rfc3339)
        .context("failed to format timestamp")
}
