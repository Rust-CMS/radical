use actix_web::{middleware, App, HttpServer};

#[path = "./routers/page.routers.rs"]
mod routers;

use routers::PageRouter;

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
                PageRouter::new()
            )
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
