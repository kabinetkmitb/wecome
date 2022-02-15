use crate::components::pages::registration_success::index::RegistrationSuccessComponent;
use yew::prelude::*;

#[function_component(RegistrationSuccess)]
pub fn profile() -> Html {
	html! {
		<main>
			<RegistrationSuccessComponent/>
		</main>
	}
}
