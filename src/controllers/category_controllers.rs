use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::{
    middleware::{errors_middleware::CustomHttpError}, 
    models::{Model, MySQLPool, 
    module_models::{ModuleCategory, MutCategory},
    pool_handler}
};

pub async fn create_category(new: web::Json<MutCategory>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    ModuleCategory::create(&uuid_new, &mysql_pool)?;

    Ok(HttpResponse::Created().json(uuid_new))
}

pub async fn update_category(updated_category: web::Json<MutCategory>, id: web::Path<String>,  pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    ModuleCategory::update(id.clone(), &updated_category, &mysql_pool)?;

    Ok(HttpResponse::Ok().json(updated_category.0))
}

pub async fn get_category(id: web::Path<String>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::read_one(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete_category(id: web::Path<String>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::delete(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}