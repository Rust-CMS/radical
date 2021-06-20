#![feature(int_error_matching)]

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;

use actix_files as fs;

mod schema;
mod routers;
mod models;
mod controllers;
mod middleware;

#[cfg(test)]
mod tests;

use routers::config_routers::{DatabaseConfigRouter, LocalConfigRouter};
use routers::page_routers::PageRouter;
use routers::module_routers::ModuleRouter;

#[macro_use]
extern crate diesel;

/// The main function is replaced by actix_web::main.
/// This allows main to be async and register the HttpServer.
/// All routes are defined here.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = models::establish_database_connection().unwrap();

    let mut handlebars = Handlebars::new();

    handlebars.register_templates_directory(".html", "./templates").unwrap();
    let handlebars_ref = web::Data::new(Mutex::new(handlebars));

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method();

        App::new()
            .wrap(cors)
            .service(
                web::scope("/v1")
                    .service(PageRouter::new())
                    .service(ModuleRouter::new())
                    .service(LocalConfigRouter::new())
                    .service(DatabaseConfigRouter::new()),
            )
            .service(fs::Files::new("/assets", "./templates/assets").show_files_listing())
            .default_service(web::get().to(controllers::page_controllers::display_page))
            .data(pool.clone())
            .app_data(handlebars_ref.clone())
    })
    .bind("127.0.0.1:9090")?
    .workers(15)
    .run()
    .await
}
