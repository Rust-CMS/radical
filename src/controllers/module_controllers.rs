use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::{Model, MySQLPool, pool_handler};
use crate::models::module_models::{Module, ModuleCategory, MutModule};

use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

pub async fn create_module(
    new: web::Json<MutModule>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    Module::create(&uuid_new, &mysql_pool)?;

    Ok(HttpResponse::Created().json(uuid_new))
}

pub async fn get_modules(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let modules = Module::read_all(&mysql_pool)?;

    Ok(HttpResponse::Created().json(modules))
}

pub async fn get_module(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let module = Module::read_one(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Created().json(module))
}

pub async fn update_module(
    updated_module: web::Json<MutModule>,
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Module::update(id.clone(), &updated_module, &mysql_pool)?;

    Ok(HttpResponse::Created().json(updated_module.0))
}

pub async fn delete_module(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = Module::delete(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Created().json(res))
}

pub async fn get_module_category(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let modules = ModuleCategory::join(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Created().json(modules))
}