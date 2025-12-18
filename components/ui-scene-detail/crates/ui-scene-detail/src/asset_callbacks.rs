use ui_api::{create_asset, delete_asset, update_asset};
use ui_core::AssetDto;
use ui_macros::{hide_callback, none_callback, set_callback};
use yew::prelude::*;

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
    let cancel_editing = none_callback!(editing_asset);
    let cancel_form = hide_callback!(show_form);
    AssetCallbacks {
        on_add: set_callback!(show_form, true),
        on_edit: build_on_edit(editing_asset, show_form),
        on_delete: build_on_delete(refresh),
        on_save: build_on_save(editing_asset, show_form, refresh, project_id, scene_id),
        on_cancel: {
            Callback::from(move |_| {
                cancel_editing.emit(());
                cancel_form.emit(());
            })
        },
    }
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
            if delete_asset(id).await.is_ok() {
                refresh.set(*refresh + 1);
            }
        });
    })
}

fn build_on_save(ea: &UseStateHandle<Option<AssetDto>>, sf: &UseStateHandle<bool>, rf: &UseStateHandle<u32>, pid: i64, sid: i64) -> Callback<AssetDto> {
    let (ea, sf, rf) = (ea.clone(), sf.clone(), rf.clone());
    Callback::from(move |mut a: AssetDto| {
        let (ea, sf, rf) = (ea.clone(), sf.clone(), rf.clone());
        a.project_id = pid; a.scene_id = Some(sid);
        wasm_bindgen_futures::spawn_local(async move {
            let ok = if a.id.is_some() { update_asset(&a).await } else { create_asset(&a).await }.is_ok();
            if ok { ea.set(None); sf.set(false); rf.set(*rf + 1); }
        });
    })
}

