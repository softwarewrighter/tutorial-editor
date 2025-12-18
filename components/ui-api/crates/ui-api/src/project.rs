//! Project API functions

use gloo_net::http::Request;
use ui_core::ProjectDto;

const API_BASE: &str = "/api";

pub async fn fetch_projects() -> Result<Vec<ProjectDto>, String> {
    let url = format!("{API_BASE}/projects");
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_project(slug: &str, title: &str) -> Result<ProjectDto, String> {
    let url = format!("{API_BASE}/projects");
    let body = serde_json::json!({ "slug": slug, "title": title });
    Request::post(&url)
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
