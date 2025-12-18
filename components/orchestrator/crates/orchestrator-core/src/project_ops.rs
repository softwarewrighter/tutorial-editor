use crate::{
    domain::Project,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
    services::OrchestratorApp,
};
use anyhow::Result;

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.project_repo.list_projects().await
    }

    pub async fn get_project(&self, id: i64) -> Result<Option<Project>> {
        self.project_repo.get_project(id).await
    }

    pub async fn create_project(&self, slug: String, title: String) -> Result<Project> {
        let project = Project::new(slug, title);
        self.project_repo.create_project(project).await
    }
}
