use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub children: Children,
	pub title: &'static str,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
	let modal_ready = crate::utils::interop::use_modal();
	html! {
		<div
			class="dialog-container"
			id="modal"
			aria-hidden="true"
			aria-labelledby="my-dialog-title"
			aria-describedby="my-dialog-description"
		>
			<div class="dialog-overlay" onclick={move |_| crate::utils::interop::close_modal("modal".to_string())}></div>
			<div class="dialog-content" role="document">
				<button class="dialog-close" aria-label="Close dialog" onclick={move |_| crate::utils::interop::close_modal("modal".to_string())}>{"X"}</button>
				<h1 id="my-dialog-title">{props.title}</h1>
				{ for props.children.iter() }
			</div>
		</div>
	}
}
