use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MemberWecome {
	pub photo_src: String,
	pub name: String,
	pub title: String,
}

fn to_member_wecome(
	name: &'static str,
	title: &'static str,
	photo_src: &'static str,
) -> MemberWecome {
	MemberWecome {
		photo_src: photo_src.to_string(),
		name: name.to_string(),
		title: title.to_string(),
	}
}

#[function_component(Organogram)]
pub fn organogram() -> Html {
	let wecome_members = vec![
		to_member_wecome("Taufiq Pangestu - TG’19", "Minister of Incubation & Creation Implementation", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645983972/Taufiq_Pangestu_rapi_dikit_Cropped_dt0nak.png"),
		to_member_wecome("Fathimah Afra Nailah Adma - AK’20", "Director General of  Bestari Karya", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645983949/Fathimah_Afra_Vice_President_of_Eksternal_1_cover_-_Afra_Cropped_tcuxhk.jpg"),
		to_member_wecome("Kevin Sean Hans L - MA’20", "Vice Director General of Bestari Karya", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645983908/Kevin_Sean_Hans_L._-_Deputy_Director_General_of_BK_-_Kevin_Sean_Cropped_yrrhm2.png"),
		to_member_wecome("Syifa Melati - TI’20", "Coordinator of  We-Come & B-Buddies", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645984231/syifa_cropped_hcxg3e.png"),
		to_member_wecome("Felix Fernando - MA’20", "UI/UX Designer of We-Come & B-Buddies", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645983967/IMG-20220122-WA0000_Cropped_tw8aeo.jpg"),
		to_member_wecome("Muhammad Sulthan Mazaya - TF’20", "Website Developer", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645983904/1607353677355_-_Muhammad_Sulthan_Mazaya_1_Cropped_tswa1p.jpg"),
		to_member_wecome("Muhammad Risqi Firdaus - IF’20", "Website Developer", "https://res.cloudinary.com/dw4bwn79m/image/upload/a_90/v1645983906/20220225_132342_-_13520043_Muhammad_Risqi_Firdaus_Cropped_h26xve.jpg"),
		to_member_wecome("Riandy Hasan - STI’20", "Website Developer", "https://res.cloudinary.com/dw4bwn79m/image/upload/v1645983909/S__3112966_-_18220058_Riandy_Hasan_Cropped_dhvhl0.jpg"),
	];

	html! {
		<div class="w-full bg-blue-gradient-app flex flex-col items-center py-10">
			<div class="m-4 relative">
				<div class="text-[2rem] font-semibold text-white text-center mb-5 mr-[4rem]">
				{"Organogram"}
				</div>
				<div class="absolute bg-white bottom-0 right-0 rounded-xl text-cyan-400 p-1 font-medium text-[0.7rem]">{"People behind we-come"}</div>
			</div>
			<div class="grid grid-cols-2 gap-10 md:grid-cols-4 my-10">
				{
					for wecome_members.iter().map(|member|
						html! {
							<div class="flex flex-col items-center text-sm text-white text-center">
								<img
									src={member.clone().photo_src}
									class="w-32 h-32 rounded-sm mb-4 shadow-xl drop-shadow-md"
									alt="Wecome Member Avatar"
								/>
								<div class="font-medium">{member.clone().name}</div>
								<div class="max-w-[16rem]">{member.clone().title}</div>
							</div>
						})
				}
			</div>
		</div>
	}
}
