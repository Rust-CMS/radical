use actix_web::{HttpResponse, web};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;

use crate::models::{Model, MySQLPool, pool_handler};
use crate::services::errors_service::CustomHttpError;
use crate::models::user_models::{MutUser, User};

pub async fn create_user(new: web::Json<MutUser>, pool: web::Data<MySQLPool>,) -> Result<HttpResponse, CustomHttpError>{
    let mysql_pool = pool_handler(pool)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let mut salted_user = new.clone();
    salted_user.password = argon2.hash_password_simple(new.password.as_bytes(), &salt).unwrap().to_string();

    let res = User::create(&salted_user, &mysql_pool)?;

    Ok(HttpResponse::Created().json(&new.clone()))
}

pub async fn get_user(id: web::Path<String>, pool: web::Data<MySQLPool>,) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let user: User = User::read_one(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(&user))
}

pub async fn update_user(id: web::Path<String>, new: web::Json<MutUser>, pool: web::Data<MySQLPool>,) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = User::update(id.clone(), &new, &mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete_user(id: web::Path<String>, pool: web::Data<MySQLPool>,) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = User::delete(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}