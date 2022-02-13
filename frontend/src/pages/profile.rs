use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use crate::components::pages::profile::header::Header;
use crate::components::pages::profile::organogram::Organogram;
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
	html! {
		<main>
			<Navbar/>
			<Header/>
			<Organogram/>
			<Footer/>
		</main>
	}
}
