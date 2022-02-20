use crate::components::common::footer::index::Footer;
use crate::components::common::navbar::Navbar;
use crate::components::pages::admin::index::AdminComponent;
use yew::prelude::*;

#[function_component(Admin)]
pub fn admin() -> Html {
	html! {
		<main>
			<Navbar/>
			<AdminComponent/>
			<Footer/>
		</main>
	}
}
