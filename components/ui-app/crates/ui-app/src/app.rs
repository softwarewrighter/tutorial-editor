use ui_core::{ProjectDto, SceneDto};
use yew::prelude::*;

use crate::callbacks::{build_callbacks, use_fetch_projects, use_fetch_scenes};
use crate::render::{render_detail_section, render_project_section, render_scene_section};

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
    let (proj, scn) = (use_state(Vec::<ProjectDto>::new), use_state(Vec::<SceneDto>::new));
    let (sel_p, sel_s) = (use_state(|| None::<i64>), use_state(|| None::<SceneDto>));
    let (sh_p, sh_s, ed) = (use_state(|| false), use_state(|| false), use_state(|| false));
    let rf = use_state(|| 0u32);
    use_fetch_projects(&proj, *rf);
    use_fetch_scenes(&scn, *sel_p, *rf);
    let cb = build_callbacks(&sel_p, &sel_s, &sh_p, &sh_s, &ed, &rf);
    html! {
        <main class="app-root">
            <Header />
            { render_project_section(&proj, &cb, *sh_p) }
            { render_scene_section(&scn, &sel_p, &cb, *sh_s) }
            { render_detail_section(&sel_s, &cb, *ed) }
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

const COPYRIGHT: &str = "Copyright (c) 2025 Michael A Wright";
const LICENSE: &str = "MIT License";
const REPOSITORY: &str = "https://github.com/softwarewrighter/tutorial-editor";
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");
const BUILD_HOST: &str = env!("BUILD_HOST");
const BUILD_TIME: &str = env!("BUILD_TIME");

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <p>{ COPYRIGHT } { " | " } { LICENSE }</p>
            <p>{ "Repository: " }<a href={REPOSITORY} target="_blank">{ REPOSITORY }</a></p>
            <p>{ format!("Build: {} @ {} ({})", BUILD_COMMIT, BUILD_HOST, BUILD_TIME) }</p>
        </footer>
    }
}
