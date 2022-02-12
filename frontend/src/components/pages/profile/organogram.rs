use yew::prelude::*;

#[function_component(Organogram)]
pub fn organogram() -> Html {
	html! {
		<div class="w-full [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)] flex flex-col items-center py-10">
			<div class="m-4 relative">
				<div class="text-[2rem] font-semibold text-white text-center mb-5 mr-[4rem]">
				{"Organogram"}
				</div>
				<div class="absolute bg-white bottom-0 right-0 rounded-xl text-cyan-400 p-1 font-medium text-[0.7rem]">{"People behind we-come"}</div>
			</div>
			<div class="grid grid-cols-2 gap-10 md:grid-cols-4 my-10">
				{
					for (1...10).map(|_|
						html! {
							<div class="flex flex-col items-center text-sm text-white text-center">
								<div class="bg-white w-32 h-32 rounded-sm"></div>
								<div class="font-medium">{"Nama Lengkap"}</div>
								<div>{"Menko Karya Inovasi KM ITB"}</div>
							</div>
						})
				}
			</div>
		</div>
	}
}
