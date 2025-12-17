use ui_core::ProjectDto;
use yew::prelude::*;

use crate::footer::Footer;
use crate::project_list::ProjectList;

#[function_component(App)]
pub fn app() -> Html {
    let projects = use_state(Vec::<ProjectDto>::new);

    {
        let projects = projects.clone();
        use_effect_with((), move |_| {
            // Placeholder: in a next iteration, fetch from `/api/projects`.
            projects.set(vec![]);
            || ()
        });
    }

    html! {
        <main class="app-root">
            <Header />
            <section>
                <h2>{ "Projects" }</h2>
                <ProjectList projects={(*projects).clone()} />
            </section>
            <Footer />
        </main>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
            <h1>{ "Avatar Video Orchestrator - UI Workbench" }</h1>
            <p>{ "Rust/Yew/WASM editor shell - wire up API calls next." }</p>
        </header>
    }
}
