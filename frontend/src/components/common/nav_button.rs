use crate::router::Route;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub route: Route,
	pub pathname: String,
	pub class: String,
	pub path_to_match: Option<String>,
}

fn class_based_on_path_matched(path_to_match: Option<String>, pathname: String) -> String {
	match path_to_match {
		None => String::from(""),
		Some(path) => {
			if path == pathname {
				String::from("border-1 border-yellow-400 border-b-4")
			} else {
				String::from("")
			}
		}
	}
}

#[function_component(NavButton)]
pub fn nav_button(props: &Props) -> Html {
	let location = use_location().unwrap();
	html! {
		<div class={props.class.clone()}>
			<Link<Route>
				to={props.route.clone()}
			>
				<div class={class_based_on_path_matched(props.path_to_match.clone(), location.pathname())}>
					{props.pathname.clone()}
				</div>
			</Link<Route>>
		</div>
	}
}
