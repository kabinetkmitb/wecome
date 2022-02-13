use super::index::LombaKategori;
use crate::router::{KompetisiQuery, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub kategori_lomba: Vec<LombaKategori>,
}

#[function_component(DesktopView)]
pub fn desktop_view(props: &Props) -> Html {
	let kategori_lomba = props.kategori_lomba.clone();
	let history = use_history().unwrap();

	let onclick_cta = {
		let history = history.clone();
		Callback::once(move |_| {
			history.push_with_query(
				Route::Kompetisi,
				KompetisiQuery {
					search: String::from(""),
					category: String::from(""),
				},
			);
		})
	};

	html! {
		<div class="w-screen [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)] flex justify-around items-start px-6 py-10 gap-5">
			<div class="flex flex-col p-8">
				<div class="text-white font-medium text-[2.5rem]">
				<h1>{"Competition"}</h1>
				<h1>{"Category"}</h1>
				</div>
				<button onclick={onclick_cta} class="px-4 py-2 my-2 text-center md:m-2 rounded-lg bg-white text-cyan-400 block hover:text-white hover:bg-cyan-400 border-cyan-400 font-bold transition w-40">
				{"See More"}
				</button>
			</div>
			<div class="grid grid-cols-4 gap-5">
			{
				for kategori_lomba
				.iter()
				.map(|kategori| {
						let kategori = kategori.clone();
						let icon_src = &kategori.icon_src;
						let name = &kategori.name;
						html! {
							<div onclick={
								let history = history.clone();
								let name = name.clone();
								Callback::once(move |_| {history.push_with_query(Route::Kompetisi, KompetisiQuery { search: String::from(""), category: String::from(name)
							 });})} class="transition hover:scale-110 cursor-pointer">
								<div class=" bg-white w-28 h-28 drop-shadow-2xl rounded-lg flex justify-center items-center">
								<img src={String::from(icon_src)} alt="Event" />
								</div>
								<div class="text-center text-white font-medium capitalize">{name}</div>
							</div>
						}
					}
				)
			}
			</div>
		</div>
	}
}
