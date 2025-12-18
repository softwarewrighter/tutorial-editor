use ui_core::AssetDto;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AssetFormProps {
    pub asset: Option<AssetDto>,
    pub project_id: i64,
    pub scene_id: Option<i64>,
    pub on_save: Callback<AssetDto>,
    pub on_cancel: Callback<()>,
}

struct FormState {
    name: UseStateHandle<String>,
    asset_type: UseStateHandle<String>,
    url: UseStateHandle<String>,
    file_path: UseStateHandle<String>,
}

#[function_component(AssetForm)]
pub fn asset_form(props: &AssetFormProps) -> Html {
    let state = FormState {
        name: use_state(|| props.asset.as_ref().map(|a| a.name.clone()).unwrap_or_default()),
        asset_type: use_state(|| props.asset.as_ref().map(|a| a.asset_type.clone()).unwrap_or_else(|| "video".to_string())),
        url: use_state(|| props.asset.as_ref().and_then(|a| a.url.clone()).unwrap_or_default()),
        file_path: use_state(|| props.asset.as_ref().and_then(|a| a.file_path.clone()).unwrap_or_default()),
    };

    let on_submit = build_submit_callback(props, &state);
    let title = if props.asset.is_some() { "Edit Asset" } else { "Add Asset" };

    html! {
        <form class="asset-form" onsubmit={on_submit}>
            <h3>{ title }</h3>
            { render_name_field(&state.name) }
            { render_type_field(&state.asset_type) }
            { render_url_field(&state.url) }
            { render_file_path_field(&state.file_path) }
            { render_form_actions(&props.on_cancel) }
        </form>
    }
}

fn build_submit_callback(props: &AssetFormProps, state: &FormState) -> Callback<SubmitEvent> {
    let asset = props.asset.clone();
    let project_id = props.project_id;
    let scene_id = props.scene_id;
    let name = state.name.clone();
    let asset_type = state.asset_type.clone();
    let url = state.url.clone();
    let file_path = state.file_path.clone();
    let on_save = props.on_save.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let updated = AssetDto {
            id: asset.as_ref().and_then(|a| a.id),
            project_id,
            scene_id,
            name: (*name).clone(),
            asset_type: (*asset_type).clone(),
            url: Some((*url).clone()).filter(|s| !s.is_empty()),
            file_path: Some((*file_path).clone()).filter(|s| !s.is_empty()),
            metadata: asset.as_ref().and_then(|a| a.metadata.clone()),
        };
        on_save.emit(updated);
    })
}

fn render_name_field(name: &UseStateHandle<String>) -> Html {
    let oninput = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                name.set(input.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="asset-name">{ "Name" }</label>
            <input type="text" id="asset-name" value={name.to_string()} {oninput} required=true />
        </div>
    }
}

fn render_type_field(asset_type: &UseStateHandle<String>) -> Html {
    let onchange = {
        let asset_type = asset_type.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                asset_type.set(input.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="asset-type">{ "Type" }</label>
            <select id="asset-type" onchange={onchange}>
                <option value="video" selected={&**asset_type == "video"}>{ "Video" }</option>
                <option value="audio" selected={&**asset_type == "audio"}>{ "Audio" }</option>
                <option value="image" selected={&**asset_type == "image"}>{ "Image" }</option>
                <option value="reference" selected={&**asset_type == "reference"}>{ "Reference" }</option>
                <option value="script" selected={&**asset_type == "script"}>{ "Script" }</option>
            </select>
        </div>
    }
}

fn render_url_field(url: &UseStateHandle<String>) -> Html {
    let oninput = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                url.set(input.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="asset-url">{ "URL (optional)" }</label>
            <input type="text" id="asset-url" value={url.to_string()} {oninput} />
        </div>
    }
}

fn render_file_path_field(file_path: &UseStateHandle<String>) -> Html {
    let oninput = {
        let file_path = file_path.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                file_path.set(input.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="asset-path">{ "File Path (optional)" }</label>
            <input type="text" id="asset-path" value={file_path.to_string()} {oninput} />
        </div>
    }
}

fn render_form_actions(on_cancel: &Callback<()>) -> Html {
    let on_cancel_click = {
        let on_cancel = on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };
    html! {
        <div class="form-actions">
            <button type="submit" class="save-btn">{ "Save" }</button>
            <button type="button" class="cancel-btn" onclick={on_cancel_click}>{ "Cancel" }</button>
        </div>
    }
}
