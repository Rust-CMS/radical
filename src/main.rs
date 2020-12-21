use actix_web::{App, HttpServer, middleware, web};

#[path = "./controllers/page.controllers.rs"]
mod controllers;


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
                .route("/", web::get().to(controllers::get_root))
                .route("/pages", web::get().to(controllers::get_pages))
                .route("/pages/{id}", web::get().to(controllers::get_page))
            )
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
