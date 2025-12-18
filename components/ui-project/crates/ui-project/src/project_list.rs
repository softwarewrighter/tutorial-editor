use ui_core::ProjectDto;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectListProps {
    pub projects: Vec<ProjectDto>,
    pub on_select: Callback<i64>,
}

#[function_component(ProjectList)]
pub fn project_list(props: &ProjectListProps) -> Html {
    if props.projects.is_empty() {
        html! {
            <p>{ "No projects yet. Create one via HTTP API or a future UI form." }</p>
        }
    } else {
        html! {
            <ul class="project-list">
                { for props.projects.iter().map(|p| render_project_item(p, &props.on_select)) }
            </ul>
        }
    }
}

fn render_project_item(project: &ProjectDto, on_select: &Callback<i64>) -> Html {
    let id = project.id.unwrap_or(0);
    let on_click = {
        let on_select = on_select.clone();
        Callback::from(move |_| on_select.emit(id))
    };
    html! {
        <li key={project.slug.clone()} class="project-item" onclick={on_click}>
            <span class="project-slug">{ &project.slug }</span>
            <span class="project-title">{ &project.title }</span>
        </li>
    }
}
