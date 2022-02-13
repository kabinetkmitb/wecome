use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_map;

#[function_component(DaftarLombaComponent)]
pub fn daftar_lomba_component() -> Html {
	let pendaftar_fields = vec![
		("nama pendaftar", "".to_string()),
		("no telp", "".to_string()),
		("email pendaftar", "".to_string()),
		("nama lembaga", "".to_string()),
		("nim", "".to_string()),
	];
	let pendaftar_data = use_map(pendaftar_fields.iter().cloned().collect());

	let detail_fields = vec![
		("nama kompetisi", "".to_string()),
		("kategori kompetisi", "".to_string()),
		("deskripsi kompetisi", "".to_string()),
		("tags kompetisi", "".to_string()),
		("tanggal pelaksanaan", "".to_string()),
		("batas registrasi", "".to_string()),
		("link registrasi lomba", "".to_string()),
	];
	let detail_data = use_map(detail_fields.iter().cloned().collect());

	let kontak_fields = vec![
		("website", "".to_string()),
		("linkedin", "".to_string()),
		("instagram", "".to_string()),
		("line", "".to_string()),
		("twitter", "".to_string()),
	];
	let kontak_data = use_map(kontak_fields.iter().cloned().collect());

	html! {
		<>
			<div class="[background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)] shadow-xl drop-shadow-xl px-6 py-10 gap-5">
				<div class="text-white text-2xl font-semibold">{"Daftarkan Lomba"}</div>
				<div class="text-white text-2xl font-semibold">{"Lembaga / Kampus Anda"}</div>
			</div>
			<div class="p-10">
			<div class="text-cyan-500 font-semibold text-xl">{"Identitas Pendaftar"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for pendaftar_fields.iter().cloned().map(|(key,_)| {
						html! {
							<div class="w-full">
								<label class="text-sm font-bold py-2 px-1 capitalize" for="username"> {key} </label>
								<input oninput={
									let map = pendaftar_data.clone();
									Callback::from(move |e: InputEvent| {
									let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
									map.update(&key, input_value);
								})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" />
							</div>
						}
					})
				}
			</div>
			<div class="text-cyan-500 font-semibold text-xl">{"Detil Kompetisi"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for detail_fields.iter().cloned().map(|(key,_)| {
						html! {
							<div class="w-full">
								<label class="text-sm font-bold py-2 px-1 capitalize" for="username"> {key} </label>
								<input oninput={
									let map = detail_data.clone();
									Callback::from(move |e: InputEvent| {
									let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
									map.update(&key, input_value);
								})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" />
							</div>
						}
					})
				}
			</div>
			<div class="text-cyan-500 font-semibold text-xl">{"Kontak Penyelenggara / Kompetisi"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for kontak_fields.iter().cloned().map(|(key,_)| {
						html! {
							<div class="w-full">
								<label class="text-sm font-bold py-2 px-1 capitalize" for="username"> {key} </label>
								<input oninput={
									let map = kontak_data.clone();
									Callback::from(move |e: InputEvent| {
									let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
									map.update(&key, input_value);
								})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" />
							</div>
						}
					})
				}
			</div>
			<label class="text-sm font-bold py-2 px-1 capitalize" for="user-avatar"> {"Poster Kompetisi"} </label>
			<input class="block w-[45%] cursor-pointer bg-gray-50 border border-gray-300 text-gray-900 focus:outline-none focus:border-transparent text-sm rounded-lg" aria-describedby="user_avatar_help" id="user_avatar" type="file"/>
			<span>{"Catatan: file harus dalam format .png atau .jpg"}</span>
			<button class="px-8 py-2 my-5 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Kirim"}</button>
			</div>
		</>
	}
}
