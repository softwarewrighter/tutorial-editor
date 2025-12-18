mod asset_routes;
mod filters;
mod handlers;
mod project_routes;
mod routes;
mod scene_routes;

use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HttpServer<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    app: Arc<OrchestratorApp<P, S, A>>,
}

impl<P, S, A> HttpServer<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub fn new(app: Arc<OrchestratorApp<P, S, A>>) -> Self {
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
