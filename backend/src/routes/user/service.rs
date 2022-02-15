use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{dto::create::CreateUser, model::User};

pub fn create_user<'a>(
    connection: &UnwrappedPool,
    user_data: CreateUser,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user;
    use crate::schema::user::dsl::*;
    let mut user_data = user_data;

    let hashed = crate::utils::hash::hash(&user_data.password);
    user_data.password = hashed;

    let insert_respond = diesel::insert_into(user::table)
        .values(&user_data)
        .get_result::<User>(connection);

    match insert_respond {
        Err(e) => return Err(e),
        _ => {}
    }

    let final_user = user
        .filter(nim.eq_all(user_data.nim))
        .limit(1)
        .load::<User>(connection)?;

    // TODO : craete verification

    Ok(final_user[0])
}
