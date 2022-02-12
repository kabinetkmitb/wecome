use super::desktop_view::DesktopView;
use super::mobile_view::MobileView;
use crate::utils::hooks::get_window_size;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
	let window_size = get_window_size();

	if window_size.width > 650.0 {
		html! {
			<DesktopView />
		}
	} else {
		html! {
			<MobileView />
		}
	}
}
