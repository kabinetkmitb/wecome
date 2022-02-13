use crate::components::common::auth_layout::AuthLayout;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_map;

#[function_component(DesktopView)]
pub fn desktop_view() -> Html {
	let fields_tuple = vec![
		("name", "".to_string()),
		("nim", "".to_string()),
		("email", "".to_string()),
		("kata sandi", "".to_string()),
		("konfirmasi kata sandi", "".to_string()),
	];
	let form_data = use_map(fields_tuple.iter().cloned().collect());

	html! {
		<AuthLayout is_admin={false}>
			<div class="ml-4 p-10 min-h-screen bg-white w-[50vw]">
				<div class="font-bold text-[2rem]">{"Register"}</div>
				<div>{"Daftarkan akun anda"}</div>
				<div class="my-4 w-full">
					{
						for fields_tuple.iter().cloned().map(|(key,_)| {
								html! {
									<div class="mb-4 w-full">
										<label class="text-sm font-bold py-2 px-1 capitalize" for="username"> {key} </label>
										<input oninput={
											let map = form_data.clone();
											Callback::from(move |e: InputEvent| {
											let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
											map.update(&key, input_value);
										})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" />
									</div>
								}
							})
					}
					<button class="px-8 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Daftar"}</button>
					<div class="flex gap-1">{"Sudah punya akun?"}<div class="text-cyan-600 font-semibold">{"Masuk"}</div></div>
				</div>
			</div>
		</AuthLayout>
	}
}
