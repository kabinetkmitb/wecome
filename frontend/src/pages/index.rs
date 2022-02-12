use crate::components::common::navbar::Navbar;
use crate::components::pages::index::competitions::index::Competitions;
use crate::components::pages::index::footer::index::Footer;
use crate::components::pages::index::hero::Hero;
use crate::components::pages::index::register_event::RegisterEvent;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
	html! {
		<main class="w-screen overflow-x-hidden">
			<Navbar/>
			<Hero/>
			<Competitions/>
			<RegisterEvent/>
			<Footer/>
		</main>
	}
}
