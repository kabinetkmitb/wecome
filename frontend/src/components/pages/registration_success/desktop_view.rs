use crate::components::common::auth_layout::AuthLayout;
use yew::prelude::*;

#[function_component(DesktopView)]
pub fn desktop_view() -> Html {
	html! {
		<AuthLayout is_admin={false}>
			<div class="ml-4 p-10 min-h-screen bg-white w-[50vw] flex flex-col justify-center items-center gap-4">
				<lottie-player src="https://assets6.lottiefiles.com/packages/lf20_wcnjmdp1.json"  background="transparent"  speed="1"  style="width: 300px; height: 300px;"  autoplay={true}></lottie-player>
				<div class="text-2xl font-bold">{"Registrasi Sukses!"}</div>
				<div>{"Untuk melanjutkan penggunaan akun, silakan membuka email dan menekan link verifikasi yang sudah tersedia."}</div>
			</div>
		</AuthLayout>
	}
}
