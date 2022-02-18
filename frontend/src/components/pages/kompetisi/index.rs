use super::modal::KompetisiModal;
use crate::components::common::modal_button::ModalButton;
use yew::{function_component, html};

#[function_component(KompetisiComponent)]
pub fn kompetisi_component() -> Html {
	html! {
		<>
		<KompetisiModal
			kategori="test"
			judul="test"
			deskripsi="test"
			tanggal_pelaksanaan="test"
			batas_regist="test"
		/>
		<div class="p-6 h-screen overflow-y-scroll relative z-10">
		<div class="flex items-center gap-2 text-[0.6rem] md:text-[1rem] max-w-[600px]">
			<div class="relative inline-block text-left">
			<div>
				<button type="button" class="md:px-4 md:py-2 md:text-sm md:font-medium inline-flex justify-center px-1 rounded-md border border-gray-300 shadow-sm bg-white font-medium text-gray-700 hover:bg-gray-50 focus:outline-none" id="menu-button" aria-expanded="true" aria-haspopup="true">
				<div class="pt-[1.15px] md:pt-0">{"Kategori"}</div>
				<svg class="-mr-1 ml-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
					<path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
				</svg>
				</button>
			</div>
			<div class="hidden origin-top-right absolute mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
				<div class="py-1" role="none">
				<a href="#" class="text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">{"Account settings"}</a>
				<a href="#" class="text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-1">{"Support"}</a>
				<a href="#" class="text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-2">{"License"}</a>
				<form method="POST" action="#" role="none">
					<button type="submit" class="text-gray-700 block w-full text-left px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-3">{"Sign out"}</button>
				</form>
				</div>
			</div>
			</div>
			<input class="appearance-none border rounded w-full p-1 md:px-4 md:py-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" />
			<button class="p-1 md:px-4 md:py-2 rounded-md hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Cari"}</button>
		</div>
		<div class="my-4 text-[0.6rem] md:text-[0.8rem] md:grid md:grid-cols-3 gap-3 flex flex-col">
			<div class="border-[1.25px] border-gray-300 shadow-sm rounded flex bg-white">
			<img class="w-[40%] object-cover" src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644714225/1590135987_i3defp.jpg" alt="Logo lomba"/>
			<div class="w-[60%] p-3 flex flex-col gap-1">
				<div class="py-1 w-fit tracking-widest px-2 text-[0.8em] rounded-2xl font-meidum bg-[#FECC30] text-white text-center inline-block">{"Kategori lomba"}</div>
				<div class="font-bold text-[1.7em] md:text-[1.2em]">{"Nama Kompetisi"}</div>
				<div class="justify">{"This is a wider card with supporting text below as a . . ."}</div>
				<div class="justify">{"Pelaksanaan : 01 Januari 2000"}</div>
				<div class="justify">{"Batas Registrasi : 01 Januari 2000"}</div>
				<ModalButton modal_id="kompetisi-modal" class="cursor-pointer mt-2 p-1 w-fit rounded-md hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Detail Kompetisi"}</ModalButton>
			</div>
			</div>
		</div>
		</div>
		<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Background logo" class="absolute bottom-20 left-[-6rem] opacity-40 z-0" style="transform: rotate(-12.79deg);"/>
		</>
	}
}
