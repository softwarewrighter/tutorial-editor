use ui_core::{ProjectDto, SceneDto};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api;
use crate::footer::Footer;
use crate::project_list::ProjectList;
use crate::scene_list::SceneList;

#[function_component(App)]
pub fn app() -> Html {
    let projects = use_state(Vec::<ProjectDto>::new);
    let scenes = use_state(Vec::<SceneDto>::new);
    let selected_project = use_state(|| None::<i64>);

    {
        let projects = projects.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(data) = api::fetch_projects().await {
                    projects.set(data);
                }
            });
            || ()
        });
    }

    {
        let scenes = scenes.clone();
        let project_id = *selected_project;
        use_effect_with(project_id, move |pid| {
            if let Some(id) = *pid {
                spawn_local(async move {
                    if let Ok(data) = api::fetch_scenes(id).await {
                        scenes.set(data);
                    }
                });
            } else {
                scenes.set(vec![]);
            }
            || ()
        });
    }

    let on_select = {
        let selected_project = selected_project.clone();
        Callback::from(move |id: i64| {
            selected_project.set(Some(id));
        })
    };

    html! {
        <main class="app-root">
            <Header />
            <section>
                <h2>{ "Projects" }</h2>
                <ProjectList projects={(*projects).clone()} on_select={on_select} />
            </section>
            { render_scenes(&selected_project, &scenes) }
            <Footer />
        </main>
    }
}

fn render_scenes(selected: &Option<i64>, scenes: &[SceneDto]) -> Html {
    match selected {
        Some(_) => html! {
            <section>
                <h2>{ "Scenes" }</h2>
                <SceneList scenes={scenes.to_vec()} />
            </section>
        },
        None => html! {},
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
            <h1>{ "Avatar Video Orchestrator - UI Workbench" }</h1>
            <p>{ "Select a project to view its scenes." }</p>
        </header>
    }
}
