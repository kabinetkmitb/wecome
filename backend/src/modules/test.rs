use actix_web::{error::Error, error::ErrorBadRequest, get, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .header("Location", "https://wecome-dev.netlify.app/login")
        .finish()
}

#[get("/email")]
async fn testing_email() -> Result<HttpResponse, Error> {
    crate::utils::email::send_verification_email(
        "Unknown User".to_string(),
        "bimbalabum@gmail.com".to_string(),
        "test".to_string(),
    )
    .map(|res| HttpResponse::Ok().body(res))
    .map_err(|err| ErrorBadRequest(err))
}
