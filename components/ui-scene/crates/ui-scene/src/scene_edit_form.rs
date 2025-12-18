use ui_core::SceneDto;
use ui_macros::{form_actions, number_field, text_field, textarea_field};
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
    let desc = use_state(|| props.scene.description.clone().unwrap_or_default());
    let order = use_state(|| props.scene.sort_order);
    let on_submit = {
        let (s, t, d, o, cb) = (props.scene.clone(), title.clone(), desc.clone(), *order, props.on_save.clone());
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            cb.emit(SceneDto { title: (*t).clone(), description: Some((*d).clone()).filter(|x| !x.is_empty()), sort_order: o, ..s.clone() });
        })
    };
    html! {
        <form class="scene-edit-form" onsubmit={on_submit}>
            { text_field!(title, "Title", "title", required) }
            { textarea_field!(desc, "Description", "description") }
            { number_field!(order, "Sort Order", "sort_order") }
            { form_actions!(props.on_cancel) }
        </form>
    }
}
