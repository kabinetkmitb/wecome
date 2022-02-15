use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use std::env;

pub fn hash(password: String) -> String {
    let secret = env::var("SECRET_HASH").expect("secret not set");
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new_with_secret(
        &secret.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .unwrap();

    argon2
        .hash_password(&password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn check_password(password: String, hashed_password: String) -> Result<bool, String> {
    let secret = env::var("SECRET_HASH").expect("secret not set");
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new_with_secret(
        &secret.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .unwrap();

    let parsed_hash = match PasswordHash::new(&hashed_password) {
        Err(_) => return Err("Failed parsing".to_string()),
        Ok(data) => data,
    };

    Ok(argon2
        .verify_password(&password.as_bytes(), &parsed_hash)
        .is_ok())
}
