use super::dto::{register::RegisterInput, token::Token};
use crate::UnwrappedPool;
use actix_web::Error;

pub fn register(db: &UnwrappedPool, payload: RegisterInput) -> Result<Token, Error> {}
