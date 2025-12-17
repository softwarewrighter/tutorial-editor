mod filters;
mod handlers;

use filters::with_app;
use handlers::{handle_create_project, handle_health, handle_list_projects, CreateProjectRequest};
use orchestrator_core::{ports::ProjectRepository, OrchestratorApp};
use std::sync::Arc;
use warp::Filter;

#[derive(Clone)]
pub struct HttpServer<R>
where
    R: ProjectRepository + 'static,
{
    app: Arc<OrchestratorApp<R>>,
}

impl<R> HttpServer<R>
where
    R: ProjectRepository + 'static,
{
    pub fn new(app: Arc<OrchestratorApp<R>>) -> Self {
        Self { app }
    }

    pub async fn run(self, bind_addr: &str, port: u16) -> anyhow::Result<()> {
        let api = self.routes();
        let addr = format!("{bind_addr}:{port}");
        tracing::info!("HTTP server listening on http://{addr}");
        warp::serve(api).run(([0, 0, 0, 0], port)).await;
        Ok(())
    }

    fn routes(self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let health = warp::path!("api" / "health")
            .and(warp::get())
            .and_then(handle_health);

        let list = warp::path!("api" / "projects")
            .and(warp::get())
            .and(with_app(self.app.clone()))
            .and_then(handle_list_projects);

        let create = warp::path!("api" / "projects")
            .and(warp::post())
            .and(with_app(self.app.clone()))
            .and(warp::body::json::<CreateProjectRequest>())
            .and_then(handle_create_project);

        health.or(list).or(create)
    }
}
