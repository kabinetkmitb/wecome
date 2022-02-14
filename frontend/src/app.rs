use crate::router::{switch, Route};
use crate::utils::interop::ResourceProvider;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ResourceProvider>
            <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ResourceProvider>
    }
}
