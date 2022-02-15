use crate::schema::verification;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "verification"]
pub struct CreateVerification {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    pub code: String,
}
