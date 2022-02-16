use super::dto::register::RegisterInput;
use crate::{modules::auth::dto::login::LoginInput, Pool};
use actix_web::{
    get, post,
    web::{self, Json},
    Error, HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::env;

#[post("/register")]
async fn register(
    db: web::Data<Pool>,
    payload: Json<RegisterInput>,
) -> Result<HttpResponse, Error> {
    super::service::register(&db.get().unwrap(), payload.into_inner())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[post("/login")]
async fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Result<HttpResponse, Error> {
    super::service::login(&db.get().unwrap(), payload.into_inner())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[get("/me")]
async fn me(auth: BearerAuth) -> Result<HttpResponse, Error> {
    super::service::me(auth.token().to_string())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[get("/verify-email/{verification_id}")]
async fn verify_email(
    db: web::Data<Pool>,
    web::Path(verification_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let app_url = env::var("FRONTEND_URL").expect("app url not set");
    super::service::verify_mail(&db.get().unwrap(), verification_id)
        .map(|_| {
            HttpResponse::Found()
                .header("Location", format!("{}login", app_url))
                .finish()
        })
        .map_err(|err| err)
}
