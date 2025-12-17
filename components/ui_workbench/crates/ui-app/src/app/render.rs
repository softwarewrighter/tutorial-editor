use ui_core::{ProjectDto, SceneDto};
use yew::prelude::*;

use crate::project_form::ProjectForm;
use crate::project_list::ProjectList;
use crate::scene_detail::SceneDetail;
use crate::scene_edit_form::SceneEditForm;
use crate::scene_form::SceneForm;
use crate::scene_list::SceneList;

use super::AppCallbacks;

pub fn render_project_section(
    projects: &[ProjectDto],
    callbacks: &AppCallbacks,
    show_form: bool,
) -> Html {
    html! {
        <section class="project-section">
            <h2>{ "Projects" }</h2>
            <button onclick={callbacks.on_show_project_form.clone()}>{ "New Project" }</button>
            { if show_form { render_project_form(callbacks) } else { html!{} } }
            <ProjectList
                projects={projects.to_vec()}
                on_select={callbacks.on_project_select.clone()}
            />
        </section>
    }
}

fn render_project_form(callbacks: &AppCallbacks) -> Html {
    html! {
        <ProjectForm
            on_submit={callbacks.on_project_submit.clone()}
            on_cancel={callbacks.on_cancel_project_form.clone()}
        />
    }
}

pub fn render_scene_section(
    scenes: &[SceneDto],
    selected_project: &Option<i64>,
    callbacks: &AppCallbacks,
    show_form: bool,
) -> Html {
    if selected_project.is_none() {
        return html! {};
    }
    html! {
        <section class="scene-section">
            <h2>{ "Scenes" }</h2>
            <button onclick={callbacks.on_show_scene_form.clone()}>{ "New Scene" }</button>
            { if show_form { render_scene_form(callbacks) } else { html!{} } }
            <SceneList scenes={scenes.to_vec()} on_select={callbacks.on_scene_select.clone()} />
        </section>
    }
}

fn render_scene_form(callbacks: &AppCallbacks) -> Html {
    html! {
        <SceneForm
            on_submit={callbacks.on_scene_submit.clone()}
            on_cancel={callbacks.on_cancel_scene_form.clone()}
        />
    }
}

pub fn render_detail_section(
    selected_scene: &Option<SceneDto>,
    callbacks: &AppCallbacks,
    editing: bool,
) -> Html {
    match selected_scene {
        None => html! {},
        Some(scene) if editing => html! {
            <section class="detail-section">
                <SceneEditForm
                    scene={scene.clone()}
                    on_save={callbacks.on_save_scene.clone()}
                    on_cancel={callbacks.on_cancel_edit.clone()}
                />
            </section>
        },
        Some(scene) => html! {
            <section class="detail-section">
                <SceneDetail scene={scene.clone()} on_edit={callbacks.on_edit_scene.clone()} />
            </section>
        },
    }
}
