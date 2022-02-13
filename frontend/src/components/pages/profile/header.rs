use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
	html! {
		<div class="p-[3rem] w-full bg-white flex flex-col md:flex-row md:justify-around items-center">
			<img class="md:h-[20rem]" src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644655516/logo-wecome-profile_iqa8j0.png" alt="We come logo"/>
			<div class="flex flex-col justify-start w-full md:w-[40vw]">
				<div class="text-cyan-400 font-medium text-2xl">{"Apa itu"}</div>
				<div class="font-medium text-[2.5rem]">{"WE-COME"}</div>
				<div class="text-sm text-justify">{"We-Come adalah sebuah karya yang dipersembahkan oleh Kedirjenan Bestari Karya, Kementerian Inkubasi Penerapan dan Karya, Kemenkoan Karya Inovasi Kabinet Akar Asa KM ITB 2021/2022. We-Come hadir dan bekerja sama dengan Ditmawa ITB sehingga website ini akan terhubung dengan website Ditmawa. Website Competition atau disingkat menjadi We-Come, memiliki arti 'Kami Datang', menjadi satu pintu kompetisi diperuntukkan bagi seluruh massa kampus ITB serta berperan sebagai wadah informasi kompetisi di bidang akademik, teknologi, business plan & business case, riset, konferensi, olahraga dan seni demi memenuhi kebutuhan massa KM ITB terhadap informasi lomba. Dengan adanya fasilitas ini, massa kampus diharapkan dapat terekspos oleh informasi-informasi seputar perlombaan dan mendapatkan akses memadai terhadap kesempatan tersebut."}</div>
			</div>
		</div>
	}
}
