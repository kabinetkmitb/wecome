use super::mobile_view::MobileView;
use crate::utils::hooks::get_window_size;
use yew::prelude::*;

#[function_component(LoginComponent)]
pub fn login_component() -> Html {
	let window_size = get_window_size();

	if window_size.width > 900.0 {
		html! {
			<></>
		}
	} else {
		html! {
			<MobileView/>
		}
	}
}
