use yew::{use_effect, use_state};
use yew_hooks::use_mount;

#[derive(PartialEq)]
pub struct WindowSize {
	pub width: f64,
	pub height: f64,
}

pub fn get_window_size() -> yew::UseStateHandle<WindowSize> {
	let window: web_sys::Window = web_sys::window().expect("window not available");
	let window_dimension = use_state(|| WindowSize {
		width: window.inner_width().unwrap().as_f64().unwrap(),
		height: window.inner_height().unwrap().as_f64().unwrap(),
	});

	{
		let window_dimension = window_dimension.clone();
		let window: web_sys::Window = web_sys::window().expect("window not available");
		use_effect(move || {
			let listener = gloo_events::EventListener::new(&window, "resize", move |_| {
				let window: web_sys::Window = web_sys::window().expect("window not available");
				let width = window.inner_width().unwrap();
				let height = window.inner_height().unwrap();
				let width_to_set = width.as_f64().unwrap();
				let height_to_set = height.as_f64().unwrap();
				window_dimension.set(WindowSize {
					width: width_to_set,
					height: height_to_set,
				});
			});
			listener.forget();

			|| {}
		});
	}

	window_dimension
}

pub fn get_search_as_state() -> yew::UseStateHandle<String> {
	let window: web_sys::Window = web_sys::window().expect("window not available");
	use_state(|| window.location().search().unwrap())
}

pub fn scroll_to_top() {
	{
		use_mount(move || {
			let window: web_sys::Window = web_sys::window().expect("window not available");
			window.scroll_to_with_x_and_y(0.0, 0.0);
		});
	}
}
