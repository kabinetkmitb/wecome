use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterInput {
    pub email: String,
    pub nim: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub verification_id: String,
}
