use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
	pub html: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &Props) -> Html {
	let div = gloo_utils::document().create_element("div").unwrap();
	div.set_inner_html(&props.html.clone());

	Html::VRef(div.into())
}
