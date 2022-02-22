use super::forms::REGISTER_FIELDS;
use crate::components::common::auth_layout::AuthLayout;
use crate::components::common::form_field::FormField;
use crate::router::Route;
use crate::types::auth::RegisterPayload;
use yew::prelude::*;
use yew_hooks::{use_async, use_map};
use yew_router::prelude::*;

#[function_component(DesktopView)]
pub fn desktop_view() -> Html {
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
				nim: form_data.current().get("nim").unwrap().clone(),
				name: form_data.current().get("full name").unwrap().clone(),
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

	let onclick = {
		let register = register.clone();
		let form_data = form_data.clone();
		Callback::from(move |_| {
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
		<AuthLayout is_admin={false}>
			<div class="ml-4 p-10 min-h-screen bg-white w-[50vw]">
				<div class="font-bold text-[2rem]">{"Register"}</div>
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
					<button {onclick} class="px-8 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{if register.loading {"Loading..."} else {"Daftar"}}</button>
					<div class="flex gap-1">{"Sudah punya akun?"}

					<Link<Route> to={Route::Login}>
						<div class="text-cyan-600 font-semibold">{"Masuk"}</div>
					</Link<Route>>
					</div>
				</div>
			</div>
		</AuthLayout>
	}
}
