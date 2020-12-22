#![feature(int_error_matching)]

use actix_web::{middleware, web, App, HttpServer};

#[path = "./routers/page.routers.rs"]
mod page_routers;

#[path = "./routers/module.routers.rs"]
mod module_routers;

use module_routers::ModuleRouter;
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
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
