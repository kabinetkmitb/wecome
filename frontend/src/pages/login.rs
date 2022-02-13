use crate::components::pages::login::index::LoginComponent;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
	html! {
		<main>
			<LoginComponent/>
		</main>
	}
}
