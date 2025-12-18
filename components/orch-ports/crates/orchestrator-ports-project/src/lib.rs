//! Project repository trait

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Project;

/// Repository trait for project persistence
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// List all projects
    async fn list_projects(&self) -> Result<Vec<Project>>;

    /// Get a project by ID
    async fn get_project(&self, id: i64) -> Result<Option<Project>>;

    /// Create a new project
    async fn create_project(&self, project: Project) -> Result<Project>;
}
