use crate::components::common::modal::Modal;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub kategori: &'static str,
	pub judul: &'static str,
	pub deskripsi: &'static str,
	pub tanggal_pelaksanaan: &'static str,
	pub batas_regist: &'static str,
}

#[function_component(KompetisiModal)]
pub fn kompetisi_modal(props: &Props) -> Html {
	html! {
		<Modal id="kompetisi-modal" title={props.judul}>
			<div class="p-5 flex flex-col md:grid md:grid-cols-2 justify-between w-full">
				<img src="https://picsum.photos/200/300.jpg" class="h-full" width="90%" alt="Logo lomba"/>
				<div class="flex flex-col gap-3">
				<div class="bg-yellow-400 w-fit text-white p-1 rounded-md">{props.kategori}</div>
				<div class="text-2xl font-bold">{props.judul}</div>
				<div class="text-justify">{props.deskripsi}</div>
				<div>
					<div class="font-bold">{"Tanggal Pelaksanaan"}</div>
					<div>{props.tanggal_pelaksanaan}</div>
				</div>
				<div>
					<div class="font-bold">{"Tanggal Batas Registrasi"}</div>
					<div>{props.batas_regist}</div>
				</div>
				<div class="flex gap-2 text-2xl">
					<i class="bx bx-world"></i>
					<i class="bx bxl-twitter"></i>
					<i class="bx bxl-linkedin"></i>
					<i class="bx bxl-instagram-alt"></i>
				</div>
				</div>
			</div>
		</Modal>
	}
}
