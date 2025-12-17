use crate::filters::with_app;
use crate::handlers::{CreateProjectRequest, handle_create_project, handle_list_projects};
use orchestrator_core::{
    OrchestratorApp,
    ports::{ProjectRepository, SceneRepository},
};
use std::sync::Arc;
use warp::Filter;

pub fn routes<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    list(app.clone()).or(create(app))
}

fn list<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::path!("api" / "projects")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_projects)
}

fn create<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::path!("api" / "projects")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<CreateProjectRequest>())
        .and_then(handle_create_project)
}
