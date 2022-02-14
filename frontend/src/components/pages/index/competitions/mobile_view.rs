use super::index::LombaKategori;
use crate::router::{KompetisiQuery, Route};
use yew::prelude::*;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;

struct PaginationIndex {
	upper_bound: usize,
	lower_bound: usize,
}

#[derive(Properties, PartialEq)]
pub struct Props {
	pub kategori_lomba: Vec<LombaKategori>,
	pub kategori_lomba_length: usize,
}

#[function_component(MobileView)]
pub fn mobile_view(props: &Props) -> Html {
	let history = use_history().unwrap();

	let pagination = use_state(|| PaginationIndex {
		upper_bound: 1,
		lower_bound: 0,
	});

	let kategori_lomba = props.kategori_lomba.clone();
	let kategori_lomba_length = props.kategori_lomba_length.clone();

	let left_click = {
		let pagination = pagination.clone();
		let upper_bound = (*pagination).upper_bound;
		let lower_bound = (*pagination).lower_bound;

		Callback::from(move |_| {
			pagination.set(PaginationIndex {
				upper_bound: if upper_bound as i32 + 2 > kategori_lomba_length as i32 - 1 {
					upper_bound + 2 - kategori_lomba_length
				} else {
					upper_bound + 2
				},
				lower_bound: if lower_bound as i32 + 2 > kategori_lomba_length as i32 - 1 {
					lower_bound + 2 - kategori_lomba_length
				} else {
					lower_bound + 2
				},
			})
		})
	};

	let right_click = {
		let pagination = pagination.clone();
		let upper_bound = (*pagination).upper_bound;
		let lower_bound = (*pagination).lower_bound;

		Callback::from(move |_| {
			pagination.set(PaginationIndex {
				upper_bound: if upper_bound as i32 - 2 < 0 {
					kategori_lomba_length + upper_bound - 2
				} else {
					upper_bound - 2
				},
				lower_bound: if lower_bound as i32 - 2 < 0 {
					kategori_lomba_length + lower_bound - 2
				} else {
					lower_bound - 2
				},
			})
		})
	};

	html! {
	<div class="w-screen bg-blue-gradient-app flex flex-col">
		<div class="font-semibold text-[2.25rem] text-white text-center">{"Kategori"}</div>
		<div class="font-semibold text-[2.25rem] text-white text-center">{"Kompetisi"}</div>
		<br />
		<div class="flex justify-around items-center px-8">
			<div onclick={left_click} class="rounded-full w-8 h-8 flex justify-center items-center bg-white shadow-lg cursor-pointer">
				<i class="bx bx-left-arrow-alt"></i>
			</div>
			{
				for kategori_lomba[(*pagination).lower_bound...(*pagination).upper_bound]
				.iter()
				.map(|kategori| {
						let kategori = kategori.clone();
						let icon_src = &kategori.icon_src;
						let name = kategori.name;
						html! {
							<div onclick={
								let name = name.clone();
								let history = history.clone();
								Callback::once(move |_| {
									history.push_with_query(Route::Kompetisi, KompetisiQuery { search: String::from(""), category: String::from(name)});
								})
							}>
								<div class="bg-white w-24 h-24 drop-shadow-2xl rounded-lg flex justify-center items-center">
									<img src={String::from(icon_src)} alt="Event" />
								</div>
								<div class="text-center text-white font-medium capitalize">{name}</div>
							</div>
						}
					}
				)
			}
			<div onclick={right_click} class="rounded-full w-8 h-8 flex justify-center items-center bg-white shadow-lg cursor-pointer">
				<i class="bx bx-right-arrow-alt"></i>
			</div>
		</div>
		<br />
		<button class="bg-white py-2 px-4 rounded mx-auto w-auto drop-shadow font-semibold text-cyan-500">{"Lihat Semua"}</button>
		<br />
	</div>
	}
}
