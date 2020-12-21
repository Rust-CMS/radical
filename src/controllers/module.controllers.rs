use actix_web::{HttpRequest, HttpResponse, Responder};
use models::Model;

#[path = "../models/models.rs"]
mod models;

#[path = "../models/module.models.rs"]
mod module;

use module::{MutModule, Module};

pub async fn create_module(req_body: String) -> impl Responder {
    let new_module: MutModule = serde_json::from_str(&req_body).unwrap();
    Module::create(&new_module);
    HttpResponse::Ok().body("Success")
}

pub async fn get_modules() -> impl Responder {
    let modules = Module::read_all();
    HttpResponse::Ok().body(serde_json::to_string(&modules).unwrap())
}

pub async fn get_module(req: HttpRequest) -> impl Responder {
    let module_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let module = Module::read_one(module_id);
    HttpResponse::Ok().body(serde_json::to_string(&module).unwrap())
}

pub async fn update_module(req_body: String, req: HttpRequest) -> impl Responder {
    let new_module: MutModule = serde_json::from_str(&req_body).unwrap();
    let module_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    Module::update(module_id, &new_module);
    HttpResponse::Ok().body("Success")
}

pub async fn delete_module(req: HttpRequest) -> impl Responder {
    let module_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    Module::delete(module_id);
    HttpResponse::Ok().body("Success")
}