use crate::schema::user;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Clone, Debug, Derivative)]
#[derivative(Default)]
#[table_name = "user"]
pub struct CreateUser {
    #[derivative(Default(value = "crate::utils::cuid::get_cuid()"))]
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}
