use diesel::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

use crate::UnwrappedPool;

use super::{dto::create::CreateVerification, model::Verification};

pub fn create_verification<'a>(
    connection: &UnwrappedPool,
    verification_data: CreateVerification,
) -> Result<Verification, diesel::result::Error> {
    use crate::schema::verification;
    use crate::schema::verification::dsl::*;

    let mut verification_data = verification_data;

    let verification_code = match verification_data.code {
        Some(c) => Some(c),
        None => {
            let random_code: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect();
            Some(random_code.to_string())
        }
    };

    verification_data.code = verification_code;

    let insert_respond = diesel::insert_into(verification::table)
        .values(&verification_data)
        .get_result::<Verification>(connection);

    match insert_respond {
        Err(e) => return Err(e),
        _ => {}
    }

    let final_verification = verification
        .filter(user_id.eq(verification_data.user_id))
        .limit(1)
        .load::<Verification>(connection)?;

    Ok(final_verification[0].clone())
}
