pub mod app;
pub mod components;
pub mod pages;
pub mod router;
pub mod types;
pub mod utils;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
