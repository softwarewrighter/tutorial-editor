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
    let editing = use_state(|| None::<AssetDto>);
    let show_form = use_state(|| false);
    let refresh = use_state(|| 0u32);
    let (sid, pid) = (props.scene.id.unwrap_or(0), props.scene.project_id);
    { let a = assets.clone(); let r = *refresh;
      use_effect_with((sid, r), move |_| {
          wasm_bindgen_futures::spawn_local(async move { if let Ok(l) = fetch_scene_assets(sid).await { a.set(l); } });
      }); }
    let on_edit = { let s = props.scene.clone(); let cb = props.on_edit.clone(); Callback::from(move |_| cb.emit(s.clone())) };
    let cbs = build_asset_callbacks(&editing, &show_form, &refresh, pid, sid);
    html! {
        <div class="scene-detail">
            { render_metadata(&props.scene, on_edit) }
            { render_script_text(&props.scene.script_text) }
            { render_assets_section(&assets, &editing, *show_form, &cbs, pid, Some(sid)) }
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

fn render_assets_section(assets: &[AssetDto], edit: &Option<AssetDto>, show: bool, cb: &AssetCallbacks, pid: i64, sid: Option<i64>) -> Html {
    if show {
        html! { <AssetForm asset={edit.clone()} project_id={pid} scene_id={sid} on_save={cb.on_save.clone()} on_cancel={cb.on_cancel.clone()} /> }
    } else {
        html! { <AssetList assets={assets.to_vec()} on_add={cb.on_add.clone()} on_edit={cb.on_edit.clone()} on_delete={cb.on_delete.clone()} /> }
    }
}
