pub mod app;
pub mod components;
pub mod pages;
pub mod router;

use app::App;

fn main() {
    yew::start_app::<App>();
}
