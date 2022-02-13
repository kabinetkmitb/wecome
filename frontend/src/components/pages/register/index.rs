use super::desktop_view::DesktopView;
use super::mobile_view::MobileView;
use crate::utils::hooks::get_window_size;
use yew::prelude::*;

#[function_component(RegisterComponent)]
pub fn register_component() -> Html {
	let window_size = get_window_size();

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
