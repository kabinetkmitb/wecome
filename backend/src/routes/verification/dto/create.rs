use crate::schema::verification;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable, Clone, Derivative)]
#[derivative(Default)]
#[table_name = "verification"]
pub struct CreateVerification {
    #[derivative(Default(value = "crate::utils::cuid::get_cuid()"))]
    pub id: String,
    pub user_id: String,
}
