use yew::prelude::*;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
	html! {
	<div class="w-screen [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)] flex flex-col">
		<div class="font-semibold text-[2.25rem] text-white text-center">{"Kategori"}</div>
		<div class="font-semibold text-[2.25rem] text-white text-center">{"Kompetisi"}</div>
		<br />
		<div class="flex justify-around items-center px-8">
			<div class="rounded-full w-8 h-8 flex justify-center items-center bg-white shadow-lg cursor-pointer">
			<i class="bx bx-left-arrow-alt"></i>
			</div>
			<div>
			<div class="bg-white py-4 px-8 drop-shadow-2xl rounded-lg">
				<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644607498/Frame_1_pfwwty.png" alt="Event" />
			</div>
			<div class="text-center text-white font-medium">{"Riset"}</div>
			</div>
			<div>
			<div class="bg-white py-4 px-8 drop-shadow-2xl rounded-lg">
				<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644607498/Frame_1_pfwwty.png" alt="Event" />
			</div>
			<div class="text-center text-white font-medium">{"Riset"}</div>
			</div>
			<div class="rounded-full w-8 h-8 flex justify-center items-center bg-white shadow-lg cursor-pointer">
			<i class="bx bx-right-arrow-alt"></i>
			</div>
		</div>
		<br />
		<button class="bg-white py-2 px-4 rounded mx-auto w-auto drop-shadow font-semibold text-cyan-500">{"Lihat Semua"}</button>
		<br />
	</div>
	}
}
