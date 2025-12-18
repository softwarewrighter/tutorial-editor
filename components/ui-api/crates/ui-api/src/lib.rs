//! HTTP API client for Tutorial Editor UI

mod asset;
mod project;
mod scene;

pub use asset::{create_asset, delete_asset, fetch_scene_assets, update_asset};
pub use project::{create_project, fetch_projects};
pub use scene::{create_scene, fetch_scenes, update_scene_metadata};
