use yew::prelude::*;

#[function_component(DesktopView)]
pub fn desktop_view() -> Html {
	html! {
		<div class="px-20 py-10 bg-white shadow-2xl drop-shadow-2xl">
			<div class="flex justify-between items-center">
				<div>
				<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644652873/Group_172_j1wmbu.png" alt="Logo footer"/>
				<div class="text-gray-600 font-medium">{"Kedirjenan Bestari Karya"}</div>
				<div class="text-gray-600 font-medium">{"Kementrian Inkubasi Penerapan Karya Kemenkoan Karya Inovasi KM ITB Akar Asa 2021/2022 x DITMAWA ITB"}</div>
				</div>
				<div>
				<div class="text-gray-600 font-medium">{"Kontak Kami"}</div>
					<div class="flex gap-3 m-2">
					<a href="#">
						<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644642617/Font_Awesome_Icon_1_zctt1t.png" alt="Linked in"/>
					</a>
					<a href="#">
						<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644642616/Font_Awesome_Icon_2_q7vl59.png" alt="Linked in"/>
					</a>
					<a href="#">
						<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644642618/Font_Awesome_Icon_gibp70.png" alt="Linked in"/>
					</a>
						</div>
				</div>
			</div>
			<div class="my-4 h-[2px] w-full bg-gray-600 opacity-50"></div>
			<div class="text-gray-600 font-medium">{"© Kedirjenan Bestari Karya | We-come 2021"}</div>
		</div>
	}
}
