use yew::prelude::*;
use crate::router::Route;
use yew_router::prelude::*;

#[function_component(DaftarLombaSuccessComponent)]
pub fn daftar_lomba_success_component() -> Html {
    html! {
        <>
		<div class="py-16 md:px-8 md:h-auto w-screen flex relative bg-slate-400 sm:bg-white sm:shadow sm:justify-center">
			<div class="relative w-full sm:w-auto p-16 flex justify-center items-center">
				<img class="opacity-40 sm:opacity-100 min-w-[20rem]" src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1645275621/Group_113_bpllly.png" alt="Logo Web" />
			</div>
			<div class="absolute z-2 flex flex-col justify-center p-16 gap-3 w-screen sm:relative">
				<div class="font-bold text-[2rem] text-cyan-400">{"Berhasil"}</div>
				<p class="text-2xl text-justify">{"Data yang anda isi berhasil dikirimkan, silahkan tunggu 1 x 24 jam untuk mendapatkan konfirmasi terhadap pendaftaran lomba lembaga / kampus anda."}</p>
				<Link<Route> to={Route::Index}>
					<button class="px-4 w-fit py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Kembali ke Beranda"}</button>
				</Link<Route>>
			</div>
		</div>
        </>
    }
}
