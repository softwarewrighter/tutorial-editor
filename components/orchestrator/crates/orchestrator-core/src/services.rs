use crate::{
    config::AppConfig,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use std::sync::Arc;

pub struct OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub config: AppConfig,
    pub project_repo: Arc<P>,
    pub scene_repo: Arc<S>,
    pub asset_repo: Arc<A>,
}

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub fn new(
        config: AppConfig,
        project_repo: Arc<P>,
        scene_repo: Arc<S>,
        asset_repo: Arc<A>,
    ) -> Self {
        Self {
            config,
            project_repo,
            scene_repo,
            asset_repo,
        }
    }
}
