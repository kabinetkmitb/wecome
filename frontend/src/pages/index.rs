use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use crate::components::pages::index::competitions::index::Competitions;
use crate::components::pages::index::hero::Hero;
use crate::components::pages::index::kategori::Kategori;
use crate::components::pages::index::register_event::RegisterEvent;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
	html! {
		<main class="max-w-screen overflow-x-hidden">
			<Navbar/>
			<Hero/>
			<Competitions/>
			<Kategori/>
			<RegisterEvent/>
			<Footer/>
		</main>
	}
}
