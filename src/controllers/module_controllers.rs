use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::{Model, MySQLPool, pool_handler};
use crate::models::module_models::{Module, ModuleCategory, MutModule};

use crate::middleware::errors_middleware::CustomHttpError;
use crate::middleware::response_middleware::HttpResponseBuilder;

pub async fn create_module(
    new: web::Json<MutModule>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    Module::create(&uuid_new, &mysql_pool)?;

    HttpResponseBuilder::new(201, &uuid_new)
}

pub async fn get_modules(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let modules = Module::read_all(&mysql_pool)?;

    HttpResponseBuilder::new(200, &modules)
}

pub async fn get_module(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let module = Module::read_one(id.clone(), &mysql_pool)?;

    HttpResponseBuilder::new(200, &module)
}

pub async fn update_module(
    updated_module: web::Json<MutModule>,
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Module::update(id.clone(), &updated_module, &mysql_pool)?;

    HttpResponseBuilder::new(200, &*updated_module)
}

pub async fn delete_module(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Module::delete(id.clone(), &mysql_pool)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", id))
}

pub async fn get_module_category(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let modules = ModuleCategory::join(id.clone(), &mysql_pool)?;

    HttpResponseBuilder::new(200, &modules)
}