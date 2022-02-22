use crate::components::common::form_field::FormField;
use crate::types::form::{FormFieldProperty, FormFieldType};
use std::collections::HashMap;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::{use_async, use_counter, use_map, use_mount, use_toggle};

#[function_component(CompetitionTab)]
pub fn competition_tab() -> Html {
	crate::utils::interop::use_toast();
	let query = use_map(HashMap::from([
		("sort by".to_string(), "".to_string()),
		("order".to_string(), "".to_string()),
	]));

	let pagination = use_counter(1);

	let get_kompetisi = {
		let query = query.clone();
		let pagination = pagination.clone();
		use_async(async move {
			let query = query.clone();
			let pagination = pagination.clone();
			let page: i32 = 10;
			let response =
				crate::services::kompetisi::get_kompetisi(crate::utils::api::admin_pagination(
					page,
					(*pagination.clone() - 1) as i64 * page as i64,
					query.current().get("sort by").unwrap().to_string(),
					query.current().get("order").unwrap().to_string(),
				))
				.await;
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
		<div class="flex flex-col w-full shadow-md rounded-md relative">
			<div class="grid grid-cols-2 gap-5 px-5 pt-2" onclick={
				let get_kompetisi = get_kompetisi.clone();
				move |_| {
					let get_kompetisi = get_kompetisi.clone();
					get_kompetisi.run();
				}
			}>
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
					key_input={"order".to_string()}
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
									let get_kompetisi = get_kompetisi.clone();
									let kompetisi_list = kompetisi_list.clone();
									let pagination = pagination.clone();
									if !kompetisi_list.clone().is_empty() {
										let pagination = pagination.clone();
										let get_kompetisi = get_kompetisi.clone();
										let kompetisi_list = kompetisi_list.clone();
										kompetisi_list.clone().into_iter().enumerate().map(move |(index,kompetisi)| {
											html_nested! {
											<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
												<td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{(index+1) + 10 * ((*pagination-1) as usize)}</td>
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
													{kompetisi.clone().created_at}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().batas_akhir_registrasi}
												</td>
												<td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
													{kompetisi.clone().status_kompetisi}
												</td>
												{
													{
														let get_kompetisi = get_kompetisi.clone();
														let kompetisi = kompetisi.clone();
														let toggle = use_toggle("hidden", "");

														let accept_kompetisi = {
															let kompetisi = kompetisi.clone();
															let get_kompetisi = get_kompetisi.clone();
															use_async(async move {
																let kompetisi = kompetisi.clone();
																let get_kompetisi = get_kompetisi.clone();
																let response =
																	crate::services::kompetisi::accept_kompetisi(kompetisi.id).await;
																get_kompetisi.run();
																match response {
																	Ok(response) => Ok(response),
																	Err(e) => {
																		crate::utils::interop::show_toast_with_message(e.to_string());
																		Err(e)
																	}
																}
															})
														};

														let decline_kompetisi = {
															let get_kompetisi = get_kompetisi.clone();
															let kompetisi = kompetisi.clone();
															use_async(async move {
																let get_kompetisi = get_kompetisi.clone();
																let kompetisi = kompetisi.clone();
																let response =
																	crate::services::kompetisi::decline_kompetisi(kompetisi.id).await;
																get_kompetisi.run();
																match response {
																	Ok(response) => Ok(response),
																	Err(e) => {
																		crate::utils::interop::show_toast_with_message(e.to_string());
																		Err(e)
																	}
																}
															})
														};
														html! {
															<div class="inline-block text-left">
																<td class="text-gray-900 text-2xl cursor-pointer font-light px-6 py-4 whitespace-nowrap">
																<i class="bx bx-dots-horizontal-rounded" onclick={
																	let toggle = toggle.clone();
																	move |_| toggle.toggle()
																}></i>
																	<div class={
																		let toggle = toggle.clone();
																		format!("{} z-10 right-0 origin-top-right absolute mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none", *toggle)
																	} role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
																		<div class="py-1" role="none">
																			<div onclick={
																				let toggle = toggle.clone();
																				let accept_kompetisi = accept_kompetisi.clone();
																				move |_| {
																					let toggle = toggle.clone();
																					let accept_kompetisi = accept_kompetisi.clone();
																					accept_kompetisi.run();
																					toggle.toggle();
																				}
																			} class="text-gray-700 hover:bg-gray-100 hover:text-gray-900 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">{"Publish competition"}</div>
																			<div onclick={
																				let decline_kompetisi = decline_kompetisi.clone();
																				let toggle = toggle.clone();
																				move |_| {
																					let toggle = toggle.clone();
																					let decline_kompetisi = decline_kompetisi.clone();
																					decline_kompetisi.run();
																					toggle.toggle();
																				}
																			} class="text-gray-700 hover:bg-gray-100 hover:text-gray-900 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-1">{"Decline competition"}</div>
																			<div class="text-gray-700 hover:bg-gray-100 hover:text-gray-900 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-2">{"See more details"}</div>
																		</div>
																	</div>
																</td>
															</div>
														}
													}
												}
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
					<li class="page-item "><div
						onclick={
							let pagination = pagination.clone();
							let get_kompetisi = get_kompetisi.clone();
							move |_| {
								let get_kompetisi = get_kompetisi.clone();
								let pagination = pagination.clone();
								if *pagination > 1 {
									pagination.decrease();
								}
								get_kompetisi.run();
							}
						}
						class="cursor-pointer page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						tabindex="-1">{"Previous"}</div></li>
					<li class="page-item"><div
						onclick={
							let pagination = pagination.clone();
							let get_kompetisi = get_kompetisi.clone();
							move |_| {
								let get_kompetisi = get_kompetisi.clone();
								let pagination = pagination.clone();
								if *pagination > 1 {
									pagination.decrease();
								}
								get_kompetisi.run();
							}
						}
						class="cursor-pointer page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						>{(*pagination.clone())-1}</div></li>
					<li class="page-item active"><div
						class="cursor-pointer page-link relative block py-1.5 px-3 border-0 bg-blue-600 outline-none transition-all duration-300 rounded-full text-white hover:text-white hover:bg-blue-600 shadow-md focus:shadow-md"
						>{*pagination.clone()} <span class="visually-hidden">{"(current)"}</span></div></li>
					<li class="page-item"><div
						onclick={
							let pagination = pagination.clone();
							let get_kompetisi = get_kompetisi.clone();
							move |_| {
								let get_kompetisi = get_kompetisi.clone();
								let pagination = pagination.clone();
								pagination.increase();
								get_kompetisi.run();
							}
						}
						class="cursor-pointer page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						>{(*pagination.clone())+1}</div></li>
					<li class="page-item"><div
						onclick={
							let pagination = pagination.clone();
							let get_kompetisi = get_kompetisi.clone();
							move |_| {
								let get_kompetisi = get_kompetisi.clone();
								let pagination = pagination.clone();
								pagination.increase();
								get_kompetisi.run();
							}
						}
						class="cursor-pointer page-link relative block py-1.5 px-3 border-0 bg-transparent outline-none transition-all duration-300 rounded-full text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none"
						>{"Next"}</div></li>
					</ul>
				</nav>
			</div>
		</div>
		</>
	}
}
