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
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let mut salted_user = new.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap());
    salted_user.password = Some(encrypted_password);
    salted_user.uuid = Some(Uuid::new_v4().to_string());

    User::create(&salted_user, &mysql_pool)?;

    Ok(HttpResponse::Created().json(&new.clone()))
}

pub async fn get_user(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let user: User = User::read_one(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(&user))
}

pub async fn update_user(
    id: web::Path<String>,
    new: web::Json<MutUser>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    // TODO maybe make this only happen whenever the password changes?
    let mut salted_user = new.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap());
    salted_user.password = Some(encrypted_password);

    // if user updates username, give them a new token.
    User::update(id.clone(), &new, &mysql_pool)?;

    Ok(HttpResponse::Ok().json(&new.clone()))
}

pub async fn delete_user(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
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

    if let Ok(_) = arg.verify_password(user.password.clone().unwrap().as_bytes(), &PasswordHash::new(&read_user.password).unwrap()) {
        let mut new_user = user.clone();

        let claim = Claims {
            exp: 100000,
            username: new_user.username.clone(),
        };
        new_user.password = None;
    
        let token_enc = encrypt(claim)?;
        new_user.token = Some(token_enc.clone());
    
        User::update_with_token(&new_user, &mysql_pool)?;

        Ok(HttpResponse::Ok()
            .cookie(http::Cookie::new("auth", &token_enc))
            .finish())
    } else {
        Ok(HttpResponse::Unauthorized().json("Failed to authenticate."))
    }
}
