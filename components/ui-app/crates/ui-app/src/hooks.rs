use ui_api::{fetch_projects, fetch_scenes};
use ui_core::{ProjectDto, SceneDto};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[hook]
pub fn use_fetch_projects(projects: &UseStateHandle<Vec<ProjectDto>>, trigger: u32) {
    let projects = projects.clone();
    use_effect_with(trigger, move |_| {
        spawn_local(async move {
            if let Ok(data) = fetch_projects().await {
                projects.set(data);
            }
        });
        || ()
    });
}

#[hook]
pub fn use_fetch_scenes(
    scenes: &UseStateHandle<Vec<SceneDto>>,
    project_id: Option<i64>,
    trigger: u32,
) {
    let scenes = scenes.clone();
    use_effect_with((project_id, trigger), move |(pid, _)| {
        if let Some(id) = *pid {
            let scenes = scenes.clone();
            spawn_local(async move {
                if let Ok(data) = fetch_scenes(id).await {
                    scenes.set(data);
                }
            });
        } else {
            scenes.set(vec![]);
        }
        || ()
    });
}
