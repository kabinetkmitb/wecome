use crate::components::common::markdown::Markdown;
use crate::components::common::modal::Modal;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
	pub kategori: String,
	pub judul: String,
	pub deskripsi: String,
	pub tanggal_pelaksanaan: String,
	pub batas_regist: String,
	pub link_poster: String,
	pub link_website: String,
	pub link_linkedin: String,
	pub link_twitter: String,
	pub link_instagram: String,
	pub link_regist: String,
}

#[function_component(KompetisiModal)]
pub fn kompetisi_modal(props: &Props) -> Html {
	let props = props.clone();
	html! {
		<Modal id="kompetisi-modal" title={props.clone().judul.clone()}>
			<div class="p-5 flex flex-col md:grid md:grid-cols-2 justify-between w-full">
				<img src={props.link_poster} class="h-full" width="90%" alt="Logo lomba"/>
				<div class="flex flex-col gap-3">
				<div class="bg-yellow-400 w-fit text-white p-1 rounded-md">{props.kategori.as_str()}</div>
				<div class="text-2xl font-bold">{props.judul.as_str()}</div>
				<div class="text-justify prose">
					<Markdown html={props.deskripsi}/>
				</div>
				<div>
					<div class="font-bold">{"Tanggal Pelaksanaan"}</div>
					<div>{props.tanggal_pelaksanaan.as_str()}</div>
				</div>
				<div>
					<div class="font-bold">{"Tanggal Batas Registrasi"}</div>
					<div>{props.batas_regist.as_str()}</div>
				</div>
				<div class="font-bold">{"Kontak Penyelenggara"}</div>
				<div class="flex gap-2 text-2xl">
					<a href={props.link_website} target="_blank">
						<i class="bx bx-world"></i>
					</a>
					<a href={props.link_twitter} target="_blank">
						<i class="bx bxl-twitter"></i>
					</a>
					<a href={props.link_linkedin} target="_blank">
						<i class="bx bxl-linkedin"></i>
					</a>
					<a href={props.link_instagram} target="_blank">
						<i class="bx bxl-instagram-alt"></i>
					</a>
				</div>
					<a href={props.link_regist} target="_blank" class="w-fit px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Daftar"}</a>
				</div>
			</div>
		</Modal>
	}
}
