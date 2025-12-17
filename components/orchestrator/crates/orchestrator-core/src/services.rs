use crate::{config::AppConfig, domain::Project, ports::ProjectRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct OrchestratorApp<R>
where
    R: ProjectRepository + 'static,
{
    pub config: AppConfig,
    pub project_repo: Arc<R>,
}

impl<R> OrchestratorApp<R>
where
    R: ProjectRepository + 'static,
{
    pub fn new(config: AppConfig, project_repo: Arc<R>) -> Self {
        Self { config, project_repo }
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.project_repo.list_projects().await
    }

    pub async fn create_project(&self, slug: String, title: String) -> Result<Project> {
        let project = Project::new(slug, title);
        self.project_repo.create_project(project).await
    }
}
