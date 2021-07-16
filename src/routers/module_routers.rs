use actix_web::{web, Scope};

use crate::controllers::module_controllers::*;

pub struct ModuleRouter;

impl ModuleRouter {
    pub fn new() -> Scope {
        web::scope("/modules")
            .route("", web::post().to(create_module))
            .route("", web::get().to(get_modules))
            .route("/{id}", web::get().to(get_module))
            .route("/{id}", web::put().to(update_module))
            .route("/{id}", web::delete().to(delete_module))
            .route("/category/{id}", web::get().to(get_module_category))
    }
}
