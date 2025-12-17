use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectFormProps {
    pub on_submit: Callback<(String, String)>,
    pub on_cancel: Callback<()>,
}

#[function_component(ProjectForm)]
pub fn project_form(props: &ProjectFormProps) -> Html {
    let slug = use_state(String::new);
    let title = use_state(String::new);

    let on_form_submit = {
        let slug = slug.clone();
        let title = title.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit(((*slug).clone(), (*title).clone()));
        })
    };

    html! {
        <form class="project-form" onsubmit={on_form_submit}>
            <h3>{ "Create Project" }</h3>
            { render_slug_field(&slug) }
            { render_title_field(&title) }
            { render_form_actions(&props.on_cancel) }
        </form>
    }
}

fn render_slug_field(slug: &UseStateHandle<String>) -> Html {
    let oninput = {
        let slug = slug.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                slug.set(input.value());
            }
        })
    };
    html! {
        <div class="form-field">
            <label for="slug">{ "Slug" }</label>
            <input type="text" id="slug" value={slug.to_string()} {oninput} required=true />
        </div>
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
