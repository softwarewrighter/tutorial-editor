use yew::prelude::*;

const COPYRIGHT: &str = "Copyright (c) 2025 Michael A Wright";
const LICENSE: &str = "MIT License";
const REPOSITORY: &str = "https://github.com/softwarewrighter/tutorial-editor";
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");
const BUILD_HOST: &str = env!("BUILD_HOST");
const BUILD_TIME: &str = env!("BUILD_TIME");

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <p>{ COPYRIGHT } { " | " } { LICENSE }</p>
            <p>
                { "Repository: " }
                <a href={REPOSITORY} target="_blank">{ REPOSITORY }</a>
            </p>
            <p>{ format!("Build: {} @ {} ({})", BUILD_COMMIT, BUILD_HOST, BUILD_TIME) }</p>
        </footer>
    }
}
