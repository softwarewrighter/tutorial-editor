use orchestrator_core::{ports::ProjectRepository, OrchestratorApp};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub slug: String,
    pub title: String,
}

pub async fn handle_health() -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct HealthReply {
        status: &'static str,
    }

    Ok(warp::reply::json(&HealthReply { status: "ok" }))
}

pub async fn handle_list_projects<R>(
    app: Arc<OrchestratorApp<R>>,
) -> Result<impl warp::Reply, Infallible>
where
    R: ProjectRepository + 'static,
{
    let projects = app.list_projects().await.unwrap_or_default();
    Ok(warp::reply::json(&projects))
}

pub async fn handle_create_project<R>(
    app: Arc<OrchestratorApp<R>>,
    payload: CreateProjectRequest,
) -> Result<impl warp::Reply, Infallible>
where
    R: ProjectRepository + 'static,
{
    let created = app
        .create_project(payload.slug, payload.title)
        .await
        .unwrap();
    Ok(warp::reply::json(&created))
}
