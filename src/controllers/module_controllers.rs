use actix_web::{HttpRequest, HttpResponse};

use crate::models::Model;

use crate::module_models::{MutModule, Module};

use crate::errors_middleware::CustomHttpError;
use crate::errors_middleware::map_parsing_error;
use crate::errors_middleware::map_sql_error;

use crate::response_middleware::HttpResponseBuilder;

pub async fn create_module(req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let new_module: MutModule = serde_json::from_str(&req_body).unwrap();

    Module::create(&new_module).map_err(map_sql_error)?;

    HttpResponseBuilder::new(201, &new_module)
}

pub async fn get_modules() -> Result<HttpResponse, CustomHttpError> {
    let modules = Module::read_all().map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &modules)
}

pub async fn get_module(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let module_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;

    let module = Module::read_one(module_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &module)
}

pub async fn update_module(req_body: String, req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let new_module: MutModule = serde_json::from_str(&req_body).unwrap();
    let module_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;

    Module::update(module_id, &new_module).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &new_module)
}

pub async fn delete_module(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let module_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;

    Module::delete(module_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", module_id))
}