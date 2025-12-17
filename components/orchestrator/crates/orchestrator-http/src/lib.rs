use orchestrator-core::{domain::Project, OrchestratorApp};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

#[derive(Clone)]
pub struct HttpServer<R> {
    app: Arc<OrchestratorApp<R>>,
}

impl<R> HttpServer<R>
where
    R: orchestrator_core::ports::ProjectRepository + 'static,
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

    fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let health = warp::path!("api" / "health")
            .and(warp::get())
            .and_then(Self::handle_health);

        let app = self.app.clone();
        let list_projects = warp::path!("api" / "projects")
            .and(warp::get())
            .and(with_app(app.clone()))
            .and_then(Self::handle_list_projects);

        let app2 = self.app.clone();
        let create_project = warp::path!("api" / "projects")
            .and(warp::post())
            .and(with_app(app2.clone()))
            .and(warp::body::json())
            .and_then(Self::handle_create_project);

        health.or(list_projects).or(create_project)
    }

    async fn handle_health() -> Result<impl warp::Reply, Infallible> {
        #[derive(Serialize)]
        struct HealthReply {
            status: &'static str,
        }

        Ok(warp::reply::json(&HealthReply { status: "ok" }))
    }

    async fn handle_list_projects<Rp>(
        app: Arc<OrchestratorApp<Rp>>,
    ) -> Result<impl warp::Reply, Infallible>
    where
        Rp: orchestrator_core::ports::ProjectRepository + 'static,
    {
        let projects = app.list_projects().await.unwrap_or_default();
        Ok(warp::reply::json(&projects))
    }

    async fn handle_create_project<Rp>(
        app: Arc<OrchestratorApp<Rp>>,
        payload: CreateProjectRequest,
    ) -> Result<impl warp::Reply, Infallible>
    where
        Rp: orchestrator_core::ports::ProjectRepository + 'static,
    {
        let created = app
            .create_project(payload.slug, payload.title)
            .await
            .unwrap();
        Ok(warp::reply::json(&created))
    }
}

fn with_app<R>(
    app: Arc<OrchestratorApp<R>>,
) -> impl Filter<Extract = (Arc<OrchestratorApp<R>>,), Error = Infallible> + Clone
where
    R: orchestrator_core::ports::ProjectRepository + 'static,
{
    warp::any().map(move || app.clone())
}

#[derive(Debug, Deserialize)]
struct CreateProjectRequest {
    slug: String,
    title: String,
}
