use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
	pub children: Children,
	pub is_admin: bool,
}

#[function_component(AuthLayout)]
pub fn auth_layout(props: &Props) -> Html {
	html! {
		<>
		<div class="absolute z-0 top-0 left-0 h-screen w-screen bg-[url('https://res.cloudinary.com/dw4bwn79m/image/upload/v1644714225/1590135987_i3defp.jpg')] bg-fixed bg-center bg-cover bg-no-repeat"></div>
		<div class="absolute z-2 top-0 left-0 h-screen w-screen bg-black opacity-70"></div>
		<div class={format!("absolute z-3 top-0 left-0 h-screen w-4 {}", if props.is_admin.clone() { "bg-[#FF5050]"} else { "bg-blue-gradient-app" })}></div>
		<div class="absolute z-4 top-0 left-0 h-screen w-screen">
			{ for props.children.iter() }
		</div>
		</>
	}
}
