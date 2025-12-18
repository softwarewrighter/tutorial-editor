use crate::routes::with_app;
use orchestrator_app::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use orchestrator_http_handlers::{CreateProjectRequest, handle_create_project, handle_list_projects};
use std::sync::Arc;
use warp::Filter;

pub fn routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    list(app.clone()).or(create(app))
}

fn list<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "projects")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_projects)
}

fn create<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "projects")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<CreateProjectRequest>())
        .and_then(handle_create_project)
}
