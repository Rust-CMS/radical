use super::controllers::*;
use actix_web::{http::StatusCode, test};

use super::models::module_models::MutModule;
use super::models::page_models::MutPage;

// Creates a page used for unit tests.
async fn create_test_page() {
    let new_page = MutPage {
        page_name: String::from("Hello world!"),
        page_url: String::from("/"),
        page_title: String::from("Hello world!"),
    };
    page_controllers::create_page(serde_json::to_string(&new_page).unwrap())
        .await
        .unwrap();
}

// Creates a module used for unit tests.
async fn create_test_module() {
    let new_module = MutModule {
        module_id: Some(-1),
        module_type_id: 1,
        content: Some(String::from("Hello world!")),
        title: String::from("test"),
        page_name: String::from("test"),
    };

    module_controllers::create_module(serde_json::to_string(&new_module).unwrap())
        .await
        .unwrap();
}

/// Deletes both pages and modules with the test IDs.
/// This function does nto have to be efficient, as it is only for tests.
async fn cleanup_test_values() {
    let cleanup_req = test::TestRequest::get().param("id", "-1").to_http_request();

    page_controllers::delete_page(cleanup_req.clone())
        .await
        .unwrap();
    module_controllers::delete_module(cleanup_req)
        .await
        .unwrap();
}

#[actix_rt::test]
async fn create_page() {
    create_test_page().await;

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::from_u16(201).unwrap());
}

#[actix_rt::test]
async fn read_all_pages() {
    create_test_page().await;

    let resp = page_controllers::get_pages().await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_page() {
    create_test_page().await;

    let req = test::TestRequest::get().param("id", "-1").to_http_request();
    let resp = page_controllers::get_page(req).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_page_join() {
    create_test_page().await;
    create_test_module().await;

    let req = test::TestRequest::get().param("id", "-1").to_http_request();
    let resp = page_controllers::get_page_join_modules(req).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_page() {
    create_test_page().await;

    let new_page = MutPage {
        page_name: String::from("Hello world!"),
        page_url: String::from("/"),
        page_title: String::from("Hello world!"),
    };

    let req = test::TestRequest::get().param("id", "-1").to_http_request();
    let resp = page_controllers::update_page(serde_json::to_string(&new_page).unwrap(), req)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn create_modules() {
    create_test_page().await;

    let new_module = MutModule {
        module_id: Some(-1),
        module_type_id: 1,
        content: Some(String::from("Hello world!")),
        title: String::from("title"),
        page_name: String::from("name"),
    };
    let resp = module_controllers::create_module(serde_json::to_string(&new_module).unwrap())
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::from_u16(201).unwrap());
}

#[actix_rt::test]
async fn read_all_modules() {
    create_test_page().await;
    create_test_module().await;

    let resp = module_controllers::get_modules().await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_module() {
    create_test_page().await;
    create_test_module().await;

    let req = test::TestRequest::get().param("id", "-1").to_http_request();
    let resp = module_controllers::get_module(req).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_modules() {
    create_test_page().await;
    create_test_module().await;

    let new_module = MutModule {
        module_id: Some(-1),
        module_type_id: 1,
        content: Some(String::from("Hello world!")),
        title: String::from("title"),
        page_name: String::from("name"),
    };

    let req = test::TestRequest::get().param("id", "-1").to_http_request();
    let resp = module_controllers::update_module(serde_json::to_string(&new_module).unwrap(), req)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}
