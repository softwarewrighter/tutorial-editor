use ui_api::{create_project, create_scene, update_scene_metadata};
use ui_core::SceneDto;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn build_project_submit(
    show: &UseStateHandle<bool>,
    refresh: &UseStateHandle<u32>,
) -> Callback<(String, String)> {
    let show = show.clone();
    let refresh = refresh.clone();
    Callback::from(move |(slug, title): (String, String)| {
        let show = show.clone();
        let refresh = refresh.clone();
        spawn_local(async move {
            if create_project(&slug, &title).await.is_ok() {
                show.set(false);
                refresh.set(*refresh + 1);
            }
        });
    })
}

pub fn build_scene_submit(
    selected_project: &UseStateHandle<Option<i64>>,
    show: &UseStateHandle<bool>,
    refresh: &UseStateHandle<u32>,
) -> Callback<(String, i32)> {
    let pid = *selected_project.clone();
    let show = show.clone();
    let refresh = refresh.clone();
    Callback::from(move |(title, sort): (String, i32)| {
        let show = show.clone();
        let refresh = refresh.clone();
        if let Some(project_id) = pid {
            spawn_local(async move {
                if create_scene(project_id, &title, sort).await.is_ok() {
                    show.set(false);
                    refresh.set(*refresh + 1);
                }
            });
        }
    })
}

pub fn build_edit_scene(editing: &UseStateHandle<bool>) -> Callback<SceneDto> {
    let editing = editing.clone();
    Callback::from(move |_: SceneDto| editing.set(true))
}

pub fn build_save_scene(ss: &UseStateHandle<Option<SceneDto>>, ed: &UseStateHandle<bool>, rf: &UseStateHandle<u32>) -> Callback<SceneDto> {
    let (ss, ed, rf) = (ss.clone(), ed.clone(), rf.clone());
    Callback::from(move |s: SceneDto| {
        let (ss, ed, rf) = (ss.clone(), ed.clone(), rf.clone());
        spawn_local(async move {
            if let Ok(u) = update_scene_metadata(s.id.unwrap_or(0), &s.title, s.description.as_deref(), s.sort_order, s.script_text.as_deref()).await {
                ss.set(Some(u)); ed.set(false); rf.set(*rf + 1);
            }
        });
    })
}
