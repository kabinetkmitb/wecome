use super::dto::create::CreateKompetisi;
use crate::Pool;
use actix_web::{
    error::ErrorBadRequest,
    post,
    web::{self, Json},
    Error, HttpResponse,
};

#[post("")]
async fn create_kompetisi(
    db: web::Data<Pool>,
    payload: Json<CreateKompetisi>,
) -> Result<HttpResponse, Error> {
    super::service::create_kompetisi(&db.get().unwrap(), payload.into_inner())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| ErrorBadRequest(err))
}
