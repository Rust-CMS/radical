#![feature(int_error_matching)]

use actix_web::{middleware, web, App, HttpServer};

use actix_files as fs;

/// All top level module declarations should go in main.rs.
/// This allows you to then `use crate::module_controllers` in other files.
#[path = "./middleware/errors_middleware.rs"]
mod errors_middleware;
#[path = "./models/models.rs"]
mod models;
#[path = "./controllers/module_controllers.rs"]
mod module_controllers;
#[path = "./models/module_models.rs"]
mod module_models;
#[path = "./routers/module_routers.rs"]
mod module_routers;
#[path = "./controllers/page_controllers.rs"]
mod page_controllers;
#[path = "./models/page_models.rs"]
mod page_models;
#[path = "./routers/page_routers.rs"]
mod page_routers;
#[path = "./middleware/response_middleware.rs"]
mod response_middleware;
#[cfg(test)]
mod tests;

use module_routers::ModuleRouter;
use page_controllers::index;
use page_routers::PageRouter;

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
            .service(fs::Files::new("/assets", "./public/assets").show_files_listing())
            .default_service(web::route().to(index))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
