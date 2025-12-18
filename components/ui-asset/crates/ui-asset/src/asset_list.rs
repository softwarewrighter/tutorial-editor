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

fn render_asset_item(a: &AssetDto, on_edit: &Callback<AssetDto>, on_del: &Callback<i64>) -> Html {
    let (id, ea) = (a.id.unwrap_or(0), a.clone());
    let (oe, od) = (on_edit.clone(), on_del.clone());
    let ec = Callback::from(move |_| oe.emit(ea.clone()));
    let dc = Callback::from(move |_| od.emit(id));
    let tb = format!("asset-type-{}", a.asset_type.to_lowercase());
    html! {
        <li class="asset-item">
            <div class="asset-info">
                <span class={classes!("asset-type-badge", tb)}>{ &a.asset_type }</span>
                <span class="asset-name">{ &a.name }</span>
                { render_asset_location(a) }
            </div>
            <div class="asset-actions">
                <button class="edit-btn" onclick={ec}>{ "Edit" }</button>
                <button class="delete-btn" onclick={dc}>{ "Delete" }</button>
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
