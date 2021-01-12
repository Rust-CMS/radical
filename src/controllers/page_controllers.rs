use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse};
use module_models::Module;

use crate::{
    models::{pool_handler, Joinable, Model, MySQLPool},
    module_models,
    page_models::PageModuleRelation,
};

use askama::Template;

use crate::page_models::{MutPage, Page};

use crate::errors_middleware::map_sql_error;
use crate::errors_middleware::CustomHttpError;

use crate::response_middleware::HttpResponseBuilder;

fn parse_page(page_vec: Vec<(Page, Module)>) -> Result<PageModuleRelation, CustomHttpError> {
    let origin_page = &page_vec.get(0).ok_or(CustomHttpError::NotFound)?.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res = PageModuleRelation {
        url_path: origin_page.url_path.to_string(),
        title: origin_page.title.to_string(),
        time_created: origin_page.time_created,
        modules: Vec::new(),
    };

    // Parsing of the tuples starts here.
    for tuple in page_vec {
        let module = tuple.1;
        res.modules.push(module);
    }

    Ok(res)
}

pub async fn display_page(
    req: web::HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let path = req.path();
    // Select the page
    let page_vec = Page::read_one_join_on(path.to_string(), &mysql_pool).map_err(map_sql_error)?;
    
    // Parse it in to one single page.
    let pagemodule = parse_page(page_vec)?;

    let s = pagemodule.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(&s))
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
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let page_id: &str = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?;

    let page: Page = Page::read_one(page_id.to_string(), &mysql_pool).map_err(map_sql_error)?;

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

    HttpResponseBuilder::new(200, &page_vec)
}

/// Updates a page by passing it a page-like JSON object and page ID.
pub async fn update_page(
    req_body: String,
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let u_page: MutPage = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;
    let page_id: &str = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?;

    Page::update(page_id.to_string(), &u_page, &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &u_page)
}

/// Deletes a page by passing an id.
pub async fn delete_page(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let page_id: &str = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?;

    Page::delete(page_id.to_string(), &mysql_pool).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", page_id))
}
