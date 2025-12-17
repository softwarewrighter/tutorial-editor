use ui_core::ProjectDto;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectListProps {
    pub projects: Vec<ProjectDto>,
}

#[function_component(ProjectList)]
pub fn project_list(props: &ProjectListProps) -> Html {
    if props.projects.is_empty() {
        html! {
            <p>{ "No projects yet. Create one via HTTP API or a future UI form." }</p>
        }
    } else {
        html! {
            <ul>
                { for props.projects.iter().map(|p| html! {
                    <li key={p.slug.clone()}>
                        { format!("{} - {}", p.slug, p.title) }
                    </li>
                }) }
            </ul>
        }
    }
}
