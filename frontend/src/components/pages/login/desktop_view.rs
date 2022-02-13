use crate::components::common::auth_layout::AuthLayout;
use crate::router::Route;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_map;
use yew_router::prelude::*;

#[function_component(DesktopView)]
pub fn desktop_view() -> Html {
	let fields_tuple = vec![("email", "".to_string()), ("kata sandi", "".to_string())];
	let form_data = use_map(fields_tuple.iter().cloned().collect());

	html! {
		<AuthLayout is_admin={false}>
			<div class="p-20">
				<div class="mb-8">
				<div class="text-[3rem] font-bold text-white">{"Login"}</div>
				<div class="text-white">{"Masuk dengan akun anda"}</div>
				</div>
				<div class="grid grid-cols-2 gap-20 justify-around">
				<div class="bg-white p-10 rounded-md relative">
					<div class="font-semibold">{"Pemilik ITB Network Account (INA)"}</div>
					<br/>
					<div>{"Untuk Mahasiswa dan Staff Fakultas dengan Akun ITB Network Account  (INA)"}</div>
					<button class="absolute bottom-10 left-10 px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Login dengan SSO / INA"}</button>
				</div>
				<div class="bg-white p-10 rounded-md relative">
					<div class="font-semibold">{"Tanpa ITB Network Account (INA)"}</div>
					<br/>
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
					<div class="text-cyan-600 font-semibold text-sm">{"Lupa kata sandi?"}</div>
					<button class="px-8 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Login"}</button>
					<div class="flex gap-1">{"Belum punya akun?"}
					<Link<Route> to={Route::Register}>
						<div class="text-cyan-600 font-semibold">{"Daftar Akun"}</div>
					</Link<Route>>
					</div>
				</div>
				</div>
			</div>
		</AuthLayout>
	}
}
