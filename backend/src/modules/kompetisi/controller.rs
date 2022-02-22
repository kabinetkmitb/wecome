use super::dto::create::CreateKompetisi;
use crate::{
    modules::kompetisi::dto::find_many::FindManyKompetisiQuery,
    modules::kompetisi::dto::propose_kompetisi::{ProposeKompetisiInput, ProposeKompetisiResponse},
    Pool,
};
use actix_web::{
    error::ErrorBadRequest,
    get, post, put,
    web::{self, Json},
    Error, HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("")]
async fn create_kompetisi(
    db: web::Data<Pool>,
    payload: Json<CreateKompetisi>,
) -> Result<HttpResponse, Error> {
    super::service::create_kompetisi(&db.get().unwrap(), payload.into_inner())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| ErrorBadRequest(err))
}

#[post("/propose-kompetisi")]
pub async fn propose_kompetisi(
    auth: BearerAuth,
    db: web::Data<Pool>,
    payload: Json<ProposeKompetisiInput>,
) -> Result<HttpResponse, Error> {
    super::service::propose_kompetisi(
        &db.get().unwrap(),
        auth.token().to_string(),
        payload.into_inner(),
    )
    .map(|res| HttpResponse::Ok().json(ProposeKompetisiResponse { message: res }))
    .map_err(|err| ErrorBadRequest(err))
}

#[get("")]
pub async fn find_many_kompetisi(
    db: web::Data<Pool>,
    query: web::Query<FindManyKompetisiQuery>,
) -> Result<HttpResponse, Error> {
    super::service::find_many_kompetisi(&db.get().unwrap(), query.into_inner())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| ErrorBadRequest(err))
}

#[put("/accept-kompetisi/{kompetisi_id}")]
pub async fn accept_kompetisi(
    auth: BearerAuth,
    db: web::Data<Pool>,
    web::Path(kompetisi_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    super::service::accept_kompetisi(&db.get().unwrap(), auth.token().to_string(), kompetisi_id)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| ErrorBadRequest(err))
}

#[put("/decline-kompetisi/{kompetisi_id}")]
pub async fn decline_kompetisi(
    auth: BearerAuth,
    db: web::Data<Pool>,
    web::Path(kompetisi_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    super::service::decline_kompetisi(&db.get().unwrap(), auth.token().to_string(), kompetisi_id)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| ErrorBadRequest(err))
}
