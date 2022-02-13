use crate::components::pages::register::index::RegisterComponent;
use yew::prelude::*;

#[function_component(Register)]
pub fn profile() -> Html {
	html! {
		<main>
			<RegisterComponent/>
		</main>
	}
}
