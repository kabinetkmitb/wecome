use crate::types::form::FormFieldProperty;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub field_property: FormFieldProperty,
	pub key: &'static str,
	pub form_data: yew_hooks::UseMapHandle<&'static str, std::string::String>,
}

#[function_component(AuthLayout)]
pub fn auth_layout(props: &Props) -> Html {
	html! {
		<>
			<div class="mb-4 w-full">
				<label class="text-sm font-bold py-2 px-1 capitalize" for={props.field_property.clone().key}> {props.field_property.clone().key} </label>
				<input oninput={
					let map = props.form_data.clone();
					let map_key = props.key.clone();
					Callback::from(move |e: InputEvent| {
						let map_key = map_key.clone();
						let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
						map.update(&map_key, input_value);
				})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" />
			</div>
		</>
	}
}
