use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{dto::create::CreateVerification, model::Verification};

pub fn create_user<'a>(
    connection: &UnwrappedPool,
    verification_data: CreateVerification,
) -> Result<Verification, diesel::result::Error> {
    use crate::schema::verification;
    use crate::schema::verification::dsl::*;

    let insert_respond = diesel::insert_into(verification::table)
        .values(&verification_data)
        .get_result::<Verification>(connection);

    match insert_respond {
        Err(e) => return Err(e),
        _ => {}
    }

    let final_user = verification
        .filter(user_id.eq(verification_data.user_id))
        .limit(1)
        .load::<Verification>(connection)?;

    Ok(final_user[0])
}
