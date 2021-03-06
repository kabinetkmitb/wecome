use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct RegisterPayload {
	pub email: String,
	pub name: String,
	pub nim: String,
	pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct RegisterResponse {
	pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct LoginPayload {
	pub email: String,
	pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct LoginResponse {
	pub id: String,
	pub token: String,
	pub name: String,
	pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MeResponse {
	pub user_id: String,
	pub name: String,
	pub is_admin: bool,
}
