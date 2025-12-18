//! Asset API functions

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

pub async fn create_asset(asset: &AssetDto) -> Result<AssetDto, String> {
    let url = format!("{API_BASE}/assets");
    Request::post(&url)
        .json(asset)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_asset(asset: &AssetDto) -> Result<AssetDto, String> {
    let id = asset.id.ok_or("Asset ID required")?;
    let url = format!("{API_BASE}/assets/{id}");
    Request::put(&url)
        .json(asset)
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
