use crate::domain::Project;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn list_projects(&self) -> Result<Vec<Project>>;
    async fn create_project(&self, project: Project) -> Result<Project>;
}
