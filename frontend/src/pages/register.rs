use crate::components::pages::register::index::RegisterComponent;
use yew::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
	html! {
		<main>
			<RegisterComponent/>
		</main>
	}
}
