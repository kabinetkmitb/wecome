use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Found()
        .header("Location", "https://wecome-dev.netlify.app/login")
        .finish()
}
