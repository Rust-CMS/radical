use actix_web::{web, Scope};

#[path = "../controllers/page.controllers.rs"]
mod controllers;

pub struct PageRouter;

impl PageRouter {
    pub fn new() -> Scope {
        web::scope("/v1")
            .route("/", web::get().to(controllers::get_root))
            .route("/pages", web::post().to(controllers::create_page))
            .route("/pages", web::get().to(controllers::get_pages))
            .route("/pages/{id}", web::get().to(controllers::get_page))
            .route("/pages/{id}", web::put().to(controllers::update_page))
            .route("/pages/{id}", web::delete().to(controllers::delete_page))
    }
}
