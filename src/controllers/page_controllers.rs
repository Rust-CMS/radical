use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse};
use handlebars::Handlebars;

use crate::models::{pool_handler, Joinable, Model, MySQLPool};

use crate::models::module_models::Module;
use crate::models::page_models::PageModuleRelation;
use crate::models::page_models::{MutPage, Page};

use crate::middleware::errors_middleware::{map_sql_error, CustomHttpError};
use crate::middleware::response_middleware::HttpResponseBuilder;

fn parse_page(page_vec: Vec<(Page, Module)>) -> Result<PageModuleRelation, CustomHttpError> {
    let origin_page = &page_vec.get(0).ok_or(CustomHttpError::NotFound)?.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res = PageModuleRelation {
        page_name: origin_page.page_name.to_string(),
        page_url: origin_page.page_url.to_string(),
        page_title: origin_page.page_title.to_string(),
        time_created: origin_page.time_created,
        fields: HashMap::new(),
        page_id: origin_page.id,
    };

    // Parsing of the tuples starts here.
    for tuple in page_vec {
        let module = tuple.1;
        res.fields.insert(module.title.clone(), module);
    }

    Ok(res)
}

pub async fn display_page(
    req: web::HttpRequest,
    pool: web::Data<MySQLPool>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let path = req.path();
    let page_vec = Page::read_one_join_on(path.to_string(), &mysql_pool).map_err(map_sql_error)?;
    // Parse it in to one single page.
    let pagemodule = parse_page(page_vec)?;

    hb.lock().unwrap().clear_templates();
    hb.lock().unwrap().register_templates_directory(".html", "./templates").unwrap();
    
    let s = hb.lock().unwrap().render(&pagemodule.page_name, &pagemodule).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

/// Creates a page by passing a page-like JSON object.
pub async fn create_page(
    req_body: String,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let new_page: MutPage = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;

    Page::create(&new_page, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(201, &new_page)
}

/// Gets all pages.
pub async fn get_pages(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let pages: Vec<Page> = Page::read_all(&mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &pages)
}

/// Gets one page by ID.
pub async fn get_page(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let page: Page = Page::read_one(*id, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &page)
}

/// This function also parses the read_one_join_on result out of a tuple.
/// A tuple generates a nasty response that isn't well defined.
/// This function parses it in to a Page that has all of the Modules as children.
pub async fn get_page_join_modules(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let page_id: &str = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?;

    let page_vec =
        Page::read_one_join_on(page_id.to_string(), &mysql_pool).map_err(map_sql_error)?;

    let pagemodules = parse_page(page_vec).or(Err(CustomHttpError::NotFound))?;

    HttpResponseBuilder::new(200, &pagemodules)
}

/// Updates a page by passing it a page-like JSON object and page ID.
pub async fn update_page(
    u_page: web::Json<MutPage>,
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Page::update(*id, &u_page, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &*u_page)
}

/// Deletes a page by passing an id.
pub async fn delete_page(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Page::delete(*id, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", id))
}
