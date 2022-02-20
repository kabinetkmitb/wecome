use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
	pub children: Children,
	pub title: String,
	pub id: String,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
	let modal_ready = crate::utils::interop::use_tw();
	let props = props.clone();
	if modal_ready {
		html! {
			<div class="modal fade fixed top-0 left-0 hidden w-full h-full outline-none overflow-x-hidden overflow-y-auto" id={props.clone().id.clone()} tabindex="-1" aria-labelledby={props.clone().id.clone()} aria-modal="true" role="dialog">
				<div class="modal-dialog modal-xl modal-dialog-centered modal-dialog-scrollable relative w-auto pointer-events-none">
					<div class="modal-content border-none shadow-lg relative flex flex-col w-full pointer-events-auto bg-white bg-clip-padding rounded-md outline-none text-current">
					<div class="modal-header flex flex-shrink-0 items-center justify-between p-4 border-b border-gray-200 rounded-t-md">
						<h5 class="text-xl font-medium leading-normal text-gray-800" id="exampleModalCenteredScrollableLabel">
						{props.clone().title.clone()}
						</h5>
						<button type="button"
						class="btn-close box-content w-4 h-4 p-1 text-black border-none rounded-none opacity-50 focus:shadow-none focus:outline-none focus:opacity-100 hover:text-black hover:opacity-75 hover:no-underline"
						data-bs-dismiss="modal" aria-label="Close"></button>
					</div>
					<div class="modal-body relative p-4">
						{ for props.clone().children.iter() }
					</div>
					<div
						class="modal-footer flex flex-shrink-0 flex-wrap items-center justify-end p-4 border-t border-gray-200 rounded-b-md">
						<button type="button"
						class="cursor-pointer mt-2 py-2 px-8 w-fit rounded-sm hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition"
						data-bs-dismiss="modal">
							{"Close"}
						</button>
					</div>
					</div>
				</div>
			</div>
		}
	} else {
		html! {
			<>
			</>
		}
	}
}
