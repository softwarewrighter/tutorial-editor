use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectDto {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}
