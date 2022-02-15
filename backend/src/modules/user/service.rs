use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{dto::create::CreateUser, model::User};

pub fn create_user<'a>(
    connection: &UnwrappedPool,
    user_data: CreateUser,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user;
    let mut user_data = user_data.clone();

    let hashed = crate::utils::hash::hash(user_data.password);
    user_data.password = hashed;

    diesel::insert_into(user::table)
        .values(&user_data)
        .get_result::<User>(connection)
}
