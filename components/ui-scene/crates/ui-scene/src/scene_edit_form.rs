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
            { text_field!(title, "Title", "title", required) }
            { textarea_field!(description, "Description", "description") }
            { number_field!(sort_order, "Sort Order", "sort_order") }
            { form_actions!(props.on_cancel) }
        </form>
    }
}
