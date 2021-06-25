#![feature(int_error_matching)]
#![feature(try_blocks)]

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;

use actix_files as fs;

mod controllers;
mod helpers;
mod middleware;
mod models;
mod routers;
mod schema;
mod watch;

#[cfg(test)]
mod tests;

use routers::config_routers::{DatabaseConfigRouter, LocalConfigRouter};
use routers::module_routers::ModuleRouter;
use routers::page_routers::PageRouter;

#[macro_use]
extern crate diesel;

/// The main function is replaced by actix_web::main.
/// This allows main to be async and register the HttpServer.
/// All routes are defined here.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = models::establish_database_connection().unwrap();

    let handlebars = Handlebars::new();

    // web::Data is Arc, so we can safely clone it and send it between our watcher and the server.
    let handlebars_ref = web::Data::new(Mutex::new(handlebars));
    let hb = handlebars_ref.clone();

    // Registers all default handlebars functions.
    helpers::default::register_helpers(handlebars_ref.clone());

    // Registers the fs watcher that updates the templates in memory every time a template is changed.
    // This is what enables hot reload.
    std::thread::spawn(|| watch::watch(hb));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

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
    .workers(2)
    .run()
    .await
}
