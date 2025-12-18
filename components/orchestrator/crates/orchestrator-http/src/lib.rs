//! HTTP server for the orchestrator

mod routes;

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use std::sync::Arc;

/// HTTP server wrapper
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
    /// Create a new HTTP server
    pub fn new(app: Arc<OrchestratorApp<P, S, A>>) -> Self {
        Self { app }
    }

    /// Run the HTTP server
    pub async fn run(self, bind_addr: &str, port: u16) -> anyhow::Result<()> {
        let api = routes::all(self.app);
        let addr = format!("{bind_addr}:{port}");
        tracing::info!("HTTP server listening on http://{addr}");
        warp::serve(api).run(([0, 0, 0, 0], port)).await;
        Ok(())
    }
}
