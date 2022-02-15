use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{dto::create::CreateVerification, model::Verification};

pub fn create_verification<'a>(
    connection: &UnwrappedPool,
    verification_data: CreateVerification,
) -> Result<Verification, diesel::result::Error> {
    use crate::schema::verification;

    diesel::insert_into(verification::table)
        .values(&verification_data)
        .get_result::<Verification>(connection)
}
