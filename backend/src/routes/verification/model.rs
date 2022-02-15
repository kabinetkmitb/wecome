use crate::schema::verification;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable, Clone)]
#[table_name = "verification"]
pub struct Verification {
    pub id: String,
    pub user_id: String,
}
