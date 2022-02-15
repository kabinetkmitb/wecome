use crate::schema::user;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable, Clone)]
#[table_name = "user"]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub nim: String,
    pub password: String,
    pub is_admin: bool,
    pub is_active: bool,
}
