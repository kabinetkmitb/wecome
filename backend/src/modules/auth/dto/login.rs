use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub name: String,
    pub is_admin: bool,
    pub token: String,
}
