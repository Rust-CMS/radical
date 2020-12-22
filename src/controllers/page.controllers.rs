use actix_web::{HttpRequest, HttpResponse, Responder};
use models::Model;
use page::{MutPage, Page};

#[path = "../models/page.models.rs"]
mod page;

#[path = "../models/models.rs"]
mod models;

#[path = "../middleware/errors.middleware.rs"]
mod middleware;

use middleware::CustomHttpError;
use middleware::map_parsing_error;
use middleware::map_sql_error;

// BEGIN ROOT CONTROLLERS

pub async fn _get_root() -> impl Responder {
    HttpResponse::Ok().body("unimplemented")
}

// BEGIN PAGES CONTROLLERS

pub async fn get_pages() -> Result<HttpResponse, CustomHttpError> {
    let pages: Vec<Page> = Page::read_all().map_err(map_sql_error)?;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&pages).unwrap()))
}

/// BEGIN PAGE CONTROLLERS

pub async fn create_page(req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let new_page: MutPage = serde_json::from_str(&req_body).expect("Did not correctly parse page.");
    Page::create(&new_page).map_err(map_sql_error)?;

    Ok(HttpResponse::Ok().body("Success"))
}

pub async fn get_page(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;
    let page: Page = Page::read_one(page_id).map_err(map_sql_error)?;

    Ok(HttpResponse::Ok()
        .body(serde_json::to_string(&page).unwrap()))
}

pub async fn update_page(req: HttpRequest, req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let u_page: MutPage = serde_json::from_str(&req_body).expect("Did not correctly parse update parse page.");
    let page_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;
    Page::update(page_id, &u_page).map_err(map_sql_error)?;

    Ok(HttpResponse::Ok().body("Success"))
}

pub async fn delete_page(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req.match_info().get("id").unwrap_or_default().parse().map_err(map_parsing_error)?;
    Page::delete(page_id).map_err(map_sql_error)?;

    Ok(HttpResponse::Ok().body("Success"))
}