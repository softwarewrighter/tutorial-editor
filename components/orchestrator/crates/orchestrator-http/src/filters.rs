use orchestrator_core::{
    OrchestratorApp,
    ports::{ProjectRepository, SceneRepository},
};
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

pub fn with_app<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = (Arc<OrchestratorApp<P, S>>,), Error = Infallible> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::any().map(move || app.clone())
}
