#![allow(dead_code)]

use gloo_net::http::Request;
use ui_core::{ProjectDto, SceneDto};

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

pub async fn fetch_scenes(project_id: i64) -> Result<Vec<SceneDto>, String> {
    let url = format!("{API_BASE}/projects/{project_id}/scenes");
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

pub async fn create_scene(
    project_id: i64,
    title: &str,
    sort_order: i32,
) -> Result<SceneDto, String> {
    let url = format!("{API_BASE}/projects/{project_id}/scenes");
    let body = serde_json::json!({ "title": title, "sort_order": sort_order });
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
