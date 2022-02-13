use crate::pages::{
	index::Index, kompetisi::Kompetisi, login::Login, profile::Profile, register::Register,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
	#[at("/")]
	Index,
	#[at("/profile")]
	Profile,
	#[at("/register")]
	Register,
	#[at("/login")]
	Login,
	#[at("/kompetisi?search=:search&category=:category")]
	Kompetisi { search: String, category: String },
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: &Route) -> Html {
	match routes {
		Route::Index => html! { <Index/> },
		Route::Profile => html! { <Profile/> },
		Route::Kompetisi { search, category } => html! { <Kompetisi/> },
		Route::Register => html! { <Register/> },
		Route::Login => html! { <Login/> },
		Route::NotFound => html! { <h1>{ "404" }</h1> },
	}
}
