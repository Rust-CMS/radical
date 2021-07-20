use std::fs::File;
use std::io::BufReader;
use actix_web::{http::StatusCode, web};
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::controllers::config_controllers::LocalConfig;
use crate::models::MySQLPool;

use super::controllers::*;
use super::models::module_models::MutModule;
use super::models::page_models::MutPage;

pub fn establish_database_connection() -> Option<MySQLPool> {
    let config_file = File::open("./rcms.json").expect("Failed to open config file.");
    let reader = BufReader::new(config_file);
    let conf: LocalConfig = serde_json::from_reader(reader).expect("Failed to read config file.");
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        conf.mysql_username,
        conf.mysql_password,
        conf.mysql_url,
        conf.mysql_port,
        conf.mysql_database
    );

    Some(init_pool(&db_url).expect("Failed to create pool."))
}

// https://dev.to/werner/practical-rust-web-development-connection-pool-46f4
pub fn init_pool(db_url: &str) -> Result<MySQLPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder().max_size(1).build(manager)
}

// Creates a page used for unit tests.
async fn create_test_page() {
    let db = web::Data::new(establish_database_connection().unwrap());
    let new_page = MutPage {
        id: Some(-1),
        page_name: String::from("Hello world!"),
        page_url: String::from("/"),
        page_title: String::from("Hello world!"),
    guid: Some(Uuid::new_v4().to_string()),
    };
    page_controllers::create_page(web::Json(new_page), db)
        .await
        .unwrap();
}

// Creates a module used for unit tests.
async fn create_test_module() {
    let db = web::Data::new(establish_database_connection().unwrap());
    let new_module = MutModule {
        id: Some(-1),
        module_type_id: 1,
        content: String::from("Hello world!"),
        title: String::from("test"),
        page_id: -1,
        guid: Some(Uuid::new_v4().to_string()),
    };

    module_controllers::create_module(web::Json(new_module), db)
        .await
        .unwrap();
}

/// Deletes both pages and modules with the test IDs.
/// This function does nto have to be efficient, as it is only for tests.
async fn cleanup_test_values() {
    let db = web::Data::new(establish_database_connection().unwrap());

    page_controllers::delete_page(web::Path(-1), db.clone())
        .await
        .unwrap();
    module_controllers::delete_module(web::Path(-1), db.clone())
        .await
        .unwrap();
}

#[actix_rt::test]
async fn create_page() {
    let db = web::Data::new(establish_database_connection().unwrap());
    let new_page = MutPage {
        id: Some(-1),
        page_name: String::from("create_page_ut"),
        page_url: String::from("/create_page_ut"),
        page_title: String::from("create_page_ut"),
        guid: Some(Uuid::new_v4().to_string()),
    };
    let resp = page_controllers::create_page(web::Json(new_page), db)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::from_u16(201).unwrap());
}

#[actix_rt::test]
async fn read_all_pages() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;

    let resp = page_controllers::get_pages(db).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_page() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;

    let resp = page_controllers::get_page(web::Path(-1), db).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_page_join() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;
    create_test_module().await;

    let resp = page_controllers::get_page_join_modules(web::Path(-1), db)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_page() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;

    let new_page = MutPage {
        id: Some(-1),
        page_name: String::from("Hello world!"),
        page_url: String::from("/"),
        page_title: String::from("Hello world!"),
        guid: Some(Uuid::new_v4().to_string()),
    };

    let resp = page_controllers::update_page(web::Json(new_page), web::Path(-1), db)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn create_modules() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;

    let new_module = MutModule {
        id: Some(-1),
        module_type_id: 1,
        content: String::from("Hello world!"),
        title: String::from("title"),
        page_id: -1,
        guid: Some(Uuid::new_v4().to_string()),
    };
    let resp = module_controllers::create_module(web::Json(new_module), db)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::from_u16(201).unwrap());
}

#[actix_rt::test]
async fn read_all_modules() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;
    create_test_module().await;

    let resp = module_controllers::get_modules(db).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_module() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;
    create_test_module().await;

    let resp = module_controllers::get_module(web::Path(-1), db)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_modules() {
    let db = web::Data::new(establish_database_connection().unwrap());
    create_test_page().await;
    create_test_module().await;

    let new_module = MutModule {
        id: Some(-1),
        module_type_id: 1,
        content: String::from("Hello world!"),
        title: String::from("title"),
        page_id: -1,
        guid: Some(Uuid::new_v4().to_string()),
    };

    let resp = module_controllers::update_module(web::Json(new_module), web::Path(-1), db)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}
