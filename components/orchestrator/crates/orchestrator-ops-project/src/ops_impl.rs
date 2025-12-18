//! Project operations implementation

use anyhow::Result;
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::Project;

use crate::ProjectOps;

impl<P, S, A> ProjectOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn list_projects(&self) -> Result<Vec<Project>> {
        self.project_repo.list_projects().await
    }

    async fn get_project(&self, id: i64) -> Result<Option<Project>> {
        self.project_repo.get_project(id).await
    }

    async fn create_project(&self, slug: String, title: String) -> Result<Project> {
        let project = Project::new(slug, title);
        self.project_repo.create_project(project).await
    }
}
