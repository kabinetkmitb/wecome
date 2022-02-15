use super::dto::register::RegisterInput;
use crate::{modules::auth::dto::login::LoginInput, Pool};
use actix_web::{
    get, post,
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

#[post("/login")]
async fn login(db: web::Data<Pool>, payload: Json<LoginInput>) -> Result<HttpResponse, Error> {
    super::service::login(&db.get().unwrap(), payload.into_inner())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| err)
}

#[get("/verify-email/{verification_id}")]
async fn verify_email(
    db: web::Data<Pool>,
    web::Path(verification_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    // TODO: ubah redirect url
    super::service::verify_mail(&db.get().unwrap(), verification_id)
        .map(|_| {
            HttpResponse::Found()
                .header("Location", "https://wecome.com")
                .finish()
        })
        .map_err(|err| err)
}
