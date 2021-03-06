use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use crate::components::pages::daftar_lomba::index::DaftarLombaComponent;
use yew::prelude::*;

#[function_component(DaftarLomba)]
pub fn daftar_lomba() -> Html {
	crate::utils::hooks::scroll_to_top();
	html! {
		<main>
			<Navbar/>
			<DaftarLombaComponent/>
			<Footer/>
		</main>
	}
}
