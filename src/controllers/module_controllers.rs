use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::{Model, MySQLPool, pool_handler};

use crate::module_models::{Module, MutModule};

use crate::errors_middleware::map_int_parsing_error;
use crate::errors_middleware::map_sql_error;
use crate::errors_middleware::CustomHttpError;

use crate::response_middleware::HttpResponseBuilder;

/// Creates a module by passing a module-like JSON object.
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

/// Gets all modules.
pub async fn get_modules(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let modules = Module::read_all(&mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &modules)
}

/// Gets one module by passing a module ID.
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

/// Updates a module by passing a module-like JSON object and a module ID.
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

/// Deletes a module by passing a module ID.
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
