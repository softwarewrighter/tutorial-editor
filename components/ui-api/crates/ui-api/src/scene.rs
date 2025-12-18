//! Scene API functions

use gloo_net::http::Request;
use ui_core::SceneDto;

const API_BASE: &str = "/api";

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

pub async fn update_scene_metadata(
    scene_id: i64,
    title: &str,
    description: Option<&str>,
    sort_order: i32,
    existing_script_text: Option<&str>,
) -> Result<SceneDto, String> {
    let url = format!("{API_BASE}/scenes/{scene_id}");
    let body = serde_json::json!({
        "title": title,
        "description": description,
        "sort_order": sort_order,
        "script_text": existing_script_text,
    });
    Request::put(&url)
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
