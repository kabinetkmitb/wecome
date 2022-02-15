use js_sys::{Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::{prelude::*, JsValue};
use yew_interop::declare_resources;

declare_resources!(
	toast
	"https://cdn.jsdelivr.net/npm/toastify-js@1.11.2/src/toastify.min.js"
	"https://cdn.jsdelivr.net/npm/toastify-js@1.11.2/src/toastify.min.css"
	modal
	"https://cdn.jsdelivr.net/npm/a11y-dialog@7/dist/a11y-dialog.min.js"
	tw
	"https://cdn.jsdelivr.net/npm/tw-elements/dist/js/index.min.js"
	lottie
	"https://unpkg.com/@lottiefiles/lottie-player@latest/dist/lottie-player.js"
);

// The javascript API: https://github.com/apvarun/toastify-js/blob/572517040fae6a7f8be4a99778dacda9c933db45/README.md
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = Toastify)]
	pub type Toast;

	#[wasm_bindgen(constructor, js_class = "Toastify")]
	pub fn new(config: &JsValue) -> Toast;

	#[wasm_bindgen(method, structural, js_class = "Toastify", js_name = showToast)]
	pub fn show_toast(this: &Toast);
}

pub fn show_toast_with_message(message: String) {
	let config = Object::new();
	Reflect::set(&config, &"text".into(), &message.into()).ok();
	let toast = Toast::new(&config);
	toast.show_toast();
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = A11yDialog)]
	pub type Modal;

	#[wasm_bindgen(constructor, js_class = "A11yDialog")]
	pub fn new(div: &web_sys::HtmlElement) -> Modal;

	#[wasm_bindgen(method, structural, js_class = "A11yDialog", js_name = show)]
	pub fn show_modal(this: &Modal);

	#[wasm_bindgen(method, structural, js_class = "A11yDialog", js_name = hide)]
	pub fn close_modal(this: &Modal);
}

pub fn show_modal(id: String) {
	let window = web_sys::window().expect("global window does not exists");
	let document = window.document().expect("expecting a document on window");
	let div = document
		.get_element_by_id(&id)
		.unwrap()
		.dyn_into::<web_sys::HtmlElement>()
		.unwrap();

	let modal = Modal::new(&div);
	modal.show_modal();
}

pub fn close_modal(id: String) {
	let window = web_sys::window().expect("global window does not exists");
	let document = window.document().expect("expecting a document on window");
	let div = document
		.get_element_by_id(&id)
		.unwrap()
		.dyn_into::<web_sys::HtmlElement>()
		.unwrap();

	let modal = Modal::new(&div);
	modal.close_modal();
}
