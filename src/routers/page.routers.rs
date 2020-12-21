use actix_web::{web, Scope};

#[path = "../controllers/page.controllers.rs"]
mod controllers;

pub struct PageRouter;

impl PageRouter {
    pub fn new() -> Scope {
        web::scope("/pages")
            .route("", web::post().to(controllers::create_page))
            .route("", web::get().to(controllers::get_pages))
            .route("/{id}", web::get().to(controllers::get_page))
            .route("/{id}", web::put().to(controllers::update_page))
            .route("/{id}", web::delete().to(controllers::delete_page))
    }
}
