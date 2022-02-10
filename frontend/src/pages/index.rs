use crate::components::common::navbar::Navbar;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
	html! {
		<main>
			<Navbar/>
		</main>
	}
}
