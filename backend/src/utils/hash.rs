use argon2::{self, Config};
use rand::Rng;
use std::env;

pub fn hash(password: String) -> String {
    let secret = env::var("SECRET_HASH").expect("secret not set");
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config {
        secret: &secret.as_bytes(),
        ..Config::default()
    };

    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}
