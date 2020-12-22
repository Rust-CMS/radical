#![feature(int_error_matching)]

use actix_web::{middleware, web, App, HttpServer};

/// All top level module declarations should go in main.rs.
/// This allows you to then `use crate::module_controllers` in other files.
#[path = "./controllers/module_controllers.rs"]
mod module_controllers;
#[path = "./controllers/page_controllers.rs"]
mod page_controllers;
#[path = "./middleware/errors_middleware.rs"]
mod errors_middleware;
#[path = "./middleware/response_middleware.rs"]
mod response_middleware;
#[path = "./models/models.rs"]
mod models;
#[path = "./models/module_models.rs"]
mod module_models;
#[path = "./models/page_models.rs"]
mod page_models;
#[path = "./routers/module_routers.rs"]
mod module_routers;
#[path = "./routers/page_routers.rs"]
mod page_routers;

use page_routers::PageRouter;
use module_routers::ModuleRouter;

#[macro_use]
extern crate diesel;

/// The main function is replaced by actix_web::main.
/// This allows main to be async and register the HttpServer.
/// All routes are defined here.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Version", "0.1")
                    .header("Content-Type", "application/json")
                    .header("Access-Control-Allow-Origin", "*"),
            )
            .service(
                web::scope("/v1")
                    .service(PageRouter::new())
                    .service(ModuleRouter::new()),
            )
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
