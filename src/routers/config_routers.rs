use actix_web::{web, Scope};

use crate::config_controllers::*;
pub struct LocalConfigRouter;

impl LocalConfigRouter {
    pub fn new() -> Scope {
        web::scope("/localConfig")
            .route("", web::get().to( read_all_local_config))
            .route("", web::put().to( update_local_config))
    }
}

pub struct DatabaseConfigRouter;

impl DatabaseConfigRouter {
    pub fn new() -> Scope {
        web::scope("/config")
            .route("", web::get().to(read_all_database_config))
            .route("/{id}", web::get().to(read_one_database_config))
            .route("/{id}", web::put().to(update_database_config))
    }
}
