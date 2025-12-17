use ui_core::SceneDto;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneDetailProps {
    pub scene: SceneDto,
    pub on_edit: Callback<SceneDto>,
}

#[function_component(SceneDetail)]
pub fn scene_detail(props: &SceneDetailProps) -> Html {
    let on_edit_click = {
        let scene = props.scene.clone();
        let on_edit = props.on_edit.clone();
        Callback::from(move |_| on_edit.emit(scene.clone()))
    };

    html! {
        <div class="scene-detail">
            { render_metadata(&props.scene, on_edit_click) }
            { render_script_text(&props.scene.script_text) }
        </div>
    }
}

fn render_metadata(scene: &SceneDto, on_edit: Callback<MouseEvent>) -> Html {
    let desc = scene.description.as_deref().unwrap_or("No description");
    html! {
        <div class="scene-metadata">
            <h2>{ &scene.title }</h2>
            <p class="scene-sort">{ format!("Sort order: {}", scene.sort_order) }</p>
            <p class="scene-description">{ desc }</p>
            <button class="edit-btn" onclick={on_edit}>{ "Edit Metadata" }</button>
        </div>
    }
}

fn render_script_text(script_text: &Option<String>) -> Html {
    match script_text {
        Some(text) if !text.is_empty() => html! {
            <div class="scene-script">
                <h3>{ "Script" }</h3>
                <pre class="script-text">{ text }</pre>
            </div>
        },
        _ => html! {
            <div class="scene-script">
                <h3>{ "Script" }</h3>
                <p class="no-script">{ "No script content yet." }</p>
            </div>
        },
    }
}
