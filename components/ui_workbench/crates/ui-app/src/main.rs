use wasm_bindgen::prelude::*;
use yew::Renderer;

mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<app::App>::new().render();
}
