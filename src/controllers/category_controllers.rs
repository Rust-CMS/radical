use actix_web::{HttpResponse, web};

use crate::{middleware::{errors_middleware::CustomHttpError, response_middleware::HttpResponseBuilder}, models::{Model, MySQLPool, module_models::{ModuleCategory, MutCategory}, pool_handler}};

pub async fn create_category(new: web::Json<MutCategory>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    ModuleCategory::create(&new, &mysql_pool)?;

    HttpResponseBuilder::new(201, &"Successfully created.".to_owned())
}

pub async fn update_category(id: web::Path<i32>, new: web::Json<MutCategory>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::update(*id, &new, &mysql_pool)?;

    HttpResponseBuilder::new(200, &res)
}

pub async fn get_category(id: web::Path<i32>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::read_one(*id, &mysql_pool)?;

    HttpResponseBuilder::new(200, &res)
}

pub async fn delete_category(id: web::Path<i32>, pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::delete(*id, &mysql_pool)?;

    HttpResponseBuilder::new(200, &res)
}