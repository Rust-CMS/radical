use actix_web::Scope;

pub mod module_routers;
pub mod page_routers;
pub mod category_routers;
pub mod user_routers;

pub trait Router {
    fn new() -> Scope;
}