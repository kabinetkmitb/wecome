use crate::router::Route;
use crate::utils::api::set_token;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum UserAction {
	Set {
		name: String,
		is_admin: bool,
		token: String,
	},
	Clear,
}

#[derive(PartialEq, Default, Clone)]
pub struct UserState {
	pub name: String,
	pub token: String,
	pub is_admin: bool,
}

impl Reducible for UserState {
	type Action = UserAction;

	fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
		let next_user = match action {
			UserAction::Set {
				name,
				token,
				is_admin,
			} => UserState {
				name,
				token,
				is_admin,
			},
			UserAction::Clear => UserState::default(),
		};

		Self { ..next_user }.into()
	}
}

pub struct UseUser {
	inner: UseReducerHandle<UserState>,
	history: AnyHistory,
}

impl UseUser {
	pub fn login(&self, value: UserState) {
		// Set global token after logged in
		set_token(Some(value.token.clone()));
		self.inner.dispatch(UserAction::Set {
			name: value.name,
			token: value.token,
			is_admin: value.is_admin,
		});

		// Redirect to home page
		self.history.push(Route::Index);
	}

	pub fn logout(&self) {
		// Clear global token after logged out
		set_token(None);
		self.inner.dispatch(UserAction::Clear);

		// Redirect to home page
		self.history.push(Route::Index);
	}

	pub fn is_logged_in(&self) -> bool {
		!self.inner.token.is_empty()
	}
}

pub fn use_user() -> UseUser {
	let inner = use_context::<UseReducerHandle<UserState>>().unwrap();
	let history = use_history().unwrap();

	UseUser { inner, history }
}
