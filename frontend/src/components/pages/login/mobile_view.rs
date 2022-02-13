use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_map;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
	let fields_tuple = vec![("email", "".to_string()), ("kata sandi", "".to_string())];
	let form_data = use_map(fields_tuple.iter().cloned().collect());

	html! {
		<>
		<div class="min-h-[97vh] flex flex-col w-full items-center">
		<div class="w-52 p-5 opacity-40">
			<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Logo Wecome" />
		</div>
		<div class="h-[1.5px] w-[90%] bg-gray-600 opacity-50"></div>
		<form class="p-5 flex flex-col items-start w-full">
			<div class="text-2xl font-semibold">{"Register"}</div>
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
			<button class="w-full px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Login"}</button>
			<div class="text-center w-full font-semibold">{"Atau"}</div>
			<button class="w-full px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Login dengan SSO (INA)"}</button>
			<div class="flex gap-1">{"Belum punya akun?"}<div class="text-cyan-600 font-semibold">{"Daftar Akun"}</div></div>
			</div>
		</form>
		</div>
		<div class="w-full h-6 [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)]"></div>
		</>
	}
}
