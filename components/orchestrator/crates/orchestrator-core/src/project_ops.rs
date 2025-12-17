use crate::{
    domain::Project,
    ports::{ProjectRepository, SceneRepository},
    services::OrchestratorApp,
};
use anyhow::Result;

impl<P, S> OrchestratorApp<P, S>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.project_repo.list_projects().await
    }

    pub async fn create_project(&self, slug: String, title: String) -> Result<Project> {
        let project = Project::new(slug, title);
        self.project_repo.create_project(project).await
    }
}
