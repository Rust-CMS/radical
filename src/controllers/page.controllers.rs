use actix_web::{HttpRequest, HttpResponse, Responder};
use models::Model;
use page::{MutPage, Page};

#[path = "../models/page.models.rs"]
mod page;

#[path = "../models/models.rs"]
mod models;

// BEGIN ROOT CONTROLLERS

pub async fn get_root() -> impl Responder {
    HttpResponse::Ok().body("unimplemented")
}

// BEGIN PAGES CONTROLLERS

pub async fn get_pages() -> impl Responder {
    let pages: Vec<Page> = Page::read_all();
    HttpResponse::Ok().body(serde_json::to_string(&pages).unwrap())
}

/// BEGIN PAGE CONTROLLERS

pub async fn create_page(req_body: String) -> impl Responder {
    let new_page: MutPage = serde_json::from_str(&req_body).expect("Did not correctly parse page.");
    Page::create(&new_page);
    HttpResponse::Ok().body("Success")
}

pub async fn get_page(req: HttpRequest) -> impl Responder {
    let page_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let page: Page = Page::read_one(page_id);
    HttpResponse::Ok()
        .body(serde_json::to_string(&page).unwrap())
}



pub async fn update_page(req_body: String) -> impl Responder {
    let u_page: MutPage = serde_json::from_str(&req_body).expect("Did not correctly parse update parse page.");

    HttpResponse::Ok().body("Success")
}

pub async fn delete_page(req: HttpRequest) -> impl Responder {
    let page_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();

    Page::delete(page_id);
    HttpResponse::Ok().body("Success")
}