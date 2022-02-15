use serde::{Deserialize, Serialize};

use super::token::Token;

#[derive(Deserialize, Serialize)]
pub struct RegisterInput {
    pub email: String,
    pub nim: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub token: Token,
    pub verification_id: String,
}
