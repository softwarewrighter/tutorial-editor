use ui_core::SceneDto;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneListProps {
    pub scenes: Vec<SceneDto>,
}

#[function_component(SceneList)]
pub fn scene_list(props: &SceneListProps) -> Html {
    if props.scenes.is_empty() {
        html! {
            <p>{ "No scenes yet. Create one to get started." }</p>
        }
    } else {
        html! {
            <ul class="scene-list">
                { for props.scenes.iter().map(render_scene_item) }
            </ul>
        }
    }
}

fn render_scene_item(scene: &SceneDto) -> Html {
    let id = scene.id.unwrap_or(0);
    html! {
        <li key={id} class="scene-item">
            <span class="scene-order">{ format!("#{}", scene.sort_order) }</span>
            <span class="scene-title">{ &scene.title }</span>
            { render_description(&scene.description) }
        </li>
    }
}

fn render_description(desc: &Option<String>) -> Html {
    match desc {
        Some(d) if !d.is_empty() => html! { <span class="scene-desc">{ d }</span> },
        _ => html! {},
    }
}
