use crate::context::user::{use_user, UserState};
use crate::router::Route;
use crate::types::auth::LoginPayload;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::{use_async, use_map};
use yew_router::prelude::*;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
    let history = use_history().unwrap();
    crate::utils::interop::use_toast();

    let user_ctx = use_user();
    let fields_tuple = vec![("email", "".to_string()), ("kata sandi", "".to_string())];
    let form_data = use_map(fields_tuple.iter().cloned().collect());

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

    let onclick = {
        let login = login.clone();
        Callback::once(move |_| login.run())
    };

    html! {
        <>
        <div class="min-h-[97vh] flex flex-col w-full items-center">
        <div class="w-52 p-5 opacity-40">
            <img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Logo Wecome" />
        </div>
        <div class="h-[1.5px] w-[90%] bg-gray-600 opacity-50"></div>
        <div class="p-5 flex flex-col items-start w-full">
            <div class="text-2xl font-semibold">{"Login"}</div>
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
            <button {onclick} class="w-full px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{ if login.loading {"Loading..."} else {"Login"}}</button>
            <div class="text-center w-full font-semibold">{"Atau"}</div>
            <button class="w-full px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Login dengan SSO (INA)"}</button>
            <div class="flex gap-1">{"Belum punya akun?"}
                <Link<Route> to={Route::Register}>
                    <div class="text-cyan-600 font-semibold">{"Daftar Akun"}</div>
                </Link<Route>>
            </div>
            </div>
        </div>
        </div>
        <div class="w-full h-6 bg-blue-gradient-app"></div>
        </>
    }
}
