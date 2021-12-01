use actix_web::cookie::Cookie;
use actix_web::{web, HttpRequest, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::models::user_models::{MutUser, User};
use crate::models::{pool_handler, Model, MySQLPool};
use crate::services::auth_service::{authenticate, encrypt, encrypt_password, Claims};
use crate::services::errors_service::CustomHttpError;

pub async fn create_user(
    new: web::Json<MutUser>,
    pool: web::Data<MySQLPool>,
    _: Claims,
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
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let user: User = User::read_one(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(&user))
}

pub async fn update_user(
    id: web::Path<String>,
    new: web::Json<MutUser>,
    pool: web::Data<MySQLPool>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    // TODO maybe make this only happen whenever the password changes?
    let mut salted_user = new.clone();

    // if you're trying to change someone elses data, don't allow it.
    if id.clone() != claim.sub {
        // TODO make this an err, not Ok.
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);

    let exp_time = chrono::Utc::now() + chrono::Duration::days(10);

    // give them a new token just in case they update their username.
    let claim = Claims {
        exp: (exp_time).timestamp() as usize,
        sub: salted_user.username.clone(),
    };

    let time: OffsetDateTime = OffsetDateTime::now_utc() + Duration::hour();

    let token_enc = encrypt(claim)?;
    let cookie = Cookie::build("auth", &token_enc)
        .expires(time)
        .path("/")
        .finish();

    let user = HttpResponse::Ok().cookie(cookie).json(&new.clone());
    salted_user.token = Some(token_enc);
    User::update(id.clone(), &salted_user, &mysql_pool)?;

    Ok(user)
}

pub async fn delete_user(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    _: Claims,
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

    let is_default = read_user.username == "root" && read_user.password == "";

    // if you're trying to login to a root user more than once with no password set, send back a forbidden.
    if read_user.token.is_some() && is_default {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // default password handler.
    if is_default {
        let mut new_user = user.clone();
        let cookie = login_res(&mut new_user)?;

        let cookie_response = HttpResponse::Accepted().cookie(cookie.clone()).finish();

        new_user.token = Some(cookie.value().to_string());

        User::update_with_token(&new_user, &mysql_pool)?;

        return Ok(cookie_response);
    }
    let read_user_password = PasswordHash::new(&read_user.password).unwrap();

    match arg.verify_password(
        user.password.clone().unwrap().as_bytes(),
        &read_user_password,
    ) {
        Ok(_) => {
            let mut new_user = user;
            let cookie = login_res(&mut new_user)?;

            let cookie_response = HttpResponse::Ok().cookie(cookie.clone()).finish();

            new_user.token = Some(cookie.value().to_string());

            User::update_with_token(&new_user, &mysql_pool)?;

            Ok(cookie_response)
        }
        _ => Ok(HttpResponse::Unauthorized().json("Failed to authenticate.")),
    }
}

fn login_res(user: &mut MutUser) -> Result<Cookie, CustomHttpError> {
    let claim = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::days(10)).timestamp() as usize,
        sub: user.username.clone(),
    };
    user.password = None;
    let token_enc = encrypt(claim)?;

    let time: OffsetDateTime = OffsetDateTime::now_utc() + Duration::hour();
    let cookie = Cookie::build("auth", token_enc)
        .expires(time)
        .path("/")
        .finish();

    Ok(cookie)
}

pub async fn logout() -> Result<HttpResponse, CustomHttpError> {
    let cookie = Cookie::build("auth", "")
        .expires(OffsetDateTime::now_utc())
        .path("/")
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}

pub async fn check_login(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let auth_header = req.headers().get("authorization");

    let auth_res = authenticate(auth_header.unwrap(), &mysql_pool).await;

    match auth_res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}
