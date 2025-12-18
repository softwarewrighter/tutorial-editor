use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Project;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn list_projects(&self) -> Result<Vec<Project>>;
    async fn get_project(&self, id: i64) -> Result<Option<Project>>;
    async fn create_project(&self, project: Project) -> Result<Project>;
}
