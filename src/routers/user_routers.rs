use actix_web::{web, Scope};

use crate::controllers::user_controllers::*;

pub struct UserRouter;

impl UserRouter {
    pub fn new() -> Scope {
        web::scope("/user")
            .route("/login", web::post().to(login))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
    }
}
