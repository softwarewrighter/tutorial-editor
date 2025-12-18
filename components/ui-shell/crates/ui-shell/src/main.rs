use ui_app::App;
use wasm_bindgen::prelude::*;
use yew::Renderer;

#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<App>::new().render();
}

fn main() {
    // Entry point for wasm-bindgen; actual startup in run_app()
}
