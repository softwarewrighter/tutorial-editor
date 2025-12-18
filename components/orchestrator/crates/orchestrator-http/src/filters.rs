use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

pub fn with_app<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = (Arc<OrchestratorApp<P, S, A>>,), Error = Infallible> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::any().map(move || app.clone())
}
