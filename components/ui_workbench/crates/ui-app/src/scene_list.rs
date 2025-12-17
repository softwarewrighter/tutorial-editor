use ui_core::SceneDto;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneListProps {
    pub scenes: Vec<SceneDto>,
    pub on_select: Callback<SceneDto>,
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
                { for props.scenes.iter().map(|s| render_scene_item(s, &props.on_select)) }
            </ul>
        }
    }
}

fn render_scene_item(scene: &SceneDto, on_select: &Callback<SceneDto>) -> Html {
    let id = scene.id.unwrap_or(0);
    let on_click = {
        let scene = scene.clone();
        let on_select = on_select.clone();
        Callback::from(move |_| on_select.emit(scene.clone()))
    };
    html! {
        <li key={id} class="scene-item" onclick={on_click}>
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
