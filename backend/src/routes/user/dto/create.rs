use crate::schema::user;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "user"]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub nim: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
}
