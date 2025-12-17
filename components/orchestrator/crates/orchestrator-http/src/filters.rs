use orchestrator_core::{ports::ProjectRepository, OrchestratorApp};
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

pub fn with_app<R>(
    app: Arc<OrchestratorApp<R>>,
) -> impl Filter<Extract = (Arc<OrchestratorApp<R>>,), Error = Infallible> + Clone
where
    R: ProjectRepository + 'static,
{
    warp::any().map(move || app.clone())
}
