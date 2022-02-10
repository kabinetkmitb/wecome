use crate::components::common::navbar::Navbar;
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
	html! {
		<main>
			<Navbar/>
		</main>
	}
}
