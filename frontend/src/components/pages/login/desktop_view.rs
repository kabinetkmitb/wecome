use super::forms::LOGIN_FIELDS;
use crate::components::common::auth_layout::AuthLayout;
use crate::components::common::form_field::FormField;
use crate::context::user::{use_user, UserState};
use crate::router::Route;
use crate::types::auth::LoginPayload;
use yew::prelude::*;
use yew_hooks::{use_async, use_map};
use yew_router::prelude::*;

#[function_component(DesktopView)]
pub fn desktop_view() -> Html {
	let history = use_history().unwrap();
	let user_ctx = use_user();
	crate::utils::interop::use_toast();

	let form_data = use_map(
		LOGIN_FIELDS
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	let login = {
		let form_data = form_data.clone();
		use_async(async move {
			let request = LoginPayload {
				email: form_data.current().get("email").unwrap().clone(),
				password: form_data.current().get("kata sandi").unwrap().clone(),
			};
			crate::services::auth::login(request).await
		})
	};

	{
		use_effect_with_deps(
			move |login| {
				if let Some(login_data) = &login.data {
					history.push(Route::Index);
					user_ctx.login(UserState {
						name: login_data.name.clone(),
						is_admin: login_data.is_admin,
						token: login_data.token.clone(),
					});
				}
				if let Some(e) = &login.error {
					crate::utils::interop::show_toast_with_message(e.to_string());
				}
				|| ()
			},
			login.clone(),
		);
	}

	let onsubmit = {
		let login = login.clone();
		Callback::once(move |_| login.run())
	};

	html! {
		<AuthLayout is_admin={false}>
			<div class="ml-4 p-10 min-h-screen bg-white w-[50vw]">
				<div class="font-bold text-[2rem]">{"Login"}</div>
				<div>{"Masuk dengan akun anda"}</div>
				<form {onsubmit} class="my-4 w-full">
					{
						for LOGIN_FIELDS.iter().cloned().map(|field_property| {
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

					<Link<Route> to={Route::Register}>
						<div class="text-sm text-cyan-600 font-semibold">{"Lupa kata sandi?"}</div>
					</Link<Route>>
					<button type="submit" class="px-8 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{if login.loading {"Loading..."} else {"Masuk"}}</button>
					<div class="flex gap-1">{"Belum punya akun?"}

					<Link<Route> to={Route::Register}>
						<div class="text-cyan-600 font-semibold">{"Register"}</div>
					</Link<Route>>
					</div>
				</form>
			</div>
		</AuthLayout>
	}
}
