use super::desktop_view::DesktopView;
use super::mobile_view::MobileView;
use crate::utils::hooks::get_window_size;
use yew::prelude::*;

#[function_component(RegistrationSuccessComponent)]
pub fn registration_success_component() -> Html {
	let window_size = get_window_size();
	crate::utils::interop::use_lottie();

	if window_size.width > 900.0 {
		html! {
			<DesktopView/>
		}
	} else {
		html! {
			<MobileView/>
		}
	}
}
