mod filters;
mod handlers;
mod project_routes;
mod routes;
mod scene_routes;

use orchestrator_core::{
    OrchestratorApp,
    ports::{ProjectRepository, SceneRepository},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HttpServer<P, S>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    app: Arc<OrchestratorApp<P, S>>,
}

impl<P, S> HttpServer<P, S>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    pub fn new(app: Arc<OrchestratorApp<P, S>>) -> Self {
        Self { app }
    }

    pub async fn run(self, bind_addr: &str, port: u16) -> anyhow::Result<()> {
        let api = routes::all(self.app);
        let addr = format!("{bind_addr}:{port}");
        tracing::info!("HTTP server listening on http://{addr}");
        warp::serve(api).run(([0, 0, 0, 0], port)).await;
        Ok(())
    }
}
