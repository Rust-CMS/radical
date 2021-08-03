use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::MysqlConnection;
use futures::{future::LocalBoxFuture, Future};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::models::{pool_handler, user_models, Model, MySQLPool};

use actix_web::{dev::Payload, http::HeaderValue, web, FromRequest, HttpRequest};

use super::errors_service::CustomHttpError;

pub fn encrypt(claim: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("B669681336E3D84E5BE598A92C524".as_ref()),
    )
}

pub fn decrypt(jwt: &String) -> Option<Claims> {
    let decoded_token = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret("B669681336E3D84E5BE598A92C524".as_ref()),
        &Validation::default(),
    )
    .ok()?;

    Some(decoded_token.claims)
}

pub fn compare(token: &Claims, enc_token: &String, pool: &MysqlConnection) -> bool {
    if let Ok(user) = user_models::User::read_one(token.sub.clone(), &pool) {
        // verify against the encrypted version of the token.
        return user.token == Some(enc_token.clone());
    } else {
        return false;
    }
}

pub fn encrypt_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    return argon2
        .hash_password_simple(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
}

impl FromRequest for Claims {
    type Error = CustomHttpError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<MySQLPool>>().unwrap().to_owned();
        let mysql_pool = pool_handler(pool).unwrap();
        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(auth) => {
                let fut = authenticate(auth, &mysql_pool);
                Box::pin(fut)
            }
            _ => Box::pin(async { Err(CustomHttpError::Unauthorized) }),
        }
    }
}

fn authenticate(
    auth_header: &HeaderValue,
    db: &MysqlConnection,
) -> impl Future<Output = Result<Claims, CustomHttpError>> {
    let encrypted_token = std::str::from_utf8(auth_header.as_bytes())
        .unwrap()
        .to_string();

    let decrypted_token = decrypt(&encrypted_token).unwrap();

    let is_logged_in = compare(&decrypted_token, &encrypted_token, db);

    async move {
        if is_logged_in {
            Ok(decrypted_token)
        } else {
            Err(CustomHttpError::Unauthorized)
        }
    }
}
