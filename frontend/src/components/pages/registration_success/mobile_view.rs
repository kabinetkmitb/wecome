use yew::prelude::*;

#[function_component(MobileView)]
pub fn mobile_view() -> Html {
	html! {
		<>
		<div class="min-h-[97vh] flex flex-col w-full items-center">
		<div class="w-52 p-5 opacity-40">
			<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644641089/Frame_l1vboh.png" alt="Logo Wecome" />
		</div>
		<div class="h-[1.5px] w-[90%] bg-gray-600 opacity-50"></div>
		<div class="p-5 flex flex-col w-full items-center justify-center">
			<lottie-player src="https://assets6.lottiefiles.com/packages/lf20_wcnjmdp1.json"  background="transparent"  speed="1"  style="width: 250px; height: 250px;"  autoplay={true}></lottie-player>
			<div class="text-2xl mb-3 font-bold">{"Registrasi Sukses!"}</div>
			<div>{"Untuk melanjutkan penggunaan akun, silakan membuka email dan menekan link verifikasi yang sudah tersedia."}</div>
		</div>
		</div>
		<div class="w-full h-6 [background:linear-gradient(139.53deg,_#32D0FA_0%,_#44A2FE_100%)]"></div>
		</>
	}
}
