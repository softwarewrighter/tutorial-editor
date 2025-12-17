use ui_core::ProjectDto;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let projects = use_state(|| Vec::<ProjectDto>::new());

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
            <header>
                <h1>{ "Avatar Video Orchestrator – UI Workbench" }</h1>
                <p>{ "Rust/Yew/WASM editor shell – wire up API calls next." }</p>
            </header>
            <section>
                <h2>{ "Projects" }</h2>
                if projects.is_empty() {
                    <p>{ "No projects yet. Create one via HTTP API or a future UI form." }</p>
                } else {
                    <ul>
                        { for projects.iter().map(|p| html! {
                            <li key={p.slug.clone()}>
                                { format!("{} – {}", p.slug, p.title) }
                            </li>
                        }) }
                    </ul>
                }
            </section>
        </main>
    }
}
