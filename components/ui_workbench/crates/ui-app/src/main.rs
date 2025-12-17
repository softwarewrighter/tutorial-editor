use wasm_bindgen::prelude::*;
use yew::Renderer;

mod app;
mod footer;
mod project_list;

#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<app::App>::new().render();
}

fn main() {
    // Entry point for wasm-bindgen; actual startup in run_app()
}
