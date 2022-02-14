use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterInput {
    pub email: String,
    pub nim: String,
    pub nama: String,
    pub password: String,
}
