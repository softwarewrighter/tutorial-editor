use ui_core::AssetDto;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AssetListProps {
    pub assets: Vec<AssetDto>,
    pub on_add: Callback<()>,
    pub on_edit: Callback<AssetDto>,
    pub on_delete: Callback<i64>,
}

#[function_component(AssetList)]
pub fn asset_list(props: &AssetListProps) -> Html {
    let on_add_click = {
        let on_add = props.on_add.clone();
        Callback::from(move |_| on_add.emit(()))
    };

    html! {
        <div class="asset-list">
            <div class="asset-list-header">
                <h3>{ "Assets" }</h3>
                <button class="add-asset-btn" onclick={on_add_click}>{ "+ Add Asset" }</button>
            </div>
            { render_assets(&props.assets, &props.on_edit, &props.on_delete) }
        </div>
    }
}

fn render_assets(
    assets: &[AssetDto],
    on_edit: &Callback<AssetDto>,
    on_delete: &Callback<i64>,
) -> Html {
    if assets.is_empty() {
        return html! {
            <p class="no-assets">{ "No assets attached to this scene." }</p>
        };
    }

    html! {
        <ul class="assets">
            { for assets.iter().map(|a| render_asset_item(a, on_edit, on_delete)) }
        </ul>
    }
}

fn render_asset_item(
    asset: &AssetDto,
    on_edit: &Callback<AssetDto>,
    on_delete: &Callback<i64>,
) -> Html {
    let asset_id = asset.id.unwrap_or(0);
    let edit_asset = asset.clone();
    let on_edit = on_edit.clone();
    let on_delete = on_delete.clone();

    let on_edit_click = Callback::from(move |_| on_edit.emit(edit_asset.clone()));
    let on_delete_click = Callback::from(move |_| on_delete.emit(asset_id));

    let type_badge = format!("asset-type-{}", asset.asset_type.to_lowercase());

    html! {
        <li class="asset-item">
            <div class="asset-info">
                <span class={classes!("asset-type-badge", type_badge)}>{ &asset.asset_type }</span>
                <span class="asset-name">{ &asset.name }</span>
                { render_asset_location(asset) }
            </div>
            <div class="asset-actions">
                <button class="edit-btn" onclick={on_edit_click}>{ "Edit" }</button>
                <button class="delete-btn" onclick={on_delete_click}>{ "Delete" }</button>
            </div>
        </li>
    }
}

fn render_asset_location(asset: &AssetDto) -> Html {
    if let Some(ref url) = asset.url {
        html! { <span class="asset-url">{ url }</span> }
    } else if let Some(ref path) = asset.file_path {
        html! { <span class="asset-path">{ path }</span> }
    } else {
        html! {}
    }
}
