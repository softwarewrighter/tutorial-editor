//! Project operations trait definition

use anyhow::Result;
use orchestrator_domain::Project;

/// Project operations trait
#[allow(async_fn_in_trait)]
pub trait ProjectOps {
    /// List all projects
    async fn list_projects(&self) -> Result<Vec<Project>>;
    /// Get a project by ID
    async fn get_project(&self, id: i64) -> Result<Option<Project>>;
    /// Create a new project
    async fn create_project(&self, slug: String, title: String) -> Result<Project>;
}
