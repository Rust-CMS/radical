use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize, Deserialize};

use crate::models::{Model, MySQLPool, pool_handler, user_models};

use actix_web::web;

pub fn encrypt(claim: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &claim, &EncodingKey::from_secret("secret".as_ref()))
}

pub fn decrypt(jwt: String) -> Option<Claims> {
    let decoded_token = decode::<Claims>(&jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).ok()?;

    Some(decoded_token.claims)
}

pub fn compare(token: Claims, pool: MySQLPool) -> bool {
    let mysql_pool = pool_handler(web::Data::new(pool));

    if let Ok(mysql_pool) = mysql_pool {
        user_models::User::read_one(token.username, &mysql_pool).is_ok()
    } else {
        return false
    }
}

pub fn root_user_insert() {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    username: String
}