use actix_web::{web, HttpResponse};

use crate::models::{Model, MySQLPool, pool_handler};
use crate::models::module_models::{Module, MutModule};

use crate::middleware::errors_middleware::CustomHttpError;
use crate::middleware::response_middleware::HttpResponseBuilder;

pub async fn create_module(
    new_module: web::Json<MutModule>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Module::create(&new_module, &mysql_pool)?;

    HttpResponseBuilder::new(201, &*new_module)
}

pub async fn get_modules(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let modules = Module::read_all(&mysql_pool)?;

    HttpResponseBuilder::new(200, &modules)
}

pub async fn get_module(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let module = Module::read_one(*id, &mysql_pool)?;

    HttpResponseBuilder::new(200, &module)
}

pub async fn update_module(
    updated_module: web::Json<MutModule>,
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Module::update(*id, &updated_module, &mysql_pool)?;

    HttpResponseBuilder::new(200, &*updated_module)
}

pub async fn delete_module(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Module::delete(*id, &mysql_pool)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", id))
}
