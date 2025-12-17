use ui_core::SceneDto;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api;

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
            if api::create_project(&slug, &title).await.is_ok() {
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
                if api::create_scene(project_id, &title, sort).await.is_ok() {
                    show.set(false);
                    refresh.set(*refresh + 1);
                }
            });
        }
    })
}

pub fn build_cancel_project_form(show: &UseStateHandle<bool>) -> Callback<()> {
    let show = show.clone();
    Callback::from(move |_| show.set(false))
}

pub fn build_cancel_scene_form(show: &UseStateHandle<bool>) -> Callback<()> {
    let show = show.clone();
    Callback::from(move |_| show.set(false))
}

pub fn build_edit_scene(editing: &UseStateHandle<bool>) -> Callback<SceneDto> {
    let editing = editing.clone();
    Callback::from(move |_: SceneDto| editing.set(true))
}

pub fn build_save_scene(
    selected_scene: &UseStateHandle<Option<SceneDto>>,
    editing: &UseStateHandle<bool>,
    refresh: &UseStateHandle<u32>,
) -> Callback<SceneDto> {
    let ss = selected_scene.clone();
    let editing = editing.clone();
    let refresh = refresh.clone();
    Callback::from(move |scene: SceneDto| {
        let ss = ss.clone();
        let editing = editing.clone();
        let refresh = refresh.clone();
        spawn_local(async move {
            let result = api::update_scene_metadata(
                scene.id.unwrap_or(0),
                &scene.title,
                scene.description.as_deref(),
                scene.sort_order,
                scene.script_text.as_deref(),
            )
            .await;
            if let Ok(updated) = result {
                ss.set(Some(updated));
                editing.set(false);
                refresh.set(*refresh + 1);
            }
        });
    })
}

pub fn build_cancel_edit(editing: &UseStateHandle<bool>) -> Callback<()> {
    let editing = editing.clone();
    Callback::from(move |_| editing.set(false))
}
