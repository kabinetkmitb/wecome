use super::nav_button::NavButton;
use crate::router::Route;
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! {
	<header>
	   <nav
		  class="
		  shadow-lg
          flex flex-wrap
          items-center
          justify-between
          w-full
          py-4
          px-8
          text-lg text-gray-700
          bg-white
        "
		>
			<div class="flex gap-2">
					<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644482777/logo_itb_1024_1_wo9wmm.png" alt="logo"/>
					<img src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644482768/Group_98_umtaom.png" alt="logo"/>
			</div>
		   <svg
			  xmlns="http://www.w3.org/2000/svg"
			  id="menu-button"
			  class="h-6 w-6 cursor-pointer md:hidden block"
			  fill="none"
			  viewBox="0 0 24 24"
			  stroke="currentColor"
			>
			  <path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M4 6h16M4 12h16M4 18h16"
			  />
			</svg>

		 <div class="hidden w-full md:flex md:items-center md:w-auto" id="menu">
			<ul
			  class="
              pt-4
              text-base text-gray-700
              md:flex
              md:justify-between 
              md:pt-0"
			>
			  <li>
				<NavButton route={Route::Index} path_to_match={String::from("/")} pathname={String::from("Beranda")} class="md:p-4 py-2 block text-cyan-400 hover:text-yellow-400 font-bold transition"/>
			  </li>
			  <li>
				<NavButton route={Route::Profile} path_to_match={String::from("/profile")} pathname={String::from("Profile")} class="md:p-4 py-2 block text-cyan-400 hover:text-yellow-400 font-bold transition"/>
			  </li>
			  <li>
				<NavButton route={Route::Profile} pathname={String::from("Register")} class="px-4 py-2 m-2 rounded-lg text-cyan-400 border-2 block hover:text-white hover:bg-cyan-400 border-cyan-400 font-bold transition"/>
			  </li>
			  <li>
				<NavButton route={Route::Profile} pathname={String::from("Login")} class="px-4 py-2 m-2 rounded-lg text-white border-2 block hover:text-cyan-400 hover:bg-white bg-cyan-400 border-cyan-400 font-bold transition"/>
			  </li>
			</ul>
		  </div>
	  </nav>
	</header>
	}
}
