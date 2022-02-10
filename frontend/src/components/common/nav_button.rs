use crate::router::Route;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub route: Route,
	pub pathname: String,
	pub class: String,
}

#[function_component(NavButton)]
pub fn nav_button(props: &Props) -> Html {
	html! {
		<div class={props.class.clone()}>
			<Link<Route> to={props.route.clone()}>{props.pathname.clone()}</Link<Route>>
		</div>
	}
}
