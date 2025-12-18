//! DTO types for UI data transfer

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectDto {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SceneDto {
    pub id: Option<i64>,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub script_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetDto {
    pub id: Option<i64>,
    pub project_id: i64,
    pub scene_id: Option<i64>,
    pub name: String,
    pub asset_type: String,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub metadata: Option<String>,
}
