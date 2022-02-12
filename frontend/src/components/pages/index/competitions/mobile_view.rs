use yew::prelude::*;

pub struct LombaKategori {
	icon_src: String,
	name: String,
}

struct PaginationIndex {
	upper_bound: usize,
	lower_bound: usize,
}

fn to_lomba_kategori(icon_src: String, name: String) -> LombaKategori {
	LombaKategori { icon_src, name }
}

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
	let pagination = use_state(|| PaginationIndex {
		upper_bound: 1,
		lower_bound: 0,
	});

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

	let kategori_length = kategori_lomba.len();

	let left_click = {
		let pagination = pagination.clone();
		let upper_bound = (*pagination).upper_bound;
		let lower_bound = (*pagination).lower_bound;

		Callback::from(move |_| {
			pagination.set(PaginationIndex {
				upper_bound: if upper_bound as i32 + 2 > kategori_length as i32 - 1 {
					upper_bound + 2 - kategori_length
				} else {
					upper_bound + 2
				},
				lower_bound: if lower_bound as i32 + 2 > kategori_length as i32 - 1 {
					lower_bound + 2 - kategori_length
				} else {
					lower_bound + 2
				},
			})
		})
	};

	let right_click = {
		let pagination = pagination.clone();
		let upper_bound = (*pagination).upper_bound;
		let lower_bound = (*pagination).lower_bound;

		Callback::from(move |_| {
			pagination.set(PaginationIndex {
				upper_bound: if upper_bound as i32 - 2 < 0 {
					kategori_length + upper_bound - 2
				} else {
					upper_bound - 2
				},
				lower_bound: if lower_bound as i32 - 2 < 0 {
					kategori_length + lower_bound - 2
				} else {
					lower_bound - 2
				},
			})
		})
	};

	html! {
	<div class="w-screen [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)] flex flex-col">
		<div class="font-semibold text-[2.25rem] text-white text-center">{"Kategori"}</div>
		<div class="font-semibold text-[2.25rem] text-white text-center">{"Kompetisi"}</div>
		<br />
		<div class="flex justify-around items-center px-8">
			<div onclick={left_click} class="rounded-full w-8 h-8 flex justify-center items-center bg-white shadow-lg cursor-pointer">
				<i class="bx bx-left-arrow-alt"></i>
			</div>
			{
				for kategori_lomba[(*pagination).lower_bound...(*pagination).upper_bound]
				.iter()
				.map(|kategori| {
						let kategori = kategori.clone();
						let icon_src = &kategori.icon_src;
						let name = &kategori.name;
						html! {
							<div>
								<div class="bg-white w-24 h-24 drop-shadow-2xl rounded-lg flex justify-center items-center">
									<img src={String::from(icon_src)} alt="Event" />
								</div>
								<div class="text-center text-white font-medium">{name}</div>
							</div>
						}
					}
				)
			}
			<div onclick={right_click} class="rounded-full w-8 h-8 flex justify-center items-center bg-white shadow-lg cursor-pointer">
				<i class="bx bx-right-arrow-alt"></i>
			</div>
		</div>
		<br />
		<button class="bg-white py-2 px-4 rounded mx-auto w-auto drop-shadow font-semibold text-cyan-500">{"Lihat Semua"}</button>
		<br />
	</div>
	}
}
