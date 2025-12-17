use crate::{
    config::AppConfig,
    ports::{ProjectRepository, SceneRepository},
};
use std::sync::Arc;

pub struct OrchestratorApp<P, S>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    pub config: AppConfig,
    pub project_repo: Arc<P>,
    pub scene_repo: Arc<S>,
}

impl<P, S> OrchestratorApp<P, S>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    pub fn new(config: AppConfig, project_repo: Arc<P>, scene_repo: Arc<S>) -> Self {
        Self {
            config,
            project_repo,
            scene_repo,
        }
    }
}
