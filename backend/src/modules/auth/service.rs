use super::dto::login::{LoginInput, LoginResponse};
use super::dto::register::{RegisterInput, RegisterResponse};
use super::dto::token::TokenClaim;
use crate::modules::user::dto::create::CreateUser;
use crate::modules::user::dto::update::UpdateUser;
use crate::modules::user::service::update_user_by_id;
use crate::modules::verification::dto::create::CreateVerification;
use crate::modules::verification::service::get_verification_by_id;
use crate::modules::{user, verification};
use crate::utils::email::send_verification_email;
use crate::utils::hash::check_password;
use crate::UnwrappedPool;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use branca::Branca;
use std::env;

pub fn verify_mail(db: &UnwrappedPool, verification_id: String) -> Result<(), Error> {
    let verification = match get_verification_by_id(db, verification_id) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(result) => result,
    };

    match update_user_by_id(
        db,
        verification.user_id,
        UpdateUser {
            is_active: Some(true),
            ..UpdateUser::default()
        },
    ) {
        Err(e) => Err(ErrorBadRequest(e)),
        Ok(_) => Ok(()),
    }
}

pub fn register(db: &UnwrappedPool, payload: RegisterInput) -> Result<RegisterResponse, Error> {
    let user = match user::service::create_user(
        db,
        CreateUser {
            name: payload.name,
            password: payload.password,
            email: payload.email,
            ..CreateUser::default()
        },
    ) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    let verification = match verification::service::create_verification(
        db,
        CreateVerification {
            user_id: user.id.clone(),
            ..CreateVerification::default()
        },
    ) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    match send_verification_email(user.name, user.email, verification.clone().id) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(_) => {}
    };

    Ok(RegisterResponse {
        message: String::from("Successfully registered!"),
    })
}

pub fn login(db: &UnwrappedPool, payload: LoginInput) -> Result<LoginResponse, Error> {
    let user = match user::service::get_user_by_email(db, payload.email) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    if !user.is_active {
        return Err(ErrorBadRequest("User has not activated account!"));
    };

    let valid = match check_password(payload.password, user.clone().password) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(data) => data,
    };

    if !valid {
        return Err(ErrorBadRequest("Wrong password!"));
    };

    let token = match create_token(TokenClaim {
        user_id: user.clone().id,
        is_admin: user.clone().is_admin,
        name: user.clone().name,
    }) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(data) => data,
    };

    Ok(LoginResponse {
        name: user.clone().name,
        is_admin: user.clone().is_admin,
        id: user.clone().id,
        token,
    })
}

pub fn me(token: String) -> Result<TokenClaim, Error> {
    get_token_data(token)
        .map(|claim| claim)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn create_token(claim: TokenClaim) -> Result<String, branca::errors::Error> {
    let mut branca = Branca::new(&env::var("TOKEN_KEY").expect("token key not set").as_bytes())?;

    branca.encode(serde_json::to_string(&claim).unwrap().as_bytes())
}

pub fn get_token_data(token: String) -> Result<TokenClaim, branca::errors::Error> {
    let branca = Branca::new(&env::var("TOKEN_KEY").expect("token key not set").as_bytes())?;

    let decoded_token = branca.decode(&token, 0)?;

    let decoded_string = &String::from_utf8(decoded_token).expect("invalid UTF-8");

    let token_claim: TokenClaim = serde_json::from_str(decoded_string).unwrap();

    Ok(token_claim)
}
