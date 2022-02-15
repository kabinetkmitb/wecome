use super::dto::register::RegisterInput;
use crate::Pool;
use actix_web::{
    post,
    web::{self, Json},
    Error, HttpResponse,
};

#[post("/register")]
async fn register(
    db: web::Data<Pool>,
    payload: Json<RegisterInput>,
) -> Result<HttpResponse, Error> {
    super::service::register(&db.get().unwrap(), payload.into_inner())
        .map(|res| HttpResponse::Ok().body(res))
        .map_err(|err| err)
}
