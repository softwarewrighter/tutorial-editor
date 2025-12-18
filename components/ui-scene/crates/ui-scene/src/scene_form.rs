use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneFormProps {
    pub on_submit: Callback<(String, i32)>,
    pub on_cancel: Callback<()>,
}

#[function_component(SceneForm)]
pub fn scene_form(props: &SceneFormProps) -> Html {
    let title = use_state(String::new);
    let sort_order = use_state(|| 0i32);

    let on_form_submit = {
        let title = title.clone();
        let sort_order = *sort_order;
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit(((*title).clone(), sort_order));
        })
    };

    html! {
        <form class="scene-form" onsubmit={on_form_submit}>
            <h3>{ "Create Scene" }</h3>
            { render_title_field(&title) }
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
            <label for="scene-title">{ "Title" }</label>
            <input type="text" id="scene-title" value={title.to_string()} {oninput} required=true />
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
            <label for="scene-sort-order">{ "Sort Order" }</label>
            <input type="number" id="scene-sort-order" value={sort_order.to_string()} {oninput} />
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
            <button type="submit" class="submit-btn">{ "Create" }</button>
            <button type="button" class="cancel-btn" onclick={on_cancel_click}>{ "Cancel" }</button>
        </div>
    }
}
