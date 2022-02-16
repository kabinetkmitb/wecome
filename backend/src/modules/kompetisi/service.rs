use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{dto::create::CreateKompetisi, model::Kompetisi};

pub fn create_kompetisi<'a>(
    connection: &UnwrappedPool,
    kompetisi_data: CreateKompetisi,
) -> Result<Kompetisi, diesel::result::Error> {
    use crate::schema::kompetisi;

    diesel::insert_into(kompetisi::table)
        .values(&kompetisi_data)
        .get_result::<Kompetisi>(connection)
}

pub fn find_many_kompetisi<'a>(
    connection: &UnwrappedPool,
    kompetisi_data: CreateKompetisi,
) -> Result<Kompetisi, diesel::result::Error> {
    use crate::schema::kompetisi;

    diesel::insert_into(kompetisi::table)
        .values(&kompetisi_data)
        .get_result::<Kompetisi>(connection)
}
