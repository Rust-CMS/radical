use actix_web::{web, Scope};

#[path = "../controllers/module.controllers.rs"]
mod controllers;

pub struct ModuleRouter;

impl ModuleRouter {
    pub fn new() -> Scope {
        web::scope("/modules")
            .route("", web::post().to(controllers::create_module))
            .route("", web::get().to(controllers::get_modules))
            .route("/{id}", web::get().to(controllers::get_module))
            .route("/{id}", web::put().to(controllers::update_module))
            .route("/{id}", web::delete().to(controllers::delete_module))
    }
}
