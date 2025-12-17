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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: Option<i64>,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub script_text: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Scene {
    pub fn new(project_id: i64, title: String, sort_order: i32) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: None,
            project_id,
            title,
            description: None,
            sort_order,
            script_text: None,
            created_at: now,
            updated_at: now,
        }
    }
}
