use ui_core::AssetDto;
use yew::prelude::*;

use crate::asset_api;

pub struct AssetCallbacks {
    pub on_add: Callback<()>,
    pub on_edit: Callback<AssetDto>,
    pub on_delete: Callback<i64>,
    pub on_save: Callback<AssetDto>,
    pub on_cancel: Callback<()>,
}

pub fn build_asset_callbacks(
    editing_asset: &UseStateHandle<Option<AssetDto>>,
    show_form: &UseStateHandle<bool>,
    refresh: &UseStateHandle<u32>,
    project_id: i64,
    scene_id: i64,
) -> AssetCallbacks {
    AssetCallbacks {
        on_add: build_on_add(show_form),
        on_edit: build_on_edit(editing_asset, show_form),
        on_delete: build_on_delete(refresh),
        on_save: build_on_save(editing_asset, show_form, refresh, project_id, scene_id),
        on_cancel: build_on_cancel(editing_asset, show_form),
    }
}

fn build_on_add(show_form: &UseStateHandle<bool>) -> Callback<()> {
    let show_form = show_form.clone();
    Callback::from(move |_| show_form.set(true))
}

fn build_on_edit(
    editing_asset: &UseStateHandle<Option<AssetDto>>,
    show_form: &UseStateHandle<bool>,
) -> Callback<AssetDto> {
    let editing_asset = editing_asset.clone();
    let show_form = show_form.clone();
    Callback::from(move |asset: AssetDto| {
        editing_asset.set(Some(asset));
        show_form.set(true);
    })
}

fn build_on_delete(refresh: &UseStateHandle<u32>) -> Callback<i64> {
    let refresh = refresh.clone();
    Callback::from(move |id: i64| {
        let refresh = refresh.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if asset_api::delete_asset(id).await.is_ok() {
                refresh.set(*refresh + 1);
            }
        });
    })
}

fn build_on_save(
    editing_asset: &UseStateHandle<Option<AssetDto>>,
    show_form: &UseStateHandle<bool>,
    refresh: &UseStateHandle<u32>,
    project_id: i64,
    scene_id: i64,
) -> Callback<AssetDto> {
    let editing_asset = editing_asset.clone();
    let show_form = show_form.clone();
    let refresh = refresh.clone();
    Callback::from(move |asset: AssetDto| {
        let editing_asset = editing_asset.clone();
        let show_form = show_form.clone();
        let refresh = refresh.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = if asset.id.is_some() {
                asset_api::update_asset(
                    asset.id.unwrap(), &asset.name, &asset.asset_type,
                    asset.file_path.as_deref(), asset.url.as_deref(), asset.metadata.as_deref(),
                ).await
            } else {
                asset_api::create_asset(
                    project_id, Some(scene_id), &asset.name, &asset.asset_type,
                    asset.file_path.as_deref(), asset.url.as_deref(), asset.metadata.as_deref(),
                ).await
            };
            if result.is_ok() {
                editing_asset.set(None);
                show_form.set(false);
                refresh.set(*refresh + 1);
            }
        });
    })
}

fn build_on_cancel(
    editing_asset: &UseStateHandle<Option<AssetDto>>,
    show_form: &UseStateHandle<bool>,
) -> Callback<()> {
    let editing_asset = editing_asset.clone();
    let show_form = show_form.clone();
    Callback::from(move |_| {
        editing_asset.set(None);
        show_form.set(false);
    })
}
