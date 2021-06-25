use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::{Model, MySQLPool, pool_handler};
use crate::models::module_models::{Module, MutModule};

use crate::middleware::errors_middleware::map_int_parsing_error;
use crate::middleware::errors_middleware::map_sql_error;
use crate::middleware::errors_middleware::CustomHttpError;

use crate::middleware::response_middleware::HttpResponseBuilder;

pub async fn create_module(
    req_body: String,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let new_module: MutModule =
        serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;

    Module::create(&new_module, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(201, &new_module)
}

pub async fn get_modules(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let modules = Module::read_all(&mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &modules)
}

pub async fn get_module(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let module_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    let module = Module::read_one(module_id, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &module)
}

pub async fn update_module(
    req_body: String,
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let new_module: MutModule =
        serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;
    let module_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    Module::update(module_id, &new_module, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &new_module)
}

pub async fn delete_module(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let module_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    Module::delete(module_id, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", module_id))
}
