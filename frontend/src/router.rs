use crate::pages::{index::Index, profile::Profile, register::Register};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
	#[at("/")]
	Index,
	#[at("/profile")]
	Profile,
	#[at("/register")]
	Register,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: &Route) -> Html {
	match routes {
		Route::Index => html! { <Index/> },
		Route::Profile => html! { <Profile/> },
		Route::Register => html! { <Register/> },
		Route::NotFound => html! { <h1>{ "404" }</h1> },
	}
}
