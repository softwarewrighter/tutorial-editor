use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Project {
    pub fn new(slug: String, title: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: None,
            slug,
            title,
            subtitle: None,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
}
