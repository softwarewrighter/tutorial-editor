use anyhow::{Context, Result};
use time::OffsetDateTime;

pub fn parse(s: String) -> OffsetDateTime {
    OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
}

pub fn format(dt: OffsetDateTime) -> Result<String> {
    dt.format(&time::format_description::well_known::Rfc3339)
        .context("failed to format timestamp")
}
