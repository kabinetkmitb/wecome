use crate::utils::hooks::get_window_size;
use yew::prelude::*;

#[function_component(RegisterEvent)]
pub fn register_event() -> Html {
	let window_size = get_window_size();

	if window_size.width > 900.0 {
		html! {
		<div class="w-screen bg-white flex flex-col px-6 py-10 sm:m-12 gap-5">
			<div>
				<div class="font-semibold text-cyan-400 text-2xl">{"Ingin mendaftarkan lomba"}</div>
				<div class="font-semibold text-2xl">{"kampus atau lembaga anda?"}</div>
			</div>
			<br/>
			<div class="flex flex-col md:flex-row justify-between">
				<div class="md:w-[560px] w-[480px]">
					<iframe width="560" height="315" src="https://www.youtube.com/embed/tgbNymZ7vqY"> </iframe>
				</div>
				<div class="py-12 md:py-0 md:px-24">
				<div class="font-medium text-justify">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam viverra lectus turpis, et volutpat nisi hendrerit a. Cras sodales maximus vehicula. Etiam velit ipsum, sagittis id ligula eu, dictum aliquam tellus. Nulla sollicitudin egestas metus sit amet rutrum. Donec eu cursus urna. Ut id enim orci. Suspendisse ac enim a orci aliquam mollis. Ut vel est quam. Integer sit amet facilisis mi. Curabitur semper justo in iaculis sodales."}</div>
				</div>
			</div>
		</div>
		}
	} else {
		html! {
			<></>
		}
	}
}
