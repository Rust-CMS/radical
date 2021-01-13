#![feature(int_error_matching)]

use actix_web::{middleware, web, App, HttpServer};

use actix_files as fs;

#[path = "./controllers/config_controllers.rs"]
mod config_controllers;
#[path = "./models/config_models.rs"]
mod config_models;
#[path = "./routers/config_routers.rs"]
mod config_routers;
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
#[path = "./schemas/schema.rs"]
mod schema;

#[cfg(test)]
mod tests;

use config_routers::{DatabaseConfigRouter, LocalConfigRouter};
use module_routers::ModuleRouter;
use page_routers::PageRouter;

#[macro_use]
extern crate diesel;

/// The main function is replaced by actix_web::main.
/// This allows main to be async and register the HttpServer.
/// All routes are defined here.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = models::establish_database_connection().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Methods", "POST, PUT, GET, OPTIONS")
                    .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
                    .header("X-Version", "0.1")
                    .header("Content-Type", "application/json")
                    .header("Access-Control-Allow-Origin", "*"),
            )
            .service(
                web::scope("/v1")
                    .service(PageRouter::new())
                    .service(ModuleRouter::new())
                    .service(LocalConfigRouter::new())
                    .service(DatabaseConfigRouter::new()),
            )
            .service(fs::Files::new("/assets", "./public/assets").show_files_listing())
            .service(fs::Files::new("/sites", "./public/sites").show_files_listing())
            .default_service(web::get().to(page_controllers::display_page))
            .data(pool.clone())
    })
    .bind("127.0.0.1:9090")?
    .workers(15)
    .run()
    .await
}
