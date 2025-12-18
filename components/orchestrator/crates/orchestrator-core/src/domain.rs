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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Option<i64>,
    pub project_id: i64,
    pub scene_id: Option<i64>,
    pub name: String,
    pub asset_type: String,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub metadata: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Asset {
    pub fn new(
        project_id: i64,
        scene_id: Option<i64>,
        name: String,
        asset_type: String,
        file_path: Option<String>,
        url: Option<String>,
        metadata: Option<String>,
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: None,
            project_id,
            scene_id,
            name,
            asset_type,
            file_path,
            url,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Target specification for MCP/Playwright screen capture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTarget {
    pub url: String,
    pub actions: Vec<SceneAction>,
    pub width: u32,
    pub height: u32,
}

/// Actions to perform during scene capture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SceneAction {
    Click { selector: String },
    Type { selector: String, text: String },
    Wait { duration_ms: u64 },
    Screenshot,
}
