use wasm_bindgen::prelude::*;
use yew::Renderer;

mod api;
mod app;
mod asset_api;
mod asset_callbacks;
mod asset_form;
mod asset_list;
mod footer;
mod project_form;
mod project_list;
mod scene_detail;
mod scene_edit_form;
mod scene_form;
mod scene_list;

#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<app::App>::new().render();
}

fn main() {
    // Entry point for wasm-bindgen; actual startup in run_app()
}
