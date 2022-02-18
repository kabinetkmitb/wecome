use super::forms::REGISTER_FIELDS;
use crate::components::common::form_field::FormField;
use crate::router::Route;
use crate::types::auth::RegisterPayload;
use yew::prelude::*;
use yew_hooks::{use_async, use_map};
use yew_router::prelude::*;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
	let history = use_history().unwrap();
	crate::utils::interop::use_toast();

	let form_data = use_map(
		REGISTER_FIELDS
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	let register = {
		let form_data = form_data.clone();
		use_async(async move {
			let request = RegisterPayload {
				name: form_data.current().get("name").unwrap().clone(),
				email: form_data.current().get("email").unwrap().clone(),
				password: form_data.current().get("kata sandi").unwrap().clone(),
			};
			crate::services::auth::register(request).await
		})
	};

	{
		use_effect_with_deps(
			move |register| {
				if let Some(_) = &register.data {
					history.push(Route::RegisterSuccess);
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
		let form_data = form_data.clone();
		Callback::from(move |e: web_sys::FocusEvent| {
			e.prevent_default();
			let konfirmasi_password = form_data
				.current()
				.get("konfirmasi kata sandi")
				.unwrap()
				.clone();
			let password = form_data.current().get("kata sandi").unwrap().clone();
			let register = register.clone();

			if konfirmasi_password != password {
				crate::utils::interop::show_toast_with_message("Kata sandi tidak sama".to_string());
			} else {
				register.run();
			}
		})
	};

	html! {
		<>
		<div class="min-h-[97vh] flex flex-col w-full items-center">
		<div class="w-52 p-5 opacity-40">
			<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Logo Wecome" />
		</div>
		<div class="h-[1.5px] w-[90%] bg-gray-600 opacity-50"></div>
		<forms {onsubmit} class="p-5 flex flex-col items-start w-full">
			<div class="text-2xl font-semibold">{"Register"}</div>
			<div>{"Daftarkan akun anda"}</div>
			<div class="my-4 w-full">
					{
						for REGISTER_FIELDS.iter().cloned().map(|field_property| {
							let form_data = form_data.clone();
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
			<button type="submit" class="w-full px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{if register.loading { "Loading..."} else {"Register"} }</button>
			<div class="flex gap-1">{"Sudah punya akun?"}
				<Link<Route> to={Route::Login}>
					<div class="text-cyan-600 font-semibold">{"Masuk"}</div>
				</Link<Route>>
			</div>
			</div>
		</forms>
		</div>
		<div class="w-full h-6 [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)]"></div>
		</>
	}
}
