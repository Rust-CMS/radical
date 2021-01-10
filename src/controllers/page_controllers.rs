use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse};

use crate::{models::{Joinable, Model}, page_models::PageModuleRelation};

use crate::page_models::{MutPage, Page};

use crate::errors_middleware::map_int_parsing_error;
use crate::errors_middleware::map_sql_error;
use crate::errors_middleware::CustomHttpError;

use crate::response_middleware::HttpResponseBuilder;

pub async fn index() -> Result<NamedFile, CustomHttpError> {
    actix_files::NamedFile::open("./public/index.html").or(Err(CustomHttpError::NotFound))
}

/// Creates a page by passing a page-like JSON object.
pub async fn create_page(req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let new_page: MutPage = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;

    Page::create(&new_page).map_err(map_sql_error)?;

    HttpResponseBuilder::new(201, &new_page)
}

/// Gets all pages.
pub async fn get_pages() -> Result<HttpResponse, CustomHttpError> {
    let pages: Vec<Page> = Page::read_all().map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &pages)
}

/// Gets one page by ID.
pub async fn get_page(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    let page: Page = Page::read_one(page_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &page)
}

/// This function also parses the read_one_join_on result out of a tuple.
/// A tuple generates a nasty response that isn't well defined.
/// This function parses it in to a Page that has all of the Modules as children.
pub async fn get_page_join_modules(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    let page = Page::read_one_join_on(page_id).map_err(map_sql_error)?;

    let origin_page = &page.get(0).ok_or(CustomHttpError::NotFound)?.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res = PageModuleRelation {
        page_id: origin_page.page_id,
        title: origin_page.title.to_owned(),
        time_created: origin_page.time_created,
        modules: Vec::new(),
    };

    // Parsing of the tuples starts here.
    for tuple in page {
        let module = tuple.1;
        res.modules.push(module);
    }

    HttpResponseBuilder::new(200, &res)
}

/// Updates a page by passing it a page-like JSON object and page ID.
pub async fn update_page(
    req_body: String,
    req: HttpRequest,
) -> Result<HttpResponse, CustomHttpError> {
    let u_page: MutPage =
        serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;
    let page_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    Page::update(page_id, &u_page).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &u_page)
}

/// Deletes a page by passing an id.
pub async fn delete_page(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    Page::delete(page_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", page_id))
}