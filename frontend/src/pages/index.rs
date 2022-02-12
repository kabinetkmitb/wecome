use crate::components::common::navbar::Navbar;
use crate::components::pages::index::competitions::index::Competitions;
use crate::components::pages::index::footer::index::Footer;
use crate::components::pages::index::hero::Hero;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
	html! {
		<main>
			<Navbar/>
			<Hero/>
			<Competitions/>
			<Footer/>
		</main>
	}
}
