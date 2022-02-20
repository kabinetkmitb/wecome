use super::forms::LOGIN_FIELDS;
use crate::components::common::form_field::FormField;
use crate::context::user::{use_user, UserState};
use crate::router::Route;
use crate::types::auth::LoginPayload;
use yew::prelude::*;
use yew_hooks::{use_async, use_map};
use yew_router::prelude::*;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
    let history = use_history().unwrap();
    crate::utils::interop::use_toast();

    let user_ctx = use_user();

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
                        id: login_data.id.clone(),
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
        Callback::once(move |e: web_sys::FocusEvent| {
            e.prevent_default();
            login.run()
        })
    };

    html! {
        <form {onsubmit}>
        <div class="min-h-[97vh] flex flex-col w-full items-center">
        <div class="w-52 p-5 opacity-40">
            <img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Logo Wecome" />
        </div>
        <div class="h-[1.5px] w-[90%] bg-gray-600 opacity-50"></div>
        <div class="p-5 flex flex-col items-start w-full">
            <div class="text-2xl font-semibold">{"Login"}</div>
            <div class="my-4 w-full">
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
            <button type="submit"  class="w-full px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{ if login.loading {"Loading..."} else {"Login"}}</button>
            <div class="flex gap-1">{"Belum punya akun?"}
                <Link<Route> to={Route::Register}>
                    <div class="text-cyan-600 font-semibold">{"Daftar Akun"}</div>
                </Link<Route>>
            </div>
            </div>
        </div>
        </div>
        <div class="w-full h-6 bg-blue-gradient-app"></div>
        </form>
    }
}
