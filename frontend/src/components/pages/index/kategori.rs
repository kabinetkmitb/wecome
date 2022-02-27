use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LombaKategori {
	pub src: String,
	pub title: String,
	pub description: String,
}

fn to_lomba_kategori(src: String, title: String, description: String) -> LombaKategori {
	LombaKategori {
		src,
		title,
		description,
	}
}

#[function_component(Kategori)]
pub fn kategori() -> Html {
	let kategori_lomba = vec![
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644607498/Frame_1_pfwwty.png",
			),
			String::from("riset"),
			String::from("Perlombaan yang mencakup Karya Tulis Ilmiah, Energi, dan IPTEK"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641358/Frame_2_qfhdmq.png",
			),
			String::from("bisnis"),
			String::from("Perlombaan yang mencakup Business Competition dan Business Case"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641100/Color_Pallete_m7klnt.png",
			),
			String::from("seni"),
			String::from("Perlombaan yang mencakup tentang seni, baik poster, mural, vokal, hingga tari"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641102/Group_80_r4hjdj.png",
			),
			String::from("akademik"),
			String::from("Perlombaan yang mencakup tentang Olimpiade Sains ataupun Pemecahan masalah tertentu"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641365/Group_119_fxnn0x.png",
			),
			String::from("olahraga"),
			String::from("Perlombaan yang berhubungan dengan aktivitas olahraga"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641363/Frame_3_zzsb2w.png",
			),
			String::from("bahasa"),
			String::from("Perlombaan yang berhubungan dengan sastra, puisi, dan bahasa baik nasional maupun internasional"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641368/image_11_auruqz.png",
			),
			String::from("konferensi"),
			String::from("Perlombaan yang mencakup lomba MUN atau Debate Competition"),
		),
		to_lomba_kategori(
			String::from(
				"https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641355/Group_87_zu03ow.png",
			),
			String::from("teknologi"),
			String::from("Perlombaan yang berhubungan dengan teknologi seperti Hackathon atau IT kompetisi"),
		),
	];

	html! {
		<div class="flex-col p-5 mt-10">
		<div class="flex gap-6 justify-center text-[2.25rem] md:text-[4rem] font-bold mb-8">
		  {"Kategori"} <div class="text-cyan-400">{"Lomba"}</div>
		</div>
		<div class="flex justify-center items-center sm:px-5 md:px-20">
		  <div class="grid sm:grid-cols-1 md:grid-cols-2 sm:gap-6 md:gap-16">
			{
				for kategori_lomba.iter().map(|kategori| {
					let kategori = kategori.clone();
					html_nested! {
						<div class="md:m-0 m-5 max-w-lg bg-blue-gradient-app rounded-lg flex items-center p-6 gap-8 text-white">
							<div class="w-[10rem] drop-shadow-2xl rounded-lg flex justify-center items-center bg-white p-4">
								<img src={kategori.clone().src} alt="Kategori" />
							</div>
							<div class="flex-col gap-y-15">
								<div class="text-2xl font-bold mb-2 capitalize">{kategori.clone().title}</div>
								<div>{kategori.clone().description}</div>
							</div>
						</div>
					}
				})
			}


		  </div>
		</div>
	  </div>
	}
}
