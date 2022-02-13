use crate::router::Route;
use crate::utils::hooks::get_window_size;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(RegisterEvent)]
pub fn register_event() -> Html {
	let window_size = get_window_size();

	if window_size.width > 900.0 {
		html! {
		<div class="bg-white flex flex-col px-6 py-10 sm:m-12 gap-5">
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
				<div class="font-medium text-justify">{"We-Come (Website Competition), yang berarti “Kami Datang”, merupakan platform wadah informasi kompetisi di bidang akademik, teknologi, business plan & business case, riset, konferensi, olahraga, dan seni. Ayo daftarkan lomba yang ingin kamu publikasikan dan ajak teman-teman untuk mengunjungi website We-Come ya!"}</div>
				<Link<Route> to={Route::DaftarLomba}>
					<button class="px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Daftarkan Lomba"}</button>
				</Link<Route>>
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
