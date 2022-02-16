//! User context provider.

use crate::utils::api::{get_token, set_token};
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use super::context::{UserAction, UserState};
use crate::services::auth::*;
use crate::types::error::Error;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
	let user_ctx = use_reducer(UserState::default);
	let current_user = use_async(async move { me().await });

	{
		let current_user = current_user.clone();
		use_mount(move || {
			if get_token().is_some() {
				current_user.run();
			}
		});
	}

	{
		let user_ctx = user_ctx.clone();
		use_effect_with_deps(
			move |current_user| {
				if let Some(user_info) = &current_user.data {
					let user_info = user_info.clone();
					user_ctx.dispatch(UserAction::Set {
						name: user_info.name.clone(),
						is_admin: user_info.is_admin.clone(),
						token: user_ctx.clone().token.clone(),
					});
				}

				if let Some(error) = &current_user.error {
					match error {
						Error::Unauthorized | Error::Forbidden => set_token(None),
						_ => (),
					}
				}
				|| ()
			},
			current_user,
		)
	}

	html! {
		<ContextProvider<UseReducerHandle<UserState>> context={user_ctx}>
			{ for props.children.iter() }
		</ContextProvider<UseReducerHandle<UserState>>>
	}
}
