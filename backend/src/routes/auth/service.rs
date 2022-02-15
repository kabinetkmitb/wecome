use super::dto::register::RegisterResponse;
use super::dto::token::TokenClaim;
use super::dto::{register::RegisterInput, token::Token};
use crate::routes::user::dto::create::CreateUser;
use crate::routes::verification::dto::create::CreateVerification;
use crate::routes::{user, verification};
use crate::UnwrappedPool;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use branca::Branca;
use std::env;

pub fn register(db: &UnwrappedPool, payload: RegisterInput) -> Result<RegisterResponse, Error> {
    let user = match user::service::create_user(
        db,
        CreateUser {
            nim: payload.nim,
            name: payload.name,
            password: payload.password,
            email: payload.email,
            is_admin: None,
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
            code: None,
            ..CreateVerification::default()
        },
    ) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    let token = match create_token(TokenClaim {
        user_id: user.id.clone(),
        is_admin: user.is_admin,
    }) {
        Err(err) => return Err(ErrorBadRequest(err)),
        Ok(data) => data,
    };

    Ok(RegisterResponse {
        token,
        verification_id: verification.id,
    })
}

pub fn create_token(claim: TokenClaim) -> Result<Token, branca::errors::Error> {
    let mut branca = Branca::new(&env::var("TOKEN_KEY").expect("token key not set").as_bytes())?;

    let token = branca.encode(serde_json::to_string(&claim).unwrap().as_bytes())?;

    Ok(Token { token })
}

pub fn get_token_data(token: Token) -> Result<TokenClaim, branca::errors::Error> {
    let branca = Branca::new(&env::var("TOKEN_KEY").expect("token key not set").as_bytes())?;

    let decoded_token = branca.decode(&token.token, 0)?;

    let decoded_string = &String::from_utf8(decoded_token).expect("invalid UTF-8");

    let token_claim: TokenClaim = serde_json::from_str(decoded_string).unwrap();

    Ok(token_claim)
}
