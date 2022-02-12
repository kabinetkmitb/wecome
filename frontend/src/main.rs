pub mod app;
pub mod components;
pub mod pages;
pub mod router;
pub mod utils;

use app::App;

fn main() {
    yew::start_app::<App>();
}
