use crate::types::form::{FormFieldProperty, FormFieldType};
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub field_property: FormFieldProperty,
	pub key_input: std::string::String,
	pub form_data: yew_hooks::UseMapHandle<std::string::String, std::string::String>,
}

#[function_component(FormField)]
pub fn form_field(props: &Props) -> Html {
	crate::utils::interop::use_tw();
	match &props.clone().field_property.input_type {
		FormFieldType::Text => {
			html! {
				<>
					<div class="mb-4 w-full">
						<label class="text-sm font-bold py-2 px-1 capitalize" for={props.field_property.clone().key}> {props.field_property.clone().key} </label>
						<input oninput={
							let map = props.form_data.clone();
							let map_key = props.key_input.clone();
							Callback::from(move |e: InputEvent| {
								let map_key = map_key.clone();
								let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
								map.update(&map_key, input_value);
						})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder={props.field_property.clone().placeholder.unwrap()} />
					</div>
				</>
			}
		}
		FormFieldType::TextField => {
			html! {
				<>
					<div class="mb-4 w-full">
						<label class="text-sm font-bold py-2 px-1 capitalize" for={props.field_property.clone().key}> {props.field_property.clone().key} </label>
						<textarea oninput={
							let map = props.form_data.clone();
							let map_key = props.key_input.clone();
							Callback::from(move |e: InputEvent| {
								let map_key = map_key.clone();
								let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
								map.update(&map_key, input_value);
						})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id={props.field_property.clone().key} type="text" placeholder={props.field_property.clone().placeholder.unwrap()} />
					</div>
				</>
			}
		}
		FormFieldType::TextWithPrefix { prefix } => {
			html! {
				<>
				<div class="mb-4 w-full">
					<label class="text-sm font-bold py-2 px-1 capitalize" for={props.field_property.clone().key}> {props.field_property.clone().key} </label>
					<div class="flex">
						<span class="inline-flex items-center px-3 text-sm text-gray-900 bg-gray-200 rounded-l-md border border-r-0 border-gray-300 dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
							{prefix}
						</span>
						<input type="text" id={props.field_property.clone().key} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" />
					</div>
				</div>
				</>
			}
		}
		FormFieldType::DateTime => {
			html! {
				<>
					<div class="datepicker relative mb-4 w-full" data-mdb-toggle-button="false">
						<label class="text-sm font-bold py-2 px-1 capitalize" for={props.field_property.clone().key}> {props.field_property.clone().key} </label>
						<input oninput={
							let map = props.form_data.clone();
							let map_key = props.key_input.clone();
							Callback::from(move |e: InputEvent| {
								let map_key = map_key.clone();
								let input_value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
								map.update(&map_key, input_value);
						})} class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id={props.field_property.clone().key} type="text" placeholder="Select a date" data-mdb-toggle="datepicker"/>
					</div>
				</>
			}
		}
	}
}
