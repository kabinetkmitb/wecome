use super::desktop_view::DesktopView;
use super::mobile_view::MobileView;
use crate::utils::hooks::get_window_size;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LombaKategori {
	pub icon_src: String,
	pub name: String,
}

fn to_lomba_kategori(icon_src: String, name: String) -> LombaKategori {
	LombaKategori { icon_src, name }
}

#[function_component(Competitions)]
pub fn competitions() -> Html {
	let window_size = get_window_size();

	let kategori_lomba = vec![
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644607498/Frame_1_pfwwty.png",
			),
			String::from("Riset"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641100/Color_Pallete_m7klnt.png",
			),
			String::from("Seni"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641102/Group_80_r4hjdj.png",
			),
			String::from("Akademik"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641355/Group_87_zu03ow.png",
			),
			String::from("Teknologi"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641358/Frame_2_qfhdmq.png",
			),
			String::from("Bisnis"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641363/Frame_3_zzsb2w.png",
			),
			String::from("Bahasa"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641365/Group_119_fxnn0x.png",
			),
			String::from("Olahraga"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641368/image_11_auruqz.png",
			),
			String::from("Konferensi"),
		),
	];

	let kategori_lomba_length = kategori_lomba.len();

	if window_size.width > 650.0 {
		html! {
			<DesktopView {kategori_lomba} />
		}
	} else {
		html! {
			<MobileView {kategori_lomba} {kategori_lomba_length}/>
		}
	}
}
