use ui_core::SceneDto;
use ui_macros::{hide_callback, show_callback, some_callback};
use yew::prelude::*;

use crate::app::AppCallbacks;
use crate::callbacks2::{build_edit_scene, build_project_submit, build_save_scene, build_scene_submit};

pub fn build_callbacks(
    selected_project: &UseStateHandle<Option<i64>>,
    selected_scene: &UseStateHandle<Option<SceneDto>>,
    show_project_form: &UseStateHandle<bool>,
    show_scene_form: &UseStateHandle<bool>,
    editing_scene: &UseStateHandle<bool>,
    refresh_trigger: &UseStateHandle<u32>,
) -> AppCallbacks {
    AppCallbacks {
        on_project_select: build_project_select(selected_project, selected_scene),
        on_scene_select: some_callback!(selected_scene),
        on_show_project_form: show_callback!(show_project_form),
        on_show_scene_form: show_callback!(show_scene_form),
        on_project_submit: build_project_submit(show_project_form, refresh_trigger),
        on_scene_submit: build_scene_submit(selected_project, show_scene_form, refresh_trigger),
        on_cancel_project_form: hide_callback!(show_project_form),
        on_cancel_scene_form: hide_callback!(show_scene_form),
        on_edit_scene: build_edit_scene(editing_scene),
        on_save_scene: build_save_scene(selected_scene, editing_scene, refresh_trigger),
        on_cancel_edit: hide_callback!(editing_scene),
    }
}

fn build_project_select(
    selected_project: &UseStateHandle<Option<i64>>,
    selected_scene: &UseStateHandle<Option<SceneDto>>,
) -> Callback<i64> {
    let sp = selected_project.clone();
    let ss = selected_scene.clone();
    Callback::from(move |id: i64| {
        sp.set(Some(id));
        ss.set(None);
    })
}
