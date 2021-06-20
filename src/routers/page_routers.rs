use actix_web::{web, Scope};

use crate::controllers::page_controllers::*;

pub struct PageRouter;

impl PageRouter {
    pub fn new() -> Scope {
        web::scope("/pages")
            .route("", web::post().to(create_page))
            .route("", web::get().to(get_pages))
            .route("/{id}", web::get().to(get_page))
            .route("/{id}/modules", web::get().to(get_page_join_modules))
            .route("/{id}", web::put().to(update_page))
            .route("/{id}", web::delete().to(delete_page))
    }
}
