use yew::prelude::*;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
	html! {
		<div class="w-screen p-4 flex flex-col items-center text-gray-600">
			<span>{"Kedirjenan Bestari Karya Kementrian"}</span>
			<span>{"Karya Inovasi KM ITB x Ditmawa ITB"}</span>
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
	}
}
