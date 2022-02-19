use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use crate::components::pages::daftar_lomba_success::DaftarLombaSuccessComponent;
use yew::prelude::*;

#[function_component(DaftarLombaSuccess)]
pub fn daftar_lomba_sukses() -> Html {
	html! {
		<main>
			<Navbar/>
			<DaftarLombaSuccessComponent/>
			<Footer/>
		</main>
	}
}
