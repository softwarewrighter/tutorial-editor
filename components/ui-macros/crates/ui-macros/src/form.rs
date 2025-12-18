//! Form field macros for Yew applications

/// Creates a text input field with state binding
///
/// # Example
/// ```ignore
/// let name = use_state(String::new);
/// html! { { text_field!(name, "Name", "field-name") } }
/// ```
#[macro_export]
macro_rules! text_field {
    ($state:expr, $label:expr, $id:expr) => {{
        use wasm_bindgen::JsCast;
        let oninput = {
            let state = $state.clone();
            yew::Callback::from(move |e: yew::InputEvent| {
                if let Some(input) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    state.set(input.value());
                }
            })
        };
        yew::html! {
            <div class="form-field">
                <label for={$id}>{ $label }</label>
                <input type="text" id={$id} value={$state.to_string()} {oninput} />
            </div>
        }
    }};
    ($state:expr, $label:expr, $id:expr, required) => {{
        use wasm_bindgen::JsCast;
        let oninput = {
            let state = $state.clone();
            yew::Callback::from(move |e: yew::InputEvent| {
                if let Some(input) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    state.set(input.value());
                }
            })
        };
        yew::html! {
            <div class="form-field">
                <label for={$id}>{ $label }</label>
                <input type="text" id={$id} value={$state.to_string()} {oninput} required=true />
            </div>
        }
    }};
}

/// Creates a textarea field with state binding
///
/// # Example
/// ```ignore
/// let description = use_state(String::new);
/// html! { { textarea_field!(description, "Description", "field-desc") } }
/// ```
#[macro_export]
macro_rules! textarea_field {
    ($state:expr, $label:expr, $id:expr) => {{
        use wasm_bindgen::JsCast;
        let oninput = {
            let state = $state.clone();
            yew::Callback::from(move |e: yew::InputEvent| {
                if let Some(input) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
                {
                    state.set(input.value());
                }
            })
        };
        yew::html! {
            <div class="form-field">
                <label for={$id}>{ $label }</label>
                <textarea id={$id} value={$state.to_string()} {oninput} />
            </div>
        }
    }};
    ($state:expr, $label:expr, $id:expr, rows = $rows:expr) => {{
        use wasm_bindgen::JsCast;
        let oninput = {
            let state = $state.clone();
            yew::Callback::from(move |e: yew::InputEvent| {
                if let Some(input) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
                {
                    state.set(input.value());
                }
            })
        };
        yew::html! {
            <div class="form-field">
                <label for={$id}>{ $label }</label>
                <textarea id={$id} rows={$rows.to_string()} value={$state.to_string()} {oninput} />
            </div>
        }
    }};
}

/// Creates a number input field with state binding
///
/// # Example
/// ```ignore
/// let sort_order = use_state(|| 0i32);
/// html! { { number_field!(sort_order, "Sort Order", "field-sort") } }
/// ```
#[macro_export]
macro_rules! number_field {
    ($state:expr, $label:expr, $id:expr) => {{
        use wasm_bindgen::JsCast;
        let oninput = {
            let state = $state.clone();
            yew::Callback::from(move |e: yew::InputEvent| {
                if let Some(input) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    if let Ok(val) = input.value().parse() {
                        state.set(val);
                    }
                }
            })
        };
        yew::html! {
            <div class="form-field">
                <label for={$id}>{ $label }</label>
                <input type="number" id={$id} value={$state.to_string()} {oninput} />
            </div>
        }
    }};
}

/// Creates form action buttons (Save and Cancel)
///
/// # Example
/// ```ignore
/// let on_cancel = Callback::from(|_| {});
/// html! { { form_actions!(on_cancel) } }
/// ```
#[macro_export]
macro_rules! form_actions {
    ($on_cancel:expr) => {{
        let on_cancel_click = {
            let on_cancel = $on_cancel.clone();
            yew::Callback::from(move |_| on_cancel.emit(()))
        };
        yew::html! {
            <div class="form-actions">
                <button type="submit" class="save-btn">{ "Save" }</button>
                <button type="button" class="cancel-btn" onclick={on_cancel_click}>{ "Cancel" }</button>
            </div>
        }
    }};
    ($on_cancel:expr, $save_label:expr) => {{
        let on_cancel_click = {
            let on_cancel = $on_cancel.clone();
            yew::Callback::from(move |_| on_cancel.emit(()))
        };
        yew::html! {
            <div class="form-actions">
                <button type="submit" class="save-btn">{ $save_label }</button>
                <button type="button" class="cancel-btn" onclick={on_cancel_click}>{ "Cancel" }</button>
            </div>
        }
    }};
}
