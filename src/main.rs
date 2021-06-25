#![feature(try_blocks)]

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use controllers::config_controllers::LocalConfig;
use handlebars::Handlebars;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use log::{info};

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
    let config_file = File::open("./rcms.json").expect("Failed to open config file.");
    let reader = BufReader::new(config_file);
    let conf: LocalConfig = serde_json::from_reader(reader).expect("Failed to read config file.");

    let pool = models::establish_database_connection(conf.clone()).unwrap();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let handlebars = Handlebars::new();

    // web::Data is Arc, so we can safely clone it and send it between our watcher and the server.
    let handlebars_ref = web::Data::new(Mutex::new(handlebars));
    let hb = handlebars_ref.clone();

    hb.lock()
        .unwrap()
        .register_templates_directory(".html", "./templates")
        .unwrap();

    // Registers all default handlebars functions.
    helpers::default::register_helpers(handlebars_ref.clone());

    // Registers the fs watcher that updates the templates in memory every time a template is changed.
    // This is what enables hot reload.
    std::thread::spawn(|| watch::watch(hb));

    let server_url = &format!(
        "{}:{}",
        &conf.bind_address.unwrap(),
        &conf.bind_port.unwrap()
    );

    let http_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a -> %U | %Dms "))
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
    .bind(server_url)?
    .workers(2)
    .run();

    println!("🚀 Server is running 🚀");

    http_server.await
}
