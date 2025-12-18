use ui_api::fetch_scene_assets;
use ui_asset::{AssetForm, AssetList};
use ui_core::{AssetDto, SceneDto};
use yew::prelude::*;

use crate::asset_callbacks::{build_asset_callbacks, AssetCallbacks};

#[derive(Properties, PartialEq)]
pub struct SceneDetailProps {
    pub scene: SceneDto,
    pub on_edit: Callback<SceneDto>,
}

#[function_component(SceneDetail)]
pub fn scene_detail(props: &SceneDetailProps) -> Html {
    let assets = use_state(Vec::<AssetDto>::new);
    let editing_asset = use_state(|| None::<AssetDto>);
    let show_asset_form = use_state(|| false);
    let refresh = use_state(|| 0u32);

    let scene_id = props.scene.id.unwrap_or(0);
    let project_id = props.scene.project_id;

    {
        let assets = assets.clone();
        let sid = scene_id;
        let refresh_val = *refresh;
        use_effect_with((sid, refresh_val), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(list) = fetch_scene_assets(sid).await {
                    assets.set(list);
                }
            });
        });
    }

    let on_edit_click = {
        let scene = props.scene.clone();
        let on_edit = props.on_edit.clone();
        Callback::from(move |_| on_edit.emit(scene.clone()))
    };

    let cbs = build_asset_callbacks(&editing_asset, &show_asset_form, &refresh, project_id, scene_id);

    html! {
        <div class="scene-detail">
            { render_metadata(&props.scene, on_edit_click) }
            { render_script_text(&props.scene.script_text) }
            { render_assets_section(&assets, &editing_asset, *show_asset_form, &cbs, project_id, Some(scene_id)) }
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

fn render_assets_section(
    assets: &[AssetDto],
    editing_asset: &Option<AssetDto>,
    show_form: bool,
    callbacks: &AssetCallbacks,
    project_id: i64,
    scene_id: Option<i64>,
) -> Html {
    if show_form {
        html! {
            <AssetForm
                asset={editing_asset.clone()}
                project_id={project_id}
                scene_id={scene_id}
                on_save={callbacks.on_save.clone()}
                on_cancel={callbacks.on_cancel.clone()}
            />
        }
    } else {
        html! {
            <AssetList
                assets={assets.to_vec()}
                on_add={callbacks.on_add.clone()}
                on_edit={callbacks.on_edit.clone()}
                on_delete={callbacks.on_delete.clone()}
            />
        }
    }
}
