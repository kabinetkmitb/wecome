use super::competition::CompetitionTab;
use yew::prelude::*;

#[function_component(AdminComponent)]
pub fn admin_component() -> Html {
	crate::utils::interop::use_tw();
	html! {
		<div class="p-[3rem] w-full bg-white">
		<ul class="nav nav-tabs flex flex-col md:flex-row flex-wrap list-none border-b-0 pl-0 mb-4" id="tabs-tab"
			role="tablist">
			<li class="nav-item" role="presentation">
				<a href="#tabs-home" class="
				nav-link
				block
				font-medium
				text-xs
				leading-tight
				uppercase
				border-x-0 border-t-0 border-b-2 border-transparent
				px-6
				py-3
				my-2
				hover:border-transparent hover:bg-gray-100
				focus:border-transparent
				active
				" id="tabs-home-tab" data-bs-toggle="pill" data-bs-target="#tabs-home" role="tab" aria-controls="tabs-home"
				aria-selected="true">{"Competition"}</a>
			</li>
			<li class="nav-item" role="presentation">
				<a href="#tabs-profile" class="
				nav-link
				block
				font-medium
				text-xs
				leading-tight
				uppercase
				border-x-0 border-t-0 border-b-2 border-transparent
				px-6
				py-3
				my-2
				hover:border-transparent hover:bg-gray-100
				focus:border-transparent
				" id="tabs-profile-tab" data-bs-toggle="pill" data-bs-target="#tabs-profile" role="tab"
				aria-controls="tabs-profile" aria-selected="false">{"Teams"}</a>
			</li>
			<li class="nav-item" role="presentation">
				<a href="#tabs-messages" class="
				nav-link
				block
				font-medium
				text-xs
				leading-tight
				uppercase
				border-x-0 border-t-0 border-b-2 border-transparent
				px-6
				py-3
				my-2
				hover:border-transparent hover:bg-gray-100
				focus:border-transparent
				" id="tabs-messages-tab" data-bs-toggle="pill" data-bs-target="#tabs-messages" role="tab"
				aria-controls="tabs-messages" aria-selected="false">{"Database"}</a>
			</li>
		</ul>
		<div class="tab-content" id="tabs-tabContent">
			<div class="tab-pane fade show active" id="tabs-home" role="tabpanel" aria-labelledby="tabs-home-tab">
				<CompetitionTab/>
			</div>
			<div class="tab-pane fade" id="tabs-profile" role="tabpanel" aria-labelledby="tabs-profile-tab">
				{"Teams"}
			</div>
			<div class="tab-pane fade" id="tabs-messages" role="tabpanel" aria-labelledby="tabs-profile-tab">
				{"Database"}
			</div>
		</div>
		</div>
	}
}
