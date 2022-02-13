use crate::pages::{index::Index, login::Login, profile::Profile, register::Register};
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
	#[at("/login")]
	Login,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: &Route) -> Html {
	match routes {
		Route::Index => html! { <Index/> },
		Route::Profile => html! { <Profile/> },
		Route::Register => html! { <Register/> },
		Route::Login => html! { <Login/> },
		Route::NotFound => html! { <h1>{ "404" }</h1> },
	}
}
