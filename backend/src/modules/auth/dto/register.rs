use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterInput {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}
