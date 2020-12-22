use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::models::Model;

use crate::page_models::{MutPage, Page};

use crate::errors_middleware::CustomHttpError;
use crate::errors_middleware::map_parsing_error;
use crate::errors_middleware::map_sql_error;

use crate::response_middleware::HttpResponseBuilder;


/// BEGIN ROOT CONTROLLERS

pub async fn _get_root() -> impl Responder {
    HttpResponse::Ok().body("unimplemented")
}

/// BEGIN PAGE CONTROLLERS

pub async fn create_page(req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let new_page: MutPage = serde_json::from_str(&req_body).expect("Did not correctly parse page.");

    Page::create(&new_page).map_err(map_sql_error)?;

    HttpResponseBuilder::new(201, &new_page)
}

pub async fn get_pages() -> Result<HttpResponse, CustomHttpError> {
    let pages: Vec<Page> = Page::read_all().map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &pages)
}

pub async fn get_page(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req
        .match_info()
        .get("id")
        .unwrap_or_default()
        .parse()
        .map_err(map_parsing_error)?;

    let page: Page = Page::read_one(page_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &page)
}

pub async fn update_page(
    req: HttpRequest,
    req_body: String,
) -> Result<HttpResponse, CustomHttpError> {
    let u_page: MutPage =
        serde_json::from_str(&req_body).expect("Did not correctly parse update parse page.");
    let page_id: i32 = req
        .match_info()
        .get("id")
        .unwrap_or_default()
        .parse()
        .map_err(map_parsing_error)?;

    Page::update(page_id, &u_page).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &u_page)
}

pub async fn delete_page(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req
        .match_info()
        .get("id")
        .unwrap_or_default()
        .parse()
        .map_err(map_parsing_error)?;

    Page::delete(page_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", page_id))
}
