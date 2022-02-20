use crate::pages::{
	admin::Admin, daftar_lomba::DaftarLomba, daftar_lomba_success::DaftarLombaSuccess,
	index::Index, kompetisi::Kompetisi, login::Login, profile::Profile, register::Register,
	registration_success::RegistrationSuccess,
};
use serde::{Deserialize, Serialize};
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
	#[at("/register-success")]
	RegisterSuccess,
	#[at("/login")]
	Login,
	#[at("/kompetisi")]
	Kompetisi,
	#[at("/daftar-lomba")]
	DaftarLomba,
	#[at("/admin")]
	Admin,
	#[at("/daftar-lomba-success")]
	DaftarLombaSuccess,
	#[not_found]
	#[at("/404")]
	NotFound,
}

#[derive(Serialize, Deserialize)]
pub struct KompetisiQuery {
	pub search: String,
	pub category: String,
}

pub fn switch(routes: &Route) -> Html {
	match routes {
		Route::Index => html! { <Index/> },
		Route::Profile => html! { <Profile/> },
		Route::Kompetisi => {
			html! { <Kompetisi/> }
		}
		Route::Register => html! { <Register/> },
		Route::RegisterSuccess => html! { <RegistrationSuccess/> },
		Route::DaftarLomba => html! { <DaftarLomba/> },
		Route::DaftarLombaSuccess => html! { <DaftarLombaSuccess/> },
		Route::Login => html! { <Login/> },
		Route::Admin => html! { <Admin/> },
		Route::NotFound => html! { <h1>{ "404" }</h1> },
	}
}
