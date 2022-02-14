use yew::prelude::*;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
	pub modal_id: &'static str,
	pub children: Children,
	pub class: &'static str,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &Props) -> Html {
	let modal_ready = crate::utils::interop::use_tw();
	if modal_ready {
		html! {
			<div class={props.class} data-bs-toggle="modal" data-bs-target={format!("#{}", props.modal_id)}>
				{ for props.children.iter() }
			</div>
		}
	} else {
		html! {
			<>
			</>
		}
	}
}
