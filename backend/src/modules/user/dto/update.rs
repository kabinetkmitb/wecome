use crate::schema::user;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Clone, Debug, Derivative, AsChangeset)]
#[derivative(Default)]
#[table_name = "user"]
pub struct UpdateUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub is_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "None"))]
    pub is_active: Option<bool>,
}
