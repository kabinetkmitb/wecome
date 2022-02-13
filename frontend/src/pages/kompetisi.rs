use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use crate::components::pages::kompetisi::index::KompetisiComponent;
use yew::prelude::*;

#[function_component(Kompetisi)]
pub fn kompetisi() -> Html {
	html! {
		<main class="max-w-screen overflow-x-hidden">
			<Navbar/>
			<KompetisiComponent />
			<Footer/>
		</main>
	}
}