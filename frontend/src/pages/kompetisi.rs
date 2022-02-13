use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use yew::prelude::*;

#[function_component(Kompetisi)]
pub fn kompetisi() -> Html {
	html! {
		<main class="max-w-screen overflow-x-hidden">
			<Navbar/>
			<Footer/>
		</main>
	}
}
