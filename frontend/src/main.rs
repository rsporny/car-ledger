use yew::Renderer;

mod app;
mod routes;
mod components;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}