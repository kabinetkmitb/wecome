use crate::router::Route;
use crate::utils::api::set_token;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum UserAction {
	Set {
		name: String,
		id: String,
		is_admin: bool,
		token: String,
	},
	Clear,
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct UserState {
	pub id: String,
	pub name: String,
	pub token: String,
	pub is_admin: bool,
}

impl UserState {
	pub fn is_logged_in(&self) -> bool {
		log::debug!("self {:?}", self);
		log::debug!("is logged in: {:?}", !self.token.is_empty());
		!self.token.is_empty()
	}
}

impl Reducible for UserState {
	type Action = UserAction;

	fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
		let next_user = match action {
			UserAction::Set {
				id,
				name,
				token,
				is_admin,
			} => UserState {
				name,
				token,
				is_admin,
				id,
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
			id: value.id,
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
}

impl std::ops::Deref for UseUser {
	type Target = UserState;

	fn deref(&self) -> &Self::Target {
		&(*self.inner)
	}
}

impl Clone for UseUser {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			history: self.history.clone(),
		}
	}
}

impl PartialEq for UseUser {
	fn eq(&self, other: &Self) -> bool {
		*self.inner == *other.inner
	}
}

impl std::fmt::Debug for UseUser {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UseUser")
			.field("value", &format!("{:?}", *self.inner))
			.finish()
	}
}

pub fn use_user() -> UseUser {
	let inner = use_context::<UseReducerHandle<UserState>>().unwrap();
	let history = use_history().unwrap();

	UseUser { inner, history }
}
