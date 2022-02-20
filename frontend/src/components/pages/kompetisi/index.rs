use super::modal::KompetisiModal;
use crate::components::common::modal_button::ModalButton;
use crate::router::{KompetisiQuery, Route};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Callback;
use yew_hooks::{use_async, use_map, use_mount, use_search_param};
use yew_router::prelude::*;

#[function_component(KompetisiComponent)]
pub fn kompetisi_component() -> Html {
	crate::utils::interop::use_tw();
	let history = use_history().unwrap();

	let search_param = use_search_param("search".to_string()).unwrap();
	let category_param = use_search_param("category".to_string()).unwrap();

	let modal_props = use_map(HashMap::from([
		("judul".to_string(), "".to_string()),
		("kategori".to_string(), "".to_string()),
		("tanggal_pelaksanaan".to_string(), "".to_string()),
		("deskripsi".to_string(), "".to_string()),
		("batas_regist".to_string(), "".to_string()),
		("link_poster".to_string(), "".to_string()),
		("link_website".to_string(), "".to_string()),
		("link_linkedin".to_string(), "".to_string()),
		("link_twitter".to_string(), "".to_string()),
		("link_instagram".to_string(), "".to_string()),
		("link_regist".to_string(), "".to_string()),
	]));

	let navigation_param_input = use_map(HashMap::from([
		("search".to_string(), search_param.clone().to_string()),
		("category".to_string(), category_param.clone().to_string()),
	]));

	let get_kompetisi = {
		let navigation_param_input = navigation_param_input.clone();
		use_async(async move {
			let response = crate::services::kompetisi::get_kompetisi(crate::utils::api::search(
				navigation_param_input
					.current()
					.get("search")
					.unwrap()
					.to_string(),
				navigation_param_input
					.current()
					.get("category")
					.unwrap()
					.to_string(),
			))
			.await;
			match response {
				Ok(response) => Ok(response),
				Err(e) => {
					crate::utils::interop::show_toast_with_message(e.to_string());
					Err(e)
				}
			}
		})
	};

	{
		let get_kompetisi = get_kompetisi.clone();
		use_mount(move || {
			get_kompetisi.run();
		});
	}

	let dropdown_onchange = {
		let navigation_param_input = navigation_param_input.clone();
		Callback::from(move |e: web_sys::Event| {
			let navigation_param_input = navigation_param_input.clone();

			let select_element = e
				.target()
				.unwrap()
				.dyn_into::<web_sys::HtmlSelectElement>()
				.unwrap();

			let chosen_index = select_element.selected_index();

			let options = select_element.options();

			match options.set_selected_index(chosen_index) {
				Ok(_) => {}
				Err(_) => {
					log::error!("Error setting selected index");
				}
			};

			let category = options
				.item(chosen_index as u32)
				.unwrap()
				.dyn_into::<web_sys::HtmlOptionElement>()
				.unwrap()
				.text();

			navigation_param_input.update(&"category".to_string(), category);
		})
	};

	let oninput = {
		let navigation_param_input = navigation_param_input.clone();
		Callback::from(move |e: web_sys::InputEvent| {
			let input_value = e
				.target()
				.unwrap()
				.dyn_into::<web_sys::HtmlInputElement>()
				.unwrap()
				.value();
			navigation_param_input.update(&"search".to_string(), input_value);
		})
	};

	let onsubmit = {
		let navigation_param_input = navigation_param_input.clone();
		let get_kompetisi = get_kompetisi.clone();
		Callback::once(move |e: web_sys::FocusEvent| {
			e.prevent_default();
			let navigation_param_input = navigation_param_input.clone();
			let get_kompetisi = get_kompetisi.clone();
			match history.push_with_query(
				Route::Kompetisi,
				KompetisiQuery {
					search: navigation_param_input
						.current()
						.get("search")
						.unwrap()
						.to_string(),
					category: navigation_param_input
						.current()
						.get("category")
						.unwrap()
						.to_string(),
				},
			) {
				Ok(_) => {}
				Err(e) => {
					crate::utils::interop::show_toast_with_message(e.to_string());
				}
			};

			get_kompetisi.run();
		})
	};

	html! {
		<>
		<KompetisiModal
			kategori={modal_props.current().get("kategori").clone().unwrap().clone()}
			judul={modal_props.current().get("judul").clone().unwrap().clone()}
			deskripsi={modal_props.current().get("deskripsi").clone().unwrap().clone()}
			tanggal_pelaksanaan={modal_props.current().get("tanggal_pelaksanaan").clone().unwrap().clone()}
			batas_regist={modal_props.current().get("batas_regist").clone().unwrap().clone()}
			link_poster={modal_props.current().get("link_poster").clone().unwrap().clone()}
			link_website={modal_props.current().get("link_website").clone().unwrap().clone()}
			link_linkedin={modal_props.current().get("link_linkedin").clone().unwrap().clone()}
			link_twitter={modal_props.current().get("link_twitter").clone().unwrap().clone()}
			link_instagram={modal_props.current().get("link_instagram").clone().unwrap().clone()}
			link_regist={modal_props.current().get("link_regist").clone().unwrap().clone()}
		/>
		<div class="p-6 h-screen overflow-y-scroll relative z-10">
		<form {onsubmit} class="flex items-center gap-2 text-[0.6rem] md:text-[1rem] max-w-[600px]">
			<select onchange={dropdown_onchange} class="form-select appearance-none
						block
						pl-[1rem]
						pr-[3rem]
						py-1.5
						text-base
						font-normal
						text-gray-700
						bg-white bg-clip-padding bg-no-repeat
						border border-solid border-gray-300
						rounded
						transition
						ease-in-out
						m-0
						focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none" aria-label="Kategori">
				<option selected={true}>{category_param.clone().to_string()}</option>
				{
					for ["riset".to_string(), "seni".to_string(), "akademik".to_string(), "teknologi".to_string(), "bisnis".to_string(), "bahasa".to_string(), "olahraga".to_string(), "konferensi".to_string()]
						.iter()
						.filter(|category| category.to_string() != category_param.clone().to_string())
						.map(|string| {
							html! {
								<option >{string}</option>
							}
						})
				}
			</select>
			<input {oninput} class="appearance-none border rounded w-full p-1 md:px-4 md:py-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Nama kompetisi" value={navigation_param_input.clone().current().get("search").unwrap().to_string()} />
			<button type="submit" class="p-1 md:px-4 md:py-2 rounded-md hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Cari"}</button>
		</form>
		<div class="my-4 text-[0.6rem] md:text-[0.8rem] md:grid md:grid-cols-3 gap-3 flex flex-col">
			{
				if let Some(kompetisi_list) = &get_kompetisi.clone().data {
					let modal_props = modal_props.clone();
					let kompetisi_list = kompetisi_list.clone();
					if !kompetisi_list.clone().is_empty() {
						let modal_props = modal_props.clone();
						let kompetisi_list = kompetisi_list.clone();
						kompetisi_list.clone().into_iter().map(move |kompetisi| {
							let modal_props = modal_props.clone();
							html_nested! {
								<div class="border-[1.25px] border-gray-300 shadow-sm rounded flex bg-white">
									<img class="w-[40%] object-cover" src={kompetisi.clone().link_poster} alt="Logo lomba"/>
									<div class="w-[60%] p-3 flex flex-col gap-1">
										<div class="py-1 w-fit tracking-widest px-2 text-[0.8em] rounded-2xl font-meidum bg-[#FECC30] text-white text-center inline-block">{kompetisi.clone().kategori_kompetisi}</div>
										<div class="font-bold text-[1.7em] md:text-[1.2em]">{kompetisi.clone().nama_kompetisi}</div>
										<div class="justify">{if kompetisi.clone().deskripsi_kompetisi.len() < 100 { kompetisi.clone().deskripsi_kompetisi } else {format!("{}...",&kompetisi.clone().deskripsi_kompetisi[0..100])}}</div>
										<div class="justify">{format!("Pelaksanaan : {}", kompetisi.clone().tanggal_pelaksanaan)}</div>
										<div class="justify">{format!("Registrasi : {}-{}", kompetisi.clone().batas_awal_registrasi, kompetisi.clone().batas_akhir_registrasi)}</div>
										<div onclick={
											let modal_props = modal_props.clone();
											move |_| {
												let modal_props = modal_props.clone();
												modal_props.update(&"kategori".to_string(), kompetisi.clone().kategori_kompetisi.clone());
												modal_props.update(&"judul".to_string(), kompetisi.clone().nama_kompetisi.clone());
												modal_props.update(&"deskripsi".to_string(), kompetisi.clone().deskripsi_kompetisi.clone());
												modal_props.update(&"tanggal_pelaksanaan".to_string(), kompetisi.clone().tanggal_pelaksanaan.clone());
												modal_props.update(&"batas_regist".to_string(), format!("{}-{}", kompetisi.clone().batas_awal_registrasi, kompetisi.clone().batas_akhir_registrasi));
												modal_props.update(&"link_poster".to_string(), kompetisi.clone().link_poster.clone());
												modal_props.update(&"link_website".to_string(), kompetisi.clone().link_website.clone());
												modal_props.update(&"link_linkedin".to_string(), kompetisi.clone().link_linkedin.clone());
												modal_props.update(&"link_twitter".to_string(), kompetisi.clone().akun_twitter.clone());
												modal_props.update(&"link_instagram".to_string(), kompetisi.clone().akun_instagram.clone());
												modal_props.update(&"link_regist".to_string(), kompetisi.clone().link_registrasi.clone());
											}
										}>
											<ModalButton modal_id="kompetisi-modal" class="cursor-pointer mt-2 p-1 w-fit rounded-md hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Detail Kompetisi"}</ModalButton>
										</div>
									</div>
								</div>
							}
						}).collect::<VNode>()
					}
					else {
						html_nested! {
							<div class="text-gray-600">{"Tidak ada kompetisi yang sesuai"}</div>
						}
					}
				} else {
					(1..7).map(|_| {
						html_nested! {
							<div class="p-[1rem] shadow-sm rounded flex bg-white opacity-40">
								<div class="animate-pulse w-full flex space-x-4">
									<div class="w-[40%] h-full bg-slate-500" />
									<div class="flex-1 space-y-6 py-1">
									<div class="h-2 bg-slate-500 rounded"></div>
									<div class="space-y-3">
										<div class="grid grid-cols-3 gap-4">
											<div class="h-2 bg-slate-500 rounded col-span-2"></div>
											<div class="h-2 bg-slate-500 rounded col-span-1"></div>
										</div>
										<div class="h-2 bg-slate-500 rounded"></div>
										<div class="h-2 bg-slate-500 rounded"></div>
										<div class="h-2 bg-slate-500 rounded"></div>
									</div>
									<div class="h-5 w-10 bg-slate-500"></div>
									</div>
								</div>
							</div>
						}
					}).collect::<VNode>()
				}
			}
		</div>
		</div>
		<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Background logo" class="absolute bottom-20 left-[-6rem] opacity-40 z-0" style="transform: rotate(-12.79deg);"/>
		</>
	}
}
