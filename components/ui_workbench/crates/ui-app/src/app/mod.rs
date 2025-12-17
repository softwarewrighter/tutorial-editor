use ui_core::{ProjectDto, SceneDto};
use yew::prelude::*;

mod callbacks;
mod callbacks2;
mod hooks;
mod render;

use callbacks::build_callbacks;
use hooks::{use_fetch_projects, use_fetch_scenes};
use render::{render_detail_section, render_project_section, render_scene_section};

use crate::footer::Footer;

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

#[function_component(App)]
pub fn app() -> Html {
    let projects = use_state(Vec::<ProjectDto>::new);
    let scenes = use_state(Vec::<SceneDto>::new);
    let selected_project = use_state(|| None::<i64>);
    let selected_scene = use_state(|| None::<SceneDto>);
    let show_project_form = use_state(|| false);
    let show_scene_form = use_state(|| false);
    let editing_scene = use_state(|| false);
    let refresh_trigger = use_state(|| 0u32);

    use_fetch_projects(&projects, *refresh_trigger);
    use_fetch_scenes(&scenes, *selected_project, *refresh_trigger);

    let callbacks = build_callbacks(
        &selected_project,
        &selected_scene,
        &show_project_form,
        &show_scene_form,
        &editing_scene,
        &refresh_trigger,
    );

    html! {
        <main class="app-root">
            <Header />
            { render_project_section(&projects, &callbacks, *show_project_form) }
            { render_scene_section(&scenes, &selected_project, &callbacks, *show_scene_form) }
            { render_detail_section(&selected_scene, &callbacks, *editing_scene) }
            <Footer />
        </main>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
            <h1>{ "Avatar Video Orchestrator - UI Workbench" }</h1>
            <p>{ "Select a project to view its scenes." }</p>
        </header>
    }
}
