use actix_web::{HttpRequest, HttpResponse};
use models::Model;

#[path = "../models/models.rs"]
mod models;

#[path = "../models/module.models.rs"]
mod module;

use module::{MutModule, Module};

#[path = "../middleware/errors.middleware.rs"]
mod middleware;

use middleware::CustomHttpError;
use middleware::map_parsing_error;
use middleware::map_sql_error;

pub async fn create_module(req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let new_module: MutModule = serde_json::from_str(&req_body).unwrap();
    Module::create(&new_module).map_err(map_sql_error)?;
    Ok(HttpResponse::Ok().body("Success"))
}

pub async fn get_modules() -> Result<HttpResponse, CustomHttpError> {
    let modules = Module::read_all().map_err(map_sql_error)?;
    Ok(HttpResponse::Ok().body(serde_json::to_string(&modules).unwrap()))
}

pub async fn get_module(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let module_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;
    let module = Module::read_one(module_id).map_err(map_sql_error)?;
    Ok(HttpResponse::Ok().body(serde_json::to_string(&module).unwrap()))
}

pub async fn update_module(req_body: String, req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let new_module: MutModule = serde_json::from_str(&req_body).unwrap();
    let module_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;
    Module::update(module_id, &new_module).map_err(map_sql_error)?;
    Ok(HttpResponse::Ok().body("Success"))
}

pub async fn delete_module(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let module_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;
    Module::delete(module_id).map_err(map_sql_error)?;
    Ok(HttpResponse::Ok().body("Success"))
}