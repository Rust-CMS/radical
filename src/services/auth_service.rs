use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Serialize, Deserialize};

use crate::models::{Model, MySQLPool, pool_handler, user_models};

use actix_web::web;

pub fn encrypt(claim: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &claim, &EncodingKey::from_secret("secret".as_ref()))
}

pub fn decrypt(jwt: &String) -> Option<Claims> {
    let decoded_token = decode::<Claims>(jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).ok()?;

    Some(decoded_token.claims)
}

pub fn compare(token: Claims, enc_token: &String, pool: MySQLPool) -> bool {
    let mysql_pool = pool_handler(web::Data::new(pool)).unwrap();

    if let Ok(user) = user_models::User::read_one(token.username, &mysql_pool) {
        // verify against the encrypted version of the token.
        return user.token == Some(enc_token.clone());
    } else {
        return false
    }
}

pub fn root_user_insert() {

}

pub fn encrypt_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    return argon2.hash_password_simple(password.as_bytes(), &salt).unwrap().to_string();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub username: String
}