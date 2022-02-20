use crate::components::common::form_field::FormField;
use crate::types::form::{FormFieldProperty, FormFieldType};
use std::collections::HashMap;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::{use_async, use_map, use_mount};

#[function_component(CompetitionTab)]
pub fn competition_tab() -> Html {
	let query = use_map(HashMap::from([
		("sort by".to_string(), "".to_string()),
		("category".to_string(), "".to_string()),
	]));

	let get_kompetisi = {
		use_async(async move {
			let response = crate::services::kompetisi::get_kompetisi("".to_string()).await;
			match response {
				Ok(response) => Ok(response),
				Err(e) => {
					crate::utils::interop::show_toast_with_message(e.to_string());
					Err(e)
				}
			}
		})
	};

	{
		let get_kompetisi = get_kompetisi.clone();
		use_effect_with_deps(
			move |_| {
				get_kompetisi.run();
				|| ()
			},
			query.clone(),
		);
	}

	{
		let get_kompetisi = get_kompetisi.clone();
		use_mount(move || {
			get_kompetisi.run();
		});
	}

	html! {
		<>
		<div class="text-2xl font-bold py-4">{"Competition List"}</div>
		<div class="flex flex-col w-full shadow-md rounded-md">
			<div class="grid grid-cols-2 gap-5 px-5 pt-2">
				<FormField
					field_property={
						FormFieldProperty {
							key: "sort by".to_string(),
							placeholder: Some("Field".to_string()),
							input_type: FormFieldType::Select {
								options: vec![
									"nama kompetisi".to_string(),
									"penyelenggara".to_string(),
									"kategori".to_string(),
									"tanggal submit".to_string(),
									"tanggal selesai".to_string(),
									"status".to_string(),
								]
							},
						}
					}
					key_input={"sort by".to_string()}
					form_data={query.clone()}
				/>
				<FormField
					field_property={
						FormFieldProperty {
							key: "order".to_string(),
							placeholder: Some("Asc/Dsc".to_string()),
							input_type: FormFieldType::Select {
								options: vec![
									"ascending".to_string(),
									"descending".to_string(),
								]
							},
						}
					}
					key_input={"category".to_string()}
					form_data={query.clone()}
				/>
			</div>
			<div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
				<div class="py-2 inline-block min-w-full sm:px-6 lg:px-8">
					<div class="overflow-hidden">
						<table class="min-w-full">
							<thead class="bg-white border-b">
								<tr>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"No"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Nama Kompetisi"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Penyelenggara"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Kategori"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Tanggal Submit"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Tanggal Selesai"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Status"}
								</th>
								<th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
									{"Option"}
								</th>
								</tr>
							</thead>
							<tbody>
							{
								if let Some(kompetisi_list) = &get_kompetisi.clone().data {
									let kompetisi_list = kompetisi_list.clone();
									if !kompetisi_list.clone().is_empty() {
										let kompetisi_list = kompetisi_list.clone();
										kompetisi_list.clone().into_iter().map(move |kompetisi| {
											html_nested! {
											<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
												<td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{"1"}</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().nama_kompetisi}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().nama_lembaga_pendaftar}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().kategori_kompetisi}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{"-"}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().batas_akhir_registrasi}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().status_kompetisi}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{"..."}
												</td>
											</tr>

											}
										}).collect::<VNode>()
									}
									else {
										html_nested! {
											<div class="text-gray-600">{"Tidak ada kompetisi"}</div>
										}
									}
								} else {
									html_nested! {
										<div class="text-gray-600">{"Loading..."}</div>
									}
								}
							}
							</tbody>
						</table>
					</div>
				</div>
			</div>
			<div class="py-5 px-2">
				<nav aria-label="Competition Pagination">
					<ul class="flex list-style-none">
					<li class="page-item disabled"><a
						class="page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-500 pointer-events-none focus:shadow-none"
						href="#" tabindex="-1" aria-disabled="true">{"Previous"}</a></li>
					<li class="page-item"><a
						class="page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						href="#">{"1"}</a></li>
					<li class="page-item active"><a
						class="page-link relative block py-1.5 px-3 border-0 bg-blue-600 outline-none transition-all duration-300 rounded-full text-white hover:text-white hover:bg-blue-600 shadow-md focus:shadow-md"
						href="#">{"2"} <span class="visually-hidden">{"(current)"}</span></a></li>
					<li class="page-item"><a
						class="page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						href="#">{"3"}</a></li>
					<li class="page-item"><a
						class="page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						href="#">{"Next"}</a></li>
					</ul>
				</nav>
			</div>
		</div>
		</>
	}
}
