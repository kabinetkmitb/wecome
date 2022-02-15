use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{
    dto::{create::CreateUser, update::UpdateUser},
    model::User,
};

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

pub fn update_user_by_id<'a>(
    connection: &UnwrappedPool,
    user_id: String,
    user_data: UpdateUser,
) -> Result<User, diesel::result::Error> {
    use crate::schema::user::dsl::*;
    let mut user_data = user_data.clone();

    if user_data.password.is_some() {
        let hashed = crate::utils::hash::hash(user_data.password.unwrap());
        user_data.password = Some(hashed);
    }

    diesel::update(user.find(user_id))
        .set(&user_data)
        .get_result::<User>(connection)
}
