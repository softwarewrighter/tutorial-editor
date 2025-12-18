//! UI App callbacks for the Avatar Video Orchestrator

mod callbacks;
mod callbacks2;

pub use callbacks::{build_callbacks, use_fetch_projects, use_fetch_scenes};
pub use callbacks2::{build_edit_scene, build_project_submit, build_save_scene, build_scene_submit};

use ui_core::SceneDto;
use yew::prelude::*;

pub struct AppCallbacks {
    pub on_project_select: Callback<i64>,
    pub on_scene_select: Callback<SceneDto>,
    pub on_show_project_form: Callback<MouseEvent>,
    pub on_show_scene_form: Callback<MouseEvent>,
    pub on_project_submit: Callback<(String, String)>,
    pub on_scene_submit: Callback<(String, i32)>,
    pub on_cancel_project_form: Callback<()>,
    pub on_cancel_scene_form: Callback<()>,
    pub on_edit_scene: Callback<SceneDto>,
    pub on_save_scene: Callback<SceneDto>,
    pub on_cancel_edit: Callback<()>,
}
