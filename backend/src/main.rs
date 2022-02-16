#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate branca;
extern crate dotenv;
extern crate env_logger;
extern crate getrandom;

use ::r2d2::PooledConnection;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::Config;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenv::dotenv;
use std::env;

pub mod modules;
pub mod schema;
pub mod utils;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type UnwrappedPool = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let database_url = env::var("DATABASE_URL").expect("Database url must be set in .env");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let result = HttpServer::new(move || {
        App::new()
            .app_data(Config::default().realm("Restricted area").scope("auth"))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(pool.clone())
            .service(
                web::scope("/kompetisi").service(modules::kompetisi::controller::create_kompetisi),
            )
            .service(
                web::scope("/auth")
                    .service(modules::auth::controller::register)
                    .service(modules::auth::controller::verify_email)
                    .service(modules::auth::controller::login)
                    .service(modules::auth::controller::me),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await;

    println!("Running blazingly fast at {}:{} 🚀", host, port);
    result
}
