use actix_web::{http, web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use uuid::Uuid;

use crate::models::user_models::{MutUser, User};
use crate::models::{pool_handler, Model, MySQLPool};
use crate::services::auth_service::{encrypt, encrypt_password, Claims};
use crate::services::errors_service::CustomHttpError;

pub async fn create_user(
    new: web::Json<MutUser>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let mut salted_user = new.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);
    salted_user.uuid = Some(Uuid::new_v4().to_string());

    User::create(&salted_user, &mysql_pool)?;

    Ok(HttpResponse::Created().json(&new.clone()))
}

pub async fn get_user(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let user: User = User::read_one(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(&user))
}

pub async fn update_user(
    id: web::Path<String>,
    new: web::Json<MutUser>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    // TODO maybe make this only happen whenever the password changes?
    let mut salted_user = new.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);

    // give them a new token just in case they update their username.
    let claim = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::days(10)).timestamp() as usize,
        sub: salted_user.username.clone(),
    };

    let token_enc = encrypt(claim)?;
    let new_user = HttpResponse::Ok().cookie(http::Cookie::new("auth", &token_enc)).json(&new.clone());
    salted_user.token = Some(token_enc);
    User::update(id.clone(), &new, &mysql_pool)?;

    Ok(new_user)
}

pub async fn delete_user(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = User::delete(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn login(
    user: web::Json<MutUser>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let arg = Argon2::default();

    let read_user = User::read_one(user.username.clone(), &mysql_pool)?;
    let read_user_password = PasswordHash::new(&read_user.password).unwrap();

    match arg.verify_password(
        user.password.clone().unwrap().as_bytes(),
        &read_user_password,
    ) {
        Ok(_) => {
            let mut new_user = user;

            let claim = Claims {
                exp: (chrono::Utc::now() + chrono::Duration::days(10)).timestamp() as usize,
                sub: new_user.username.clone(),
            };
            new_user.password = None;

            let token_enc = encrypt(claim)?;

            let cookie_response = HttpResponse::Ok()
                .cookie(http::Cookie::new("auth", &token_enc))
                .finish();

            new_user.token = Some(token_enc);

            User::update_with_token(&new_user, &mysql_pool)?;

            Ok(cookie_response)
        }
        _ => Ok(HttpResponse::Unauthorized().json("Failed to authenticate.")),
    }
}
