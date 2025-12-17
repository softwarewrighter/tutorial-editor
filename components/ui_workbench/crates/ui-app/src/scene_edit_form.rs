use ui_core::SceneDto;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneEditFormProps {
    pub scene: SceneDto,
    pub on_save: Callback<SceneDto>,
    pub on_cancel: Callback<()>,
}

#[function_component(SceneEditForm)]
pub fn scene_edit_form(props: &SceneEditFormProps) -> Html {
    let title = use_state(|| props.scene.title.clone());
    let description = use_state(|| props.scene.description.clone().unwrap_or_default());
    let sort_order = use_state(|| props.scene.sort_order);

    let on_submit = {
        let scene = props.scene.clone();
        let title = title.clone();
        let description = description.clone();
        let sort_order = *sort_order;
        let on_save = props.on_save.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let updated = SceneDto {
                title: (*title).clone(),
                description: Some((*description).clone()).filter(|s| !s.is_empty()),
                sort_order,
                ..scene.clone()
            };
            on_save.emit(updated);
        })
    };

    html! {
        <form class="scene-edit-form" onsubmit={on_submit}>
            { render_title_field(&title) }
            { render_description_field(&description) }
            { render_sort_order_field(&sort_order) }
            { render_form_actions(&props.on_cancel) }
        </form>
    }
}

fn render_title_field(title: &UseStateHandle<String>) -> Html {
    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                title.set(input.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="title">{ "Title" }</label>
            <input type="text" id="title" value={title.to_string()} {oninput} required=true />
        </div>
    }
}

fn render_description_field(description: &UseStateHandle<String>) -> Html {
    let oninput = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(ta) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                description.set(ta.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="description">{ "Description" }</label>
            <textarea id="description" value={description.to_string()} {oninput} />
        </div>
    }
}

fn render_sort_order_field(sort_order: &UseStateHandle<i32>) -> Html {
    let oninput = {
        let sort_order = sort_order.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                && let Ok(val) = input.value().parse::<i32>()
            {
                sort_order.set(val);
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="sort_order">{ "Sort Order" }</label>
            <input type="number" id="sort_order" value={sort_order.to_string()} {oninput} />
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
