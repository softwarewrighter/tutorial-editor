use gloo_net::http::Request;
use ui_core::AssetDto;

const API_BASE: &str = "/api";

pub async fn fetch_scene_assets(scene_id: i64) -> Result<Vec<AssetDto>, String> {
    let url = format!("{API_BASE}/scenes/{scene_id}/assets");
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_asset(
    project_id: i64,
    scene_id: Option<i64>,
    name: &str,
    asset_type: &str,
    file_path: Option<&str>,
    url: Option<&str>,
    metadata: Option<&str>,
) -> Result<AssetDto, String> {
    let api_url = format!("{API_BASE}/assets");
    let body = serde_json::json!({
        "project_id": project_id,
        "scene_id": scene_id,
        "name": name,
        "asset_type": asset_type,
        "file_path": file_path,
        "url": url,
        "metadata": metadata,
    });
    Request::post(&api_url)
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_asset(
    asset_id: i64,
    name: &str,
    asset_type: &str,
    file_path: Option<&str>,
    url: Option<&str>,
    metadata: Option<&str>,
) -> Result<AssetDto, String> {
    let api_url = format!("{API_BASE}/assets/{asset_id}");
    let body = serde_json::json!({
        "name": name,
        "asset_type": asset_type,
        "file_path": file_path,
        "url": url,
        "metadata": metadata,
    });
    Request::put(&api_url)
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_asset(asset_id: i64) -> Result<(), String> {
    let url = format!("{API_BASE}/assets/{asset_id}");
    Request::delete(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
