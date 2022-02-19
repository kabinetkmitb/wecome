use super::forms::{DETAIL_FIELDS, KONTAK_FIELDS, PENDAFTAR_FIELDS};
use crate::components::common::form_field::FormField;
use crate::router::Route;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::{use_async, use_map};
use yew_router::prelude::*;

#[function_component(DaftarLombaComponent)]
pub fn daftar_lomba_component() -> Html {
	let history = use_history().unwrap();
	crate::utils::interop::use_toast();
	let pendaftar_data = use_map(
		PENDAFTAR_FIELDS
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	let detail_data = use_map(
		DETAIL_FIELDS
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	let kontak_data = use_map(
		KONTAK_FIELDS
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);
	let file_data: yew::UseStateHandle<Option<web_sys::File>> = use_state(|| None);

	let register = {
		let file_data = file_data.clone();
		let pendaftar_data = pendaftar_data.clone();
		let detail_data = detail_data.clone();
		let kontak_data = kontak_data.clone();
		use_async(async move {
			let url = match crate::utils::api::upload_file(
				file_data.as_ref().unwrap().slice().unwrap(),
				file_data.as_ref().unwrap().name(),
			)
			.await
			{
				Ok(url) => url,
				Err(e) => {
					crate::utils::interop::show_toast_with_message(e.to_string());
					return Err(e);
				}
			};

			let request = crate::types::kompetisi::ProposeKompetisiPayload {
				nama_lembaga_pendaftar: pendaftar_data
					.current()
					.get("nama lembaga")
					.unwrap()
					.clone(),
				no_telp: pendaftar_data.current().get("no telp").unwrap().clone(),
				nim_pendaftar: pendaftar_data.current().get("nim").unwrap().clone(),
				nama_kompetisi: detail_data.current().get("nama kompetisi").unwrap().clone(),
				kategori_kompetisi: detail_data
					.current()
					.get("kategori kompetisi")
					.unwrap()
					.clone(),
				deskripsi_kompetisi: detail_data
					.current()
					.get("deskripsi kompetisi")
					.unwrap()
					.clone(),
				tags_kompetisi: detail_data.current().get("tags kompetisi").unwrap().clone(),
				tanggal_pelaksanaan: detail_data
					.current()
					.get("tanggal pelaksanaan")
					.unwrap()
					.clone(),
				batas_awal_registrasi: detail_data
					.current()
					.get("batas awal registrasi")
					.unwrap()
					.clone(),
				batas_akhir_registrasi: detail_data
					.current()
					.get("batas akhir registrasi")
					.unwrap()
					.clone(),
				link_registrasi: detail_data
					.current()
					.get("link registrasi lomba")
					.unwrap()
					.clone(),
				link_website: kontak_data.current().get("website").unwrap().clone(),
				link_linkedin: kontak_data.current().get("linkedin").unwrap().clone(),
				id_line: kontak_data.current().get("line").unwrap().clone(),
				akun_instagram: format!(
					"https://instagram.com/{}",
					kontak_data.current().get("instagram").unwrap().clone()
				),
				akun_twitter: format!(
					"https://twitter.com/{}",
					kontak_data.current().get("twitter").unwrap().clone()
				),
				link_poster: url,
			};

			crate::services::kompetisi::propose_kompetisi(request).await
		})
	};

	{
		use_effect_with_deps(
			move |register| {
				if let Some(_) = &register.data {
					history.push(Route::DaftarLombaSuccess);
				}
				if let Some(e) = &register.error {
					crate::utils::interop::show_toast_with_message(e.to_string());
				}
				|| ()
			},
			register.clone(),
		);
	}

	let onsubmit = {
		let register = register.clone();
		Callback::once(move |e: web_sys::FocusEvent| {
			e.prevent_default();
			register.run()
		})
	};

	let onchange = {
		let file_data = file_data.clone();
		Callback::once(move |e: web_sys::Event| {
			let input_value = e
				.target()
				.unwrap()
				.dyn_into::<web_sys::HtmlInputElement>()
				.unwrap()
				.files()
				.unwrap()
				.get(0)
				.unwrap();
			file_data.set(Some(input_value))
		})
	};

	html! {
		<form {onsubmit}>
			<div class="bg-blue-gradient-app shadow-xl drop-shadow-xl px-6 py-10 gap-5">
				<div class="text-white text-2xl font-semibold">{"Daftarkan Lomba"}</div>
				<div class="text-white text-2xl font-semibold">{"Lembaga / Kampus Anda"}</div>
			</div>
			<div class="p-10">
			<div class="text-cyan-500 font-semibold text-xl">{"Identitas Pendaftar"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for PENDAFTAR_FIELDS.iter().cloned().map(|field_property| {
						let form_data = pendaftar_data.clone();
						let field_property = field_property.clone();
						html_nested! {
							<FormField
								field_property={field_property.clone()}
								key_input={field_property.key}
								form_data={form_data.clone()}
							/>
						}
					})
				}
			</div>
			<div class="text-cyan-500 font-semibold text-xl">{"Detil Kompetisi"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for DETAIL_FIELDS.iter().cloned().map(|field_property| {
						let form_data = detail_data.clone();
						let field_property = field_property.clone();
						html_nested! {
							<FormField
								field_property={field_property.clone()}
								key_input={field_property.key}
								form_data={form_data.clone()}
							/>
						}
					})
				}
			</div>
			<div class="text-cyan-500 font-semibold text-xl">{"Kontak Penyelenggara / Kompetisi"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for KONTAK_FIELDS.iter().cloned().map(|field_property| {
						let form_data = kontak_data.clone();
						let field_property = field_property.clone();
						html_nested! {
							<FormField
								field_property={field_property.clone()}
								key_input={field_property.key}
								form_data={form_data.clone()}
							/>
						}
					})
				}
			</div>
			<label class="text-sm font-bold py-2 px-1 capitalize" for="user-avatar"> {"Poster Kompetisi"} </label>
			<input required={true} {onchange} class="block w-[45%] cursor-pointer bg-gray-50 border border-gray-300 text-gray-900 focus:outline-none focus:border-transparent text-sm rounded-lg" aria-describedby="user_avatar_help" id="user_avatar" type="file"/>
			<span>{"Catatan: file harus dalam format .png atau .jpg"}</span>
			<button type="submit" class="px-8 py-2 my-5 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{if register.loading {"Loading..."} else {"Kirim"} }</button>
			</div>
		</form>
	}
}
