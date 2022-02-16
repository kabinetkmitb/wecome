use crate::types::auth::{
	LoginPayload, LoginResponse, MeResponse, RegisterPayload, RegisterResponse,
};
use crate::types::error::Error;
use crate::utils::api::{request_get, request_post};

pub async fn register(register_payload: RegisterPayload) -> Result<RegisterResponse, Error> {
	request_post::<RegisterPayload, RegisterResponse>(
		"/auth/register".to_string(),
		register_payload,
	)
	.await
}

pub async fn login(login_payload: LoginPayload) -> Result<LoginResponse, Error> {
	request_post::<LoginPayload, LoginResponse>("/auth/login".to_string(), login_payload).await
}

pub async fn me() -> Result<MeResponse, Error> {
	request_get::<MeResponse>("/auth/me".to_string()).await
}
